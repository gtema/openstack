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

use openstack_sdk::api::load_balancer::v2::pool::member::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct LoadBalancerPoolMemberList {
    #[builder(default)]
    pub address: Option<String>,
    #[builder(default)]
    pub admin_state_up: Option<bool>,
    #[builder(default)]
    pub backup: Option<bool>,
    #[builder(default)]
    pub created_at: Option<String>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub monitor_address: Option<String>,
    #[builder(default)]
    pub monitor_port: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub not_tags: Option<String>,
    #[builder(default)]
    pub not_tags_any: Option<String>,
    #[builder(default)]
    pub operating_status: Option<String>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    pub pool_id: String,
    #[builder(default)]
    pub pool_name: Option<String>,
    #[builder(default)]
    pub project_id: Option<String>,
    #[builder(default)]
    pub project_name: Option<String>,
    #[builder(default)]
    pub protocol_port: Option<i32>,
    #[builder(default)]
    pub provisioning_status: Option<String>,
    #[builder(default)]
    pub subnet_id: Option<String>,
    #[builder(default)]
    pub subnet_name: Option<String>,
    #[builder(default)]
    pub tags: Option<String>,
    #[builder(default)]
    pub tags_any: Option<String>,
    #[builder(default)]
    pub updated_at: Option<String>,
    #[builder(default)]
    pub weight: Option<i32>,
}

impl fmt::Display for LoadBalancerPoolMemberList {
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

        parts.push(format!(
            "pool: {}",
            self.pool_name.clone().unwrap_or(self.pool_id.clone())
        ));
        if self.project_id.is_some() || self.project_name.is_some() {
            parts.push(format!(
                "project: {}",
                self.project_name
                    .as_ref()
                    .or(self.project_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.subnet_id.is_some() || self.subnet_name.is_some() {
            parts.push(format!(
                "subnet: {}",
                self.subnet_name
                    .as_ref()
                    .or(self.subnet_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl From<&LoadBalancerPoolMemberList> for RequestBuilder<'_> {
    fn from(value: &LoadBalancerPoolMemberList) -> Self {
        let mut ep_builder = Self::default();
        ep_builder.pool_id(value.pool_id.clone());
        if let Some(val) = &value.address {
            ep_builder.address(val.clone());
        }
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.backup {
            ep_builder.backup(*val);
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
        }
        if let Some(val) = &value.monitor_address {
            ep_builder.monitor_address(val.clone());
        }
        if let Some(val) = &value.monitor_port {
            ep_builder.monitor_port(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }
        if let Some(val) = &value.project_id {
            ep_builder.project_id(val.clone());
        }
        if let Some(val) = &value.protocol_port {
            ep_builder.protocol_port(*val);
        }
        if let Some(val) = &value.subnet_id {
            ep_builder.subnet_id(val.clone());
        }
        if let Some(val) = &value.updated_at {
            ep_builder.updated_at(val.clone());
        }
        if let Some(val) = &value.weight {
            ep_builder.weight(*val);
        }
        if let Some(val) = &value.provisioning_status {
            ep_builder.provisioning_status(val.clone());
        }
        if let Some(val) = &value.operating_status {
            ep_builder.operating_status(val.clone());
        }
        if let Some(val) = &value.tags {
            ep_builder.tags(val.clone());
        }
        if let Some(val) = &value.tags_any {
            ep_builder.tags_any(val.clone());
        }
        if let Some(val) = &value.not_tags {
            ep_builder.not_tags(val.clone());
        }
        if let Some(val) = &value.not_tags_any {
            ep_builder.not_tags_any(val.clone());
        }
        ep_builder
    }
}

impl ExecuteApiRequest for LoadBalancerPoolMemberList {
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
