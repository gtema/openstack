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
use serde_json::Value;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::action::Action;
use crate::cloud_worker::{Cloud, Resource};

pub mod types;
use types::*;

pub trait ComputeExt {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        resource: Resource,
    ) -> Result<()>;

    async fn get_flavors(&mut self, filters: &ComputeFlavorFilters) -> Result<Vec<Value>>;
    async fn get_servers(&mut self, filters: &ComputeServerFilters) -> Result<Vec<Value>>;
    async fn get_server_console_output<S: AsRef<str>>(&mut self, id: S) -> Result<Value>;
    async fn get_quota(&mut self) -> Result<Value>;
    async fn get_hypervisors(&mut self, filters: &ComputeHypervisorFilters) -> Result<Vec<Value>>;
    async fn get_aggregates(&mut self, filters: &ComputeAggregateFilters) -> Result<Vec<Value>>;
}

impl ComputeExt for Cloud {
    async fn perform_api_request(
        &mut self,
        app_tx: &UnboundedSender<Action>,
        resource: Resource,
    ) -> Result<()> {
        match resource {
            Resource::ComputeFlavors(ref filters) => match self.get_flavors(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch compute flavors: {:?}",
                    err
                )))?,
            },
            Resource::ComputeServers(ref filters) => match self.get_servers(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch compute servers: {:?}",
                    err
                )))?,
            },
            Resource::ComputeServerConsoleOutput(ref id) => {
                match self.get_server_console_output(id).await {
                    Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch server console output: {:?}",
                        err
                    )))?,
                }
            }
            Resource::ComputeQuota => match self.get_quota().await {
                Ok(data) => app_tx.send(Action::ResourceData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch compute quota: {:?}",
                    err
                )))?,
            },
            Resource::ComputeAggregates(ref filters) => match self.get_aggregates(filters).await {
                Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                Err(err) => app_tx.send(Action::Error(format!(
                    "Failed to fetch compute aggregates: {:?}",
                    err
                )))?,
            },
            Resource::ComputeHypervisors(ref filters) => {
                match self.get_hypervisors(filters).await {
                    Ok(data) => app_tx.send(Action::ResourcesData { resource, data })?,
                    Err(err) => app_tx.send(Action::Error(format!(
                        "Failed to fetch compute hypervisors: {:?}",
                        err
                    )))?,
                }
            }
            _ => {
                todo!()
            }
        }
        Ok(())
    }

    async fn get_flavors(&mut self, _filters: &ComputeFlavorFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::compute::v2::flavor::list_detailed::RequestBuilder::try_from(
                    _filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_servers(&mut self, filters: &ComputeServerFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::compute::v2::server::list_detailed::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_server_console_output<S: AsRef<str>>(&mut self, id: S) -> Result<Value> {
        if let Some(session) = &self.cloud {
            debug!("Fetching server console output for {:?}", id.as_ref());
            let ep =
                openstack_sdk::api::compute::v2::server::os_get_console_output::Request::builder()
                    .id(id.as_ref())
                    .os_get_console_output(openstack_sdk::api::compute::v2::server::os_get_console_output::OsGetConsoleOutputBuilder::default().build()?)
                    .build()?;

            let res: Value = ep.query_async(session).await?;
            return Ok(res.get("output").unwrap_or(&Value::Null).to_owned());
        }
        Ok(Value::Null)
    }

    async fn get_hypervisors(&mut self, filters: &ComputeHypervisorFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep =
                openstack_sdk::api::compute::v2::hypervisor::list_detailed::RequestBuilder::try_from(
                    filters,
                )?
                .build()?;

            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_aggregates(&mut self, filters: &ComputeAggregateFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let ep = openstack_sdk::api::compute::v2::aggregate::list::RequestBuilder::try_from(
                filters,
            )?
            .build()?;

            let res: Vec<Value> = ep.query_async(session).await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_quota(&mut self) -> Result<Value> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::compute::v2::quota_set::details::Request::builder();

            if let Some(auth) = session.get_auth_info() {
                if let Some(project) = auth.token.project {
                    if let Some(pid) = project.id {
                        ep_builder.id(pid);
                        let ep = ep_builder.build()?;
                        let res: Value = ep.query_async(session).await?;
                        return Ok(res);
                    }
                }
            }
        }
        Ok(Value::Null)
    }
}
