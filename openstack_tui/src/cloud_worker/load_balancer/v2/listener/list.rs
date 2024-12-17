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
use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};
use std::fmt;
use tokio::sync::mpsc::UnboundedSender;

use crate::action::Action;
use crate::cloud_worker::common::CloudWorkerError;
use crate::cloud_worker::types::{ApiRequest, ExecuteApiRequest};

use openstack_sdk::api::load_balancer::v2::listener::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerListenerList {
    admin_state_up: Option<bool>,
    alpn_protocols: Option<String>,
    connection_limit: Option<String>,
    created_at: Option<String>,
    default_pool_id: Option<String>,
    default_pool_name: Option<String>,
    description: Option<String>,
    hsts_include_subdomains: Option<bool>,
    hsts_max_age: Option<i32>,
    hsts_preload: Option<bool>,
    id: Option<String>,
    limit: Option<i32>,
    load_balancer_id: Option<String>,
    load_balancer_name: Option<String>,
    marker: Option<String>,
    name: Option<String>,
    not_tags: Option<String>,
    not_tags_any: Option<String>,
    operating_status: Option<String>,
    page_reverse: Option<bool>,
    project_id: Option<String>,
    project_name: Option<String>,
    protocol: Option<String>,
    protocol_port: Option<i32>,
    provisioning_status: Option<String>,
    tags: Option<String>,
    tags_any: Option<String>,
    timeout_client_data: Option<i32>,
    timeout_member_connect: Option<i32>,
    timeout_member_data: Option<i32>,
    timeout_tcp_inspect: Option<i32>,
    tls_ciphers: Option<String>,
    tls_versions: Option<String>,
    updated_at: Option<String>,
}

impl fmt::Display for LoadBalancerListenerList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut parts: Vec<String> = Vec::new();
        if self.default_pool_id.is_some() || self.default_pool_name.is_some() {
            parts.push(format!(
                "default_pool: {}",
                self.default_pool_name
                    .as_ref()
                    .or(self.default_pool_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.id.is_some() || self.name.is_some() {
            parts.push(format!(
                "name/id: {}",
                self.name
                    .as_ref()
                    .or(self.id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.load_balancer_id.is_some() || self.load_balancer_name.is_some() {
            parts.push(format!(
                "load_balancer: {}",
                self.load_balancer_name
                    .as_ref()
                    .or(self.load_balancer_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        if self.project_id.is_some() || self.project_name.is_some() {
            parts.push(format!(
                "project: {}",
                self.project_name
                    .as_ref()
                    .or(self.project_id.as_ref())
                    .unwrap_or(&String::default())
            ));
        }

        write!(f, "{}", parts.join(","))
    }
}

impl From<&LoadBalancerListenerList> for RequestBuilder<'_> {
    fn from(value: &LoadBalancerListenerList) -> Self {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.alpn_protocols {
            ep_builder.alpn_protocols(val.clone());
        }
        if let Some(val) = &value.connection_limit {
            ep_builder.connection_limit(val.clone());
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.default_pool_id {
            ep_builder.default_pool_id(val.clone());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.hsts_include_subdomains {
            ep_builder.hsts_include_subdomains(*val);
        }
        if let Some(val) = &value.hsts_max_age {
            ep_builder.hsts_max_age(*val);
        }
        if let Some(val) = &value.hsts_preload {
            ep_builder.hsts_preload(*val);
        }
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &value.load_balancer_id {
            ep_builder.load_balancer_id(val.clone());
        }
        if let Some(val) = &value.marker {
            ep_builder.marker(val.clone());
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
        if let Some(val) = &value.protocol {
            ep_builder.protocol(val.clone());
        }
        if let Some(val) = &value.protocol_port {
            ep_builder.protocol_port(*val);
        }
        if let Some(val) = &value.timeout_client_data {
            ep_builder.timeout_client_data(*val);
        }
        if let Some(val) = &value.timeout_member_connect {
            ep_builder.timeout_member_connect(*val);
        }
        if let Some(val) = &value.timeout_member_data {
            ep_builder.timeout_member_data(*val);
        }
        if let Some(val) = &value.timeout_tcp_inspect {
            ep_builder.timeout_tcp_inspect(*val);
        }
        if let Some(val) = &value.tls_ciphers {
            ep_builder.tls_ciphers(val.clone());
        }
        if let Some(val) = &value.tls_versions {
            ep_builder.tls_versions(val.clone());
        }
        if let Some(val) = &value.updated_at {
            ep_builder.updated_at(val.clone());
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

impl ExecuteApiRequest for LoadBalancerListenerList {
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
