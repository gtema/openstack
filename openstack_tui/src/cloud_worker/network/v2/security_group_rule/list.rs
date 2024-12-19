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
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use openstack_sdk::api::network::v2::security_group_rule::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkSecurityGroupRuleList {
    #[builder(default)]
    pub belongs_to_default_sg: Option<bool>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub direction: Option<String>,
    #[builder(default)]
    pub ethertype: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub normalized_cidr: Option<String>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    #[builder(default)]
    pub port_range_max: Option<i32>,
    #[builder(default)]
    pub port_range_min: Option<i32>,
    #[builder(default)]
    pub protocol: Option<String>,
    #[builder(default)]
    pub remote_address_group_id: Option<String>,
    #[builder(default)]
    pub remote_address_group_name: Option<String>,
    #[builder(default)]
    pub remote_group_id: Option<String>,
    #[builder(default)]
    pub remote_group_name: Option<String>,
    #[builder(default)]
    pub remote_ip_prefix: Option<String>,
    #[builder(default)]
    pub revision_number: Option<String>,
    #[builder(default)]
    pub security_group_id: Option<String>,
    #[builder(default)]
    pub security_group_name: Option<String>,
    #[builder(default)]
    pub sort_dir: Option<Vec<String>>,
    #[builder(default)]
    pub sort_key: Option<Vec<String>>,
    #[builder(default)]
    pub tenant_id: Option<String>,
    #[builder(default)]
    pub tenant_name: Option<String>,
}

impl fmt::Display for NetworkSecurityGroupRuleList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.id.is_some() || self.name.is_some() {
            parts.push(format!(
                "name/id: {}",
                self.name
                    .as_ref()
                    .or(self.id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.remote_address_group_id.is_some() || self.remote_address_group_name.is_some() {
            parts.push(format!(
                "remote_address_group: {}",
                self.remote_address_group_name
                    .as_ref()
                    .or(self.remote_address_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.remote_group_id.is_some() || self.remote_group_name.is_some() {
            parts.push(format!(
                "remote_group: {}",
                self.remote_group_name
                    .as_ref()
                    .or(self.remote_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.security_group_id.is_some() || self.security_group_name.is_some() {
            parts.push(format!(
                "security_group: {}",
                self.security_group_name
                    .as_ref()
                    .or(self.security_group_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.tenant_id.is_some() || self.tenant_name.is_some() {
            parts.push(format!(
                "tenant: {}",
                self.tenant_name
                    .as_ref()
                    .or(self.tenant_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl From<&NetworkSecurityGroupRuleList> for RequestBuilder<'_> {
    fn from(value: &NetworkSecurityGroupRuleList) -> Self {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.security_group_id {
            ep_builder.security_group_id(val.clone());
        }
        if let Some(val) = &value.remote_group_id {
            ep_builder.remote_group_id(val.clone());
        }
        if let Some(val) = &value.direction {
            ep_builder.direction(val.clone());
        }
        if let Some(val) = &value.protocol {
            ep_builder.protocol(val.clone());
        }
        if let Some(val) = &value.port_range_min {
            ep_builder.port_range_min(*val);
        }
        if let Some(val) = &value.port_range_max {
            ep_builder.port_range_max(*val);
        }
        if let Some(val) = &value.ethertype {
            ep_builder.ethertype(val.clone());
        }
        if let Some(val) = &value.remote_ip_prefix {
            ep_builder.remote_ip_prefix(val.clone());
        }
        if let Some(val) = &value.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &value.revision_number {
            ep_builder.revision_number(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.normalized_cidr {
            ep_builder.normalized_cidr(val.clone());
        }
        if let Some(val) = &value.remote_address_group_id {
            ep_builder.remote_address_group_id(val.clone());
        }
        if let Some(val) = &value.belongs_to_default_sg {
            ep_builder.belongs_to_default_sg(*val);
        }
        if let Some(val) = &value.sort_key {
            ep_builder.sort_key(val.iter().cloned());
        }
        if let Some(val) = &value.sort_dir {
            ep_builder.sort_dir(val.iter().cloned());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }
        ep_builder
    }
}

impl ExecuteApiRequest for NetworkSecurityGroupRuleList {
    async fn execute_request(
        &self,
        session: &mut AsyncOpenStack,
        request: &ApiRequest,
        app_tx: &UnboundedSender<Action>,
    ) -> Result<(), CloudWorkerError> {
        let ep = Into::<RequestBuilder>::into(self)
            .build()
            .wrap_err("Cannot prepare request")?;
        app_tx.send(Action::ApiResponsesData {
            request: request.clone(),
            data: paged(ep, Pagination::All).query_async(session).await?,
        })?;
        Ok(())
    }
}
