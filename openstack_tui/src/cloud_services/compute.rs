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
use tracing::debug;

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::action::{ComputeFlavorFilters, ComputeServerFilters};
use crate::cloud_services::ComputeExt;
use crate::cloud_worker::Cloud;

impl ComputeExt for Cloud {
    async fn get_compute_flavors(&mut self, _filters: &ComputeFlavorFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::compute::v2::flavor::list_detailed::Request::builder();

            ep_builder.sort_key("name");

            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::All)
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_compute_servers(&mut self, filters: &ComputeServerFilters) -> Result<Vec<Value>> {
        if let Some(session) = &self.cloud {
            let mut ep_builder =
                openstack_sdk::api::compute::v2::server::list_detailed::Request::builder();

            ep_builder.sort_key("display_name");
            ep_builder.sort_dir("asc");

            if let Some(true) = &filters.all_tenants {
                ep_builder.all_tenants("true");
            }

            let ep = ep_builder.build()?;
            let res: Vec<Value> = openstack_sdk::api::paged(ep, Pagination::Limit(100))
                .query_async(session)
                .await?;
            return Ok(res);
        }
        Ok(Vec::new())
    }

    async fn get_compute_server_console_output(&mut self, id: &String) -> Result<Value> {
        if let Some(session) = &self.cloud {
            debug!("Fetching server console output for {:?}", id);
            let ep =
                openstack_sdk::api::compute::v2::server::os_get_console_output::Request::builder()
                    .id(id)
                    .os_get_console_output(openstack_sdk::api::compute::v2::server::os_get_console_output::OsGetConsoleOutputBuilder::default().build()?)
                    .build()?;

            let res: Value = ep.query_async(session).await?;
            return Ok(res.get("output").unwrap_or(&Value::Null).to_owned());
        }
        Ok(Value::Null)
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
}
