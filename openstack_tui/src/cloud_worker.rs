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

use eyre::Result;
use openstack_sdk::{config::ConfigFile, AsyncOpenStack};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tracing::debug;

use crate::action::{Action, Resource};
use crate::cloud_services::{ComputeExt, ImageExt, NetworkExt};

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
            .get_cloud_config(cloud)?
            .expect("Valid cloud");
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
                            "Failed to fetch compute flavors: {:?}",
                            err
                        )))?,
                    },
                    Action::ListClouds => {
                        app_tx.send(Action::Clouds(self.cloud_configs.get_available_clouds()))?;
                    }
                    Action::RequestCloudResource(resource) => match resource {
                        Resource::ComputeFlavors(ref _filters) => match self
                            .get_compute_flavors()
                            .await
                        {
                            Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Failed to fetch compute flavors: {:?}",
                                err
                            )))?,
                        },
                        Resource::ComputeServers(ref _filters) => match self
                            .get_compute_servers()
                            .await
                        {
                            Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Failed to fetch compute servers: {:?}",
                                err
                            )))?,
                        },
                        Resource::ComputeServerConsoleOutput(ref id) => {
                            match self.get_compute_server_console_output(id).await {
                                Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                                Err(err) => app_tx.send(Action::Error(format!(
                                    "Failed to fetch server console output: {:?}",
                                    err
                                )))?,
                            }
                        }
                        Resource::ComputeQuota => match self.get_compute_quota().await {
                            Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Failed to fetch compute quota: {:?}",
                                err
                            )))?,
                        },
                        Resource::NetworkNetworks(ref filters) => match self
                            .get_network_networks(filters)
                            .await
                        {
                            Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Failed to fetch networks: {:?}",
                                err
                            )))?,
                        },
                        Resource::NetworkSubnets(ref filters) => match self
                            .get_network_subnets(filters)
                            .await
                        {
                            Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                            Err(err) => app_tx.send(Action::Error(format!(
                                "Failed to fetch subnets: {:?}",
                                err
                            )))?,
                        },
                        Resource::ImageImages(ref filters) => {
                            match self.get_image_images(filters).await {
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
