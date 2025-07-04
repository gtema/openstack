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

use chrono::TimeDelta;
use eyre::{Report, Result, eyre};
use openstack_sdk::{
    AsyncOpenStack, auth::AuthState, config::ConfigFile, types::identity::v3::AuthResponse,
};
use std::path::PathBuf;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tracing::debug;

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
}

impl Cloud {
    pub fn new(
        client_config_config_file: Option<PathBuf>,
        client_secure_config_file: Option<PathBuf>,
    ) -> Self {
        let cfg = ConfigFile::new_with_user_specified_configs(
            client_config_config_file.as_deref(),
            client_secure_config_file.as_deref(),
        )
        .expect("unable to load config files");

        Self {
            cloud_configs: cfg,
            cloud: None,
        }
    }

    pub async fn connect_to_cloud(&mut self, cloud: String) -> Result<()> {
        debug!("Connecting to cloud {}", cloud);
        let profile = self
            .cloud_configs
            .get_cloud_config(cloud.clone())?
            .ok_or_else(|| eyre!("Cloud `{}` is not present in configuration files", cloud))?;
        let mut session = AsyncOpenStack::new_interactive(&profile, false).await?;

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
