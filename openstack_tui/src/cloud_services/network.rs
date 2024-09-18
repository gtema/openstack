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

use openstack_sdk::{api::Pagination, api::QueryAsync};

use crate::action::{NetworkNetworkFilters, NetworkSubnetFilters};
use crate::cloud_services::NetworkExt;
use crate::cloud_worker::Cloud;

impl NetworkExt for Cloud {
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
}
