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

/// Security Group Rule API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkSecurityGroupRuleApiRequest {
    /// List
    List(NetworkSecurityGroupRuleList),
}

impl From<NetworkSecurityGroupRuleApiRequest> for ApiRequest {
    fn from(item: NetworkSecurityGroupRuleApiRequest) -> Self {
        ApiRequest::Network(NetworkApiRequest::from(item))
    }
}

impl ExecuteApiRequest for NetworkSecurityGroupRuleApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            NetworkSecurityGroupRuleApiRequest::List(ref req) => {
                let ep = TryInto::<
                    openstack_sdk::api::network::v2::security_group_rule::list::Request<'_>,
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
pub struct NetworkSecurityGroupRuleList {
    pub security_group_id: Option<String>,
    pub security_group_name: Option<String>,
}

impl fmt::Display for NetworkSecurityGroupRuleList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.security_group_id.is_some() || self.security_group_name.is_some() {
            write!(
                f,
                "security_group: {}",
                self.security_group_name
                    .as_ref()
                    .or(self.security_group_id.as_ref())
                    .unwrap_or(&String::new())
            )?;
        }
        Ok(())
    }
}

impl TryFrom<&NetworkSecurityGroupRuleList>
    for openstack_sdk::api::network::v2::security_group_rule::list::Request<'_>
{
    type Error = openstack_sdk::api::network::v2::security_group_rule::list::RequestBuilderError;

    fn try_from(value: &NetworkSecurityGroupRuleList) -> Result<Self, Self::Error> {
        let mut ep_builder =
            openstack_sdk::api::network::v2::security_group_rule::list::Request::builder();

        ep_builder.sort_key(["ethertype", "direction", "protocol", "port_range_min"].into_iter());
        ep_builder.sort_dir(["asc", "asc", "asc", "asc"].into_iter());

        if let Some(security_group_id) = &value.security_group_id {
            ep_builder.security_group_id(security_group_id.clone());
        }

        if let Some(security_group_id) = &value.security_group_id {
            ep_builder.security_group_id(security_group_id.clone());
        }

        ep_builder.build()
    }
}
