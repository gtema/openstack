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
use openstack_sdk::api::Pagination;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::config::ConfigFile;
use openstack_sdk::AsyncOpenStack;
use serde_json::Value;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tracing::debug;

use crate::action::{Action, ImageFilters, NetworkNetworkFilters, NetworkSubnetFilters, Resource};

pub(crate) struct Cloud {
    cloud_configs: ConfigFile,
    cloud: Option<AsyncOpenStack>,
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

    async fn get_compute_flavors(&mut self) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::compute::v2::flavor::list_detailed::Request::builder()
                .sort_key("name")
                .build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_compute_servers(&mut self) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::compute::v2::server::list_detailed::Request::builder()
                .sort_key("display_name")
                .sort_dir("asc")
                .build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::Limit(100))
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_compute_quota(&mut self) -> Result<Value> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::compute::v2::quota_set::details::Request::builder();

            ep_builder.id(self
                .cloud
                .as_ref()
                .expect("Connected")
                .get_auth_info()
                .expect("Authorized")
                .token
                .project
                .expect("Project scoped")
                .id
                .expect("ID is known"));
            let ep = ep_builder.build()?;
            let res: Value = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Value::Null)
    }

    async fn get_image_images(&mut self, filters: &ImageFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::image::v2::image::list::Request::builder();
            ep_builder.sort_key("name");
            ep_builder.sort_dir("asc");

            if let Some(vis) = &filters.visibility {
                ep_builder.visibility(vis);
            }
            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::Limit(100))
                .query_async(session)
                .await?;
            //let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_network_networks(
        &mut self,
        _filters: &NetworkNetworkFilters,
    ) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::network::v2::network::list::Request::builder();
            ep_builder.sort_key("name");
            ep_builder.sort_dir("asc");

            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::Limit(100))
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_network_subnets(&mut self, filters: &NetworkSubnetFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder = openstack_sdk::api::network::v2::subnet::list::Request::builder();
            ep_builder.sort_key("name");
            ep_builder.sort_dir("asc");

            if let Some(network_id) = &filters.network_id {
                ep_builder.network_id(network_id.clone());
            }
            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::Limit(100))
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
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
