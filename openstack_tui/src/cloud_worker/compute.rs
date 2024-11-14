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
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use tracing::debug;

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::cloud_worker::Cloud;

pub trait ComputeExt {
    async fn get_flavors(&mut self, filters: &ComputeFlavorFilters) -> Result<Vec<Value>>;
    async fn get_servers(&mut self, filters: &ComputeServerFilters) -> Result<Vec<Value>>;
    async fn get_server_console_output<S: AsRef<str>>(&mut self, id: S) -> Result<Value>;
    async fn get_quota(&mut self) -> Result<Value>;
    async fn get_hypervisors(&mut self, filters: &ComputeHypervisorFilters) -> Result<Vec<Value>>;
    async fn get_aggregates(&mut self, filters: &ComputeAggregateFilters) -> Result<Vec<Value>>;
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeFlavorFilters {}

impl fmt::Display for ComputeFlavorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeFlavorFilters>
    for openstack_sdk::api::compute::v2::flavor::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeFlavorFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::flavor::list_detailed::Request::builder();

        ep_builder.sort_key("name");
        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeServerFilters {
    pub all_tenants: Option<bool>,
}

impl fmt::Display for ComputeServerFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&ComputeServerFilters>
    for openstack_sdk::api::compute::v2::server::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(value: &ComputeServerFilters) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::compute::v2::server::list_detailed::Request::builder();

        ep_builder.sort_key("display_name");
        ep_builder.sort_dir("asc");

        if let Some(true) = &value.all_tenants {
            ep_builder.all_tenants("true");
        }

        Ok(ep_builder)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeHypervisorFilters {}

impl fmt::Display for ComputeHypervisorFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl TryFrom<&ComputeHypervisorFilters>
    for openstack_sdk::api::compute::v2::hypervisor::list_detailed::RequestBuilder<'_>
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeHypervisorFilters) -> Result<Self, Self::Error> {
        Ok(openstack_sdk::api::compute::v2::hypervisor::list_detailed::Request::builder())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeAggregateFilters {}

impl fmt::Display for ComputeAggregateFilters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
impl TryFrom<&ComputeAggregateFilters>
    for openstack_sdk::api::compute::v2::aggregate::list::RequestBuilder
{
    type Error = eyre::Report;

    fn try_from(_value: &ComputeAggregateFilters) -> Result<Self, Self::Error> {
        Ok(openstack_sdk::api::compute::v2::aggregate::list::Request::builder())
    }
}

impl ComputeExt for Cloud {
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
}
