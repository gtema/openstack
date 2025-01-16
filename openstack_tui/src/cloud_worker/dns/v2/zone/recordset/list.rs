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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
use derive_builder::Builder;
use eyre::{Report, Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use openstack_sdk::api::dns::v2::zone::recordset::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct DnsZoneRecordsetList {
    #[builder(default)]
    pub _type: Option<String>,
    #[builder(default)]
    pub data: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub market: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub sort_dir: Option<String>,
    #[builder(default)]
    pub sort_key: Option<String>,
    #[builder(default)]
    pub status: Option<String>,
    #[builder(default)]
    pub ttl: Option<i32>,
    pub zone_id: String,
    #[builder(default)]
    pub zone_name: Option<String>,
}

impl fmt::Display for DnsZoneRecordsetList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!(
            "zone: {}",
            self.zone_name.clone().unwrap_or(self.zone_id.clone())
        ));
        write!(f, "{}", parts.join(","))
    }
}

impl TryFrom<&DnsZoneRecordsetList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &DnsZoneRecordsetList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        ep_builder.zone_id(value.zone_id.clone());
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.market {
            ep_builder.market(val.clone());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.clone());
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value._type {
            ep_builder._type(val.clone());
        }
        if let Some(val) = &value.status {
            ep_builder.status(val.clone());
        }
        if let Some(val) = &value.ttl {
            ep_builder.ttl(*val);
        }
        if let Some(val) = &value.data {
            ep_builder.data(val.clone());
        }

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for DnsZoneRecordsetList {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = TryInto::<RequestBuilder>::try_into(self)?
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponsesData {
            request: request.clone(),
            data: paged(ep, Pagination::All).query_async(session).await?,
        })?;
        Ok(())
    }
}
