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

use eyre::{eyre, Report, Result};
use openstack_sdk::{
    config::ConfigFile, types::identity::v3::AuthResponse, types::ServiceType, AsyncOpenStack,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tracing::debug;

use crate::action::Action;

mod block_storage;
mod common;
mod compute;
mod dns;
mod identity;
mod image;
mod load_balancer;
mod network;
pub mod types;

use crate::cloud_worker::{
    block_storage::BlockStorageExt, compute::ComputeExt, dns::DnsExt, identity::IdentityExt,
    image::ImageExt, load_balancer::LoadBalancerExt, network::NetworkExt, types::*,
};

/// Cloud worker struct
pub(crate) struct Cloud {
    cloud_configs: ConfigFile,
    pub(crate) cloud: Option<AsyncOpenStack>,
    should_quit: bool,
}

impl Cloud {
    pub fn new() -> Self {
        let cfg = ConfigFile::new().unwrap();
        Self {
            cloud_configs: cfg,
            cloud: None,
            should_quit: false,
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
        if let Some(ref mut session) = self.cloud {
            debug!("Switching connection scope to {:?}", scope);
            session.authorize(Some(scope.clone()), true, false).await?;
            debug!("Authed as {:?}", session.get_auth_info());

            session
                .discover_service_endpoint(&openstack_sdk::types::ServiceType::Compute)
                .await?;
            session
                .discover_service_endpoint(&openstack_sdk::types::ServiceType::Image)
                .await?;

            session
                .discover_service_endpoint(&openstack_sdk::types::ServiceType::Network)
                .await?;

            Ok(session.get_auth_info())
        } else {
            Err(eyre!("Cannot change scope without being connected first"))
        }
    }

    pub async fn run(
        &mut self,
        app_tx: UnboundedSender<Action>,
        action_rx: &mut UnboundedReceiver<Action>,
    ) -> Result<()> {
        loop {
            while let Some(action) = action_rx.recv().await {
                debug!("Got action {:?}", action);
                match action {
                    Action::Quit => self.should_quit = true,
                    Action::ConnectToCloud(cloud) => match self.connect_to_cloud(cloud).await {
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
                        Err(err) => app_tx.send(Action::Error(format!(
                            "Failed to connect to the cloud: {:?}",
                            err
                        )))?,
                    },
                    Action::ListClouds => {
                        app_tx.send(Action::Clouds(self.cloud_configs.get_available_clouds()))?;
                    }
                    Action::CloudChangeScope(ref scope) => {
                        match self.switch_auth_scope(scope).await {
                            Ok(auth_response) => {
                                if let Some(auth_info) = auth_response {
                                    app_tx.send(Action::ConnectedToCloud(Box::new(
                                        auth_info.token,
                                    )))?;
                                }
                            }
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Cannot switch session scope: {:?}",
                                err
                            )))?,
                        }
                    }
                    Action::PerformApiRequest(request) => {
                        // Request the resource using the service extension trait
                        match ServiceType::from(request.clone()) {
                            ServiceType::BlockStorage => {
                                <Cloud as BlockStorageExt>::perform_api_request(
                                    self, &app_tx, request,
                                )
                                .await?
                            }
                            ServiceType::Compute => {
                                <Cloud as ComputeExt>::perform_api_request(self, &app_tx, request)
                                    .await?
                            }
                            ServiceType::Dns => {
                                <Cloud as DnsExt>::perform_api_request(self, &app_tx, request)
                                    .await?
                            }
                            ServiceType::Identity => {
                                <Cloud as IdentityExt>::perform_api_request(self, &app_tx, request)
                                    .await?
                            }
                            ServiceType::Image => {
                                <Cloud as ImageExt>::perform_api_request(self, &app_tx, request)
                                    .await?
                            }
                            ServiceType::LoadBalancer => {
                                <Cloud as LoadBalancerExt>::perform_api_request(
                                    self, &app_tx, request,
                                )
                                .await?
                            }
                            ServiceType::Network => {
                                <Cloud as NetworkExt>::perform_api_request(self, &app_tx, request)
                                    .await?
                            }
                            _ => todo!(),
                        }
                    }
                    _ => {}
                };
            }
            if self.should_quit {
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
        Ok(())
    }
}
