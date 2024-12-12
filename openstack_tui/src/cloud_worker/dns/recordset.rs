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
use serde_json::Value;
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use openstack_sdk::{
    api::{paged, Pagination, QueryAsync},
    AsyncOpenStack,
};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::dns::types::DnsApiRequest;
use crate::cloud_worker::types::ApiRequest;
use crate::cloud_worker::types::ExecuteApiRequest;

/// Recordset API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsRecordsetApiRequest {
    /// List
    List(DnsRecordsetList),
}

impl From<DnsRecordsetApiRequest> for ApiRequest {
    fn from(item: DnsRecordsetApiRequest) -> Self {
        ApiRequest::Dns(DnsApiRequest::from(item))
    }
}

impl ExecuteApiRequest for DnsRecordsetApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            DnsRecordsetApiRequest::List(ref filters) => {
                let res: Vec<Value> = if filters.zone_id.is_some() {
                    let ep = TryInto::<
                        openstack_sdk::api::dns::v2::zone::recordset::list::Request<'_>,
                    >::try_into(filters)
                    .wrap_err("Cannot prepare request")?;
                    paged(ep, Pagination::All).query_async(session).await?
                } else {
                    let ep = TryInto::<openstack_sdk::api::dns::v2::recordset::list::Request<'_>>::try_into(
                            filters,
                        )
                        .wrap_err("Cannot prepare request")?;
                    paged(ep, Pagination::All).query_async(session).await?
                };
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: res,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsRecordsetList {
    /// Zone Id
    pub zone_id: Option<String>,
    /// Optional name (for info purposes)
    pub zone_name: Option<String>,
}

impl fmt::Display for DnsRecordsetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.zone_id.is_some() || self.zone_name.is_some() {
            parts.push(format!(
                "zone: {}",
                self.zone_name
                    .as_ref()
                    .or(self.zone_name.as_ref())
                    .unwrap_or(&String::new())
            ));
        }
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&DnsRecordsetList>
    for openstack_sdk::api::dns::v2::zone::recordset::list::Request<'_>
{
    type Error = openstack_sdk::api::dns::v2::zone::recordset::list::RequestBuilderError;

    fn try_from(value: &DnsRecordsetList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::builder();
        ep_builder.sort_key("name");
        ep_builder.sort_dir("asc");

        if let Some(zone_id) = &value.zone_id {
            ep_builder.zone_id(zone_id.clone());
        }

        ep_builder.build()
    }
}

impl TryFrom<&DnsRecordsetList> for openstack_sdk::api::dns::v2::recordset::list::Request<'_> {
    type Error = openstack_sdk::api::dns::v2::recordset::list::RequestBuilderError;

    fn try_from(_value: &DnsRecordsetList) -> Result<Self, Self::Error> {
        Self::builder().sort_key("name").sort_dir("asc").build()
    }
}
