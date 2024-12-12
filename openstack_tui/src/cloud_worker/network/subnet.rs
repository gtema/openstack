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
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::network::types::NetworkApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

/// Subnet API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSubnetApiRequest {
    /// List
    List(NetworkSubnetList),
}

impl From<NetworkSubnetApiRequest> for ApiRequest {
    fn from(item: NetworkSubnetApiRequest) -> Self {
        ApiRequest::Network(NetworkApiRequest::from(item))
    }
}

impl ExecuteApiRequest for NetworkSubnetApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            NetworkSubnetApiRequest::List(ref req) => {
                let ep = TryInto::<
                    openstack_sdk::api::network::v2::subnet::list::Request<'_>,
                >::try_into(req)
                .wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: ep.query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkSubnetList {
    /// Network id
    pub network_id: Option<String>,
    /// Name of the parent network
    pub network_name: Option<String>,
}
impl fmt::Display for NetworkSubnetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.network_id.is_some() || self.network_name.is_some() {
            parts.push(format!(
                "network: {}",
                self.network_name
                    .as_ref()
                    .or(self.network_id.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&NetworkSubnetList> for openstack_sdk::api::network::v2::subnet::list::Request<'_> {
    type Error = openstack_sdk::api::network::v2::subnet::list::RequestBuilderError;

    fn try_from(value: &NetworkSubnetList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        ep_builder.sort_key(["name"].into_iter());
        ep_builder.sort_dir(["asc"].into_iter());
        if let Some(val) = &value.network_id {
            ep_builder.network_id(val.clone());
        }

        ep_builder.build()
    }
}
