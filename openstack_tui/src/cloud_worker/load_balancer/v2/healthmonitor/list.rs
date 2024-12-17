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

use openstack_sdk::api::load_balancer::v2::healthmonitor::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadBalancerHealthmonitorList {
    _type: Option<String>,
    admin_state_up: Option<bool>,
    created_at: Option<String>,
    delay: Option<i32>,
    description: Option<String>,
    expected_codes: Option<String>,
    http_method: Option<String>,
    id: Option<String>,
    limit: Option<i32>,
    marker: Option<String>,
    max_retries: Option<i32>,
    max_retries_down: Option<i32>,
    name: Option<String>,
    not_tags: Option<String>,
    not_tags_any: Option<String>,
    operating_status: Option<String>,
    page_reverse: Option<bool>,
    pool_id: Option<String>,
    pool_name: Option<String>,
    project_id: Option<String>,
    project_name: Option<String>,
    provisioning_status: Option<String>,
    tags: Option<String>,
    tags_any: Option<String>,
    timeout: Option<i32>,
    updated_at: Option<String>,
    url_path: Option<String>,
}

impl fmt::Display for LoadBalancerHealthmonitorList {
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

        if self.pool_id.is_some() || self.pool_name.is_some() {
            parts.push(format!(
                "pool: {}",
                self.pool_name
                    .as_ref()
                    .or(self.pool_id.as_ref())
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

impl From<&LoadBalancerHealthmonitorList> for RequestBuilder<'_> {
    fn from(value: &LoadBalancerHealthmonitorList) -> Self {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.created_at {
            ep_builder.created_at(val.clone());
        }
        if let Some(val) = &value.delay {
            ep_builder.delay(*val);
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
        }
        if let Some(val) = &value.expected_codes {
            ep_builder.expected_codes(val.clone());
        }
        if let Some(val) = &value.http_method {
            ep_builder.http_method(val.clone());
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
        if let Some(val) = &value.max_retries {
            ep_builder.max_retries(*val);
        }
        if let Some(val) = &value.max_retries_down {
            ep_builder.max_retries_down(*val);
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.page_reverse {
            ep_builder.page_reverse(*val);
        }
        if let Some(val) = &value.pool_id {
            ep_builder.pool_id(val.clone());
        }
        if let Some(val) = &value.project_id {
            ep_builder.project_id(val.clone());
        }
        if let Some(val) = &value.timeout {
            ep_builder.timeout(*val);
        }
        if let Some(val) = &value._type {
            ep_builder._type(val.clone());
        }
        if let Some(val) = &value.updated_at {
            ep_builder.updated_at(val.clone());
        }
        if let Some(val) = &value.url_path {
            ep_builder.url_path(val.clone());
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

impl ExecuteApiRequest for LoadBalancerHealthmonitorList {
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
