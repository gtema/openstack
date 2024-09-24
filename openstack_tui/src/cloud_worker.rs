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
use openstack_sdk::{config::ConfigFile, types::identity::v3::AuthResponse, AsyncOpenStack};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tracing::debug;

use crate::action::Action;

mod compute;
mod identity;
mod image;
mod network;
pub mod types;

use crate::cloud_worker::{
    compute::ComputeExt, identity::IdentityExt, image::ImageExt, network::NetworkExt, types::*,
};

/// Cloud worker struct
pub(crate) struct Cloud {
    cloud_configs: ConfigFile,
    pub(crate) cloud: Option<AsyncOpenStack>,
    app_tx: Option<UnboundedSender<Action>>,
    should_quit: bool,
}

impl Cloud {
    pub fn new() -> Self {
        let cfg = ConfigFile::new().unwrap();
        Self {
            cloud_configs: cfg,
            cloud: None,
            app_tx: None,
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
            .discover_service_endpoint(&openstack_sdk::types::ServiceType::Image)
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

    fn register_app_tx(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.app_tx = Some(tx);
        Ok(())
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
                    Action::RequestCloudResource(resource) => match resource {
                        Resource::ComputeFlavors(ref filters) => {
                            match <Cloud as ComputeExt>::get_flavors(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch compute flavors: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::ComputeServers(ref filters) => {
                            match <Cloud as ComputeExt>::get_servers(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch compute servers: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::ComputeServerConsoleOutput(ref id) => {
                            match <Cloud as ComputeExt>::get_server_console_output(self, id).await {
                                Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch server console output: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::ComputeQuota => {
                            match <Cloud as ComputeExt>::get_quota(self).await {
                                Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch compute quota: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::NetworkNetworks(ref filters) => {
                            match <Cloud as NetworkExt>::get_networks(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch networks: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::NetworkSubnets(ref filters) => {
                            match <Cloud as NetworkExt>::get_subnets(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch subnets: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::IdentityAuthProjects(ref filters) => {
                            match <Cloud as IdentityExt>::get_auth_projects(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch available project scopes: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::IdentityProjects(ref filters) => {
                            match <Cloud as IdentityExt>::get_projects(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch available projects: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::ImageImages(ref filters) => {
                            match <Cloud as ImageExt>::get_images(self, filters).await {
                                Ok(data) => {
                                    app_tx.send(Action::ResourcesData { resource, data })?
                                }
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch images: {:?}",
                                    err
                                )))?,
                            }
                        }
                    },
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
