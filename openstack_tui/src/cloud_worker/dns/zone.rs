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

use openstack_sdk::{
    api::{paged, Pagination, QueryAsync},
    AsyncOpenStack,
};

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::dns::types::DnsApiRequest;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};
use crate::cloud_worker::ConfirmableRequest;

/// Zone API operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsZoneApiRequest {
    /// Delete
    Delete(DnsZoneDelete),
    /// List Zones
    List(DnsZoneList),
}

impl From<DnsZoneApiRequest> for ApiRequest {
    fn from(item: DnsZoneApiRequest) -> Self {
        ApiRequest::Dns(DnsApiRequest::from(item))
    }
}

impl ConfirmableRequest for DnsZoneApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            DnsZoneApiRequest::Delete(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}

impl ExecuteApiRequest for DnsZoneApiRequest {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        match self {
            DnsZoneApiRequest::Delete(ref filters) => {
                let ep =
                    TryInto::<openstack_sdk::api::dns::v2::zone::delete::Request<'_>>::try_into(
                        filters,
                    )
                    .wrap_err("Cannot prepare request")?;
                openstack_sdk::api::ignore(ep).query_async(session).await?;
                app_tx.send(Action::Refresh)?;
            }
            DnsZoneApiRequest::List(ref filters) => {
                let ep: openstack_sdk::api::dns::v2::zone::list::Request<'_> =
                    filters.try_into().wrap_err("Cannot prepare request")?;
                app_tx.send(Action::ApiResponsesData {
                    request: request.clone(),
                    data: paged(ep, Pagination::All).query_async(session).await?,
                })?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsZoneList {}

impl fmt::Display for DnsZoneList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl TryFrom<&DnsZoneList> for openstack_sdk::api::dns::v2::zone::list::Request<'_> {
    type Error = openstack_sdk::api::dns::v2::zone::list::RequestBuilderError;

    fn try_from(_value: &DnsZoneList) -> Result<Self, Self::Error> {
        Self::builder().sort_key("name").sort_dir("asc").build()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DnsZoneDelete {
    pub zone_id: String,
    pub zone_name: Option<String>,
}

impl fmt::Display for DnsZoneDelete {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

impl ConfirmableRequest for DnsZoneDelete {
    fn get_confirm_message(&self) -> Option<String> {
        Some(format!(
            "Delete DNS Zone {} ?",
            self.zone_name.clone().unwrap_or(self.zone_id.clone())
        ))
    }
}

impl TryFrom<&DnsZoneDelete> for openstack_sdk::api::dns::v2::zone::delete::Request<'_> {
    type Error = openstack_sdk::api::dns::v2::zone::delete::RequestBuilderError;

    fn try_from(value: &DnsZoneDelete) -> Result<Self, Self::Error> {
        Self::builder().id(value.zone_id.clone()).build()
    }
}
