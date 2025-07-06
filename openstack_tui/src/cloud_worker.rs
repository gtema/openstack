// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//! Cloud worker module.
//!
//! Handle communication with the cloud including connection, re-connection (when auth expires) and
//! all the API requests.

use async_trait::async_trait;
use chrono::TimeDelta;
use eyre::{Report, Result, eyre};
use openstack_sdk::{
    AsyncOpenStack,
    auth::AuthState,
    auth::auth_helper::{AuthHelper, AuthHelperError},
    config::ConfigFile,
    types::identity::v3::AuthResponse,
};
use secrecy::SecretString;
use std::path::PathBuf;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use tracing::{debug, instrument, trace};

use crate::action::Action;

pub mod block_storage;
mod common;
pub mod compute;
pub mod dns;
pub mod identity;
pub mod image;
pub mod load_balancer;
pub mod network;
pub mod types;

pub use crate::cloud_worker::common::CloudWorkerError;

use crate::cloud_worker::types::*;
use crate::error::TuiError;

/// Cloud worker struct
pub(crate) struct Cloud {
    cloud_configs: ConfigFile,
    pub(crate) cloud: Option<AsyncOpenStack>,
    auth_helper: TuiAuthHelper,
}

#[derive(Debug)]
pub(crate) enum AuthAction {
    Data(String),
    Secret(SecretString),
    Cancel,
}

impl Cloud {
    pub fn new(
        client_config_config_file: Option<PathBuf>,
        client_secure_config_file: Option<PathBuf>,
        auth_helper_control_tx: mpsc::Sender<oneshot::Sender<AuthAction>>,
    ) -> Self {
        let cfg = ConfigFile::new_with_user_specified_configs(
            client_config_config_file.as_deref(),
            client_secure_config_file.as_deref(),
        )
        .expect("unable to load config files");

        Self {
            cloud_configs: cfg,
            cloud: None,
            auth_helper: TuiAuthHelper::new(auth_helper_control_tx),
        }
    }

    pub async fn connect_to_cloud(&mut self, cloud: String) -> Result<()> {
        debug!("Connecting to cloud {}", cloud);
        let profile = self
            .cloud_configs
            .get_cloud_config(cloud.clone())?
            .ok_or_else(|| eyre!("Cloud `{}` is not present in configuration files", cloud))?;
        self.auth_helper.set_cloud_name(Some(cloud));
        let mut session =
            AsyncOpenStack::new_with_authentication_helper(&profile, &mut self.auth_helper, false)
                .await?;

        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::Compute)
            .await?;
        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::BlockStorage)
            .await?;
        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::Dns)
            .await?;
        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::Image)
            .await?;
        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::LoadBalancer)
            .await?;
        session
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::Network)
            .await?;

        self.cloud = Some(session);

        Ok(())
    }

    pub async fn switch_auth_scope(
        &mut self,
        scope: &openstack_sdk::auth::authtoken::AuthTokenScope,
    ) -> Result<Option<AuthResponse>, Report> {
        match self.cloud {
            Some(ref mut session) => {
                debug!("Switching connection scope to {:?}", scope);
                session.authorize(Some(scope.clone()), true, false).await?;
                debug!("Authed as {:?}", session.get_auth_info());

                session
                    .discover_service_endpoint(&openstack_sdk::types::ServiceType::Compute)
                    .await?;
                session
                    .discover_service_endpoint(&openstack_sdk::types::ServiceType::BlockStorage)
                    .await?;
                session
                    .discover_service_endpoint(&openstack_sdk::types::ServiceType::Image)
                    .await?;

                session
                    .discover_service_endpoint(&openstack_sdk::types::ServiceType::Network)
                    .await?;

                Ok(session.get_auth_info())
            }
            _ => Err(eyre!("Cannot change scope without being connected first")),
        }
    }

    pub async fn run(
        &mut self,
        app_tx: UnboundedSender<Action>,
        action_rx: &mut UnboundedReceiver<Action>,
    ) -> Result<(), TuiError> {
        self.auth_helper.set_action_tx(app_tx.clone())?;
        while let Some(action) = action_rx.recv().await {
            debug!("Got action {:?}", action);
            match action {
                ref ac @ Action::ConnectToCloud(ref cloud) => {
                    match self.connect_to_cloud(cloud.clone()).await {
                        Ok(()) => {
                            if let Some(auth_info) = self
                                .cloud
                                .as_ref()
                                .expect("Connected to the cloud")
                                .get_auth_info()
                            {
                                app_tx.send(Action::ConnectedToCloud(Box::new(auth_info.token)))?;
                            }
                        }
                        Err(err) => app_tx.send(Action::Error {
                            msg: format!("Failed to connect to the cloud: {err:?}"),
                            action: Some(Box::new(ac.clone())),
                        })?,
                    }
                }
                Action::ListClouds => {
                    app_tx.send(Action::Clouds(self.cloud_configs.get_available_clouds()))?;
                }
                Action::CloudChangeScope(ref scope) => match self.switch_auth_scope(scope).await {
                    Ok(auth_response) => {
                        if let Some(auth_info) = auth_response {
                            app_tx.send(Action::ConnectedToCloud(Box::new(auth_info.token)))?;
                        }
                    }
                    Err(err) => app_tx.send(Action::Error {
                        msg: format!("Cannot switch session scope: {err:?}"),
                        action: Some(Box::new(action.clone())),
                    })?,
                },
                ref ac @ Action::PerformApiRequest(ref request) => {
                    if let Some(ref mut conn) = self.cloud {
                        // Check if reauth is necessary
                        match &conn.get_auth_state(Some(TimeDelta::seconds(10))) {
                            Some(AuthState::Expired) | Some(AuthState::AboutToExpire) => {
                                conn.authorize(None, false, true).await?;
                            }
                            _ => {}
                        }

                        request
                            .execute_request(conn, request, &app_tx)
                            .await
                            .or_else(|err| {
                                app_tx.send(Action::Error {
                                    msg: format!("Error performing API request\n\n{err:?}"),
                                    action: Some(Box::new(ac.clone())),
                                })
                            })?;
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }
}

struct TuiAuthHelper {
    cloud_name: Option<String>,
    app_tx: Option<UnboundedSender<Action>>,
    auth_helper_control_tx: mpsc::Sender<oneshot::Sender<AuthAction>>,
}

impl TuiAuthHelper {
    pub fn new(auth_helper_control_tx: mpsc::Sender<oneshot::Sender<AuthAction>>) -> Self {
        Self {
            cloud_name: None,
            app_tx: None,
            auth_helper_control_tx,
        }
    }

    pub fn set_action_tx(&mut self, app_tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.app_tx = Some(app_tx);
        Ok(())
    }

    #[instrument(skip(self))]
    async fn initiate(
        &mut self,
        prompt: String,
        connection_name: Option<String>,
        is_sensitive: bool,
    ) -> Result<oneshot::Receiver<AuthAction>, TuiError> {
        let (sender, receiver) = oneshot::channel();
        self.auth_helper_control_tx.send(sender).await.unwrap();
        if let Some(app_tx) = &self.app_tx {
            trace!("Sending request to the app");
            app_tx.send(Action::AuthDataRequired {
                prompt,
                connection_name,
                is_sensitive,
            })?;
        } else {
            return Err(eyre!(
                "Channel between cloud worker and application is missing".to_string(),
            )
            .into());
        }
        Ok(receiver)
    }
}

#[async_trait]
impl AuthHelper for TuiAuthHelper {
    #[instrument(skip(self))]
    async fn get(
        &mut self,
        prompt: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        let receiver = self
            .initiate(prompt, connection_name, true)
            .await
            .map_err(|e| AuthHelperError::Other(e.to_string()))?;
        trace!("Waiting for the auth data to arrive from the UI");
        match receiver.await {
            Ok(AuthAction::Data(dt)) => {
                trace!("auth data received");
                return Ok(dt);
            }
            _ => {
                trace!("auth data request cancelled");
                return Err(AuthHelperError::Other(
                    "error receiving the requested data".to_string(),
                ));
            }
        }
    }

    #[instrument(skip(self))]
    async fn get_secret(
        &mut self,
        prompt: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        let receiver = self
            .initiate(prompt, connection_name, true)
            .await
            .map_err(|e| AuthHelperError::Other(e.to_string()))?;
        trace!("Waiting for the auth data to arrive from the UI");
        match receiver.await {
            Ok(AuthAction::Secret(dt)) => {
                trace!("auth data received");
                Ok(dt)
            }
            _ => {
                trace!("auth data request cancelled");
                return Err(AuthHelperError::Other(
                    "error receiving the requested data".to_string(),
                ));
            }
        }
    }

    #[instrument(skip(self))]
    fn set_cloud_name(&mut self, cloud_name: Option<String>) {
        trace!("Setting cloud name to {:?}", cloud_name);
        self.cloud_name = cloud_name;
    }

    #[instrument(skip(self))]
    fn get_cloud_name(&self) -> Option<String> {
        self.cloud_name.clone()
    }
}

impl ExecuteApiRequest for ApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            ApiRequest::BlockStorage(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::Compute(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::Dns(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::Identity(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::Image(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::LoadBalancer(data) => data.execute_request(session, request, app_tx).await,
            ApiRequest::Network(data) => data.execute_request(session, request, app_tx).await,
        }
    }
}
