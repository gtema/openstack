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

use openstack_sdk::api::network::v2::router::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkRouterList {
    admin_state_up: Option<bool>,
    description: Option<String>,
    enable_ndp_proxy: Option<bool>,
    id: Option<String>,
    limit: Option<i32>,
    marker: Option<String>,
    name: Option<String>,
    not_tags: Option<Vec<String>>,
    not_tags_any: Option<Vec<String>>,
    page_reverse: Option<bool>,
    revision_number: Option<String>,
    sort_dir: Option<Vec<String>>,
    sort_key: Option<Vec<String>>,
    tags: Option<Vec<String>>,
    tags_any: Option<Vec<String>>,
    tenant_id: Option<String>,
    tenant_name: Option<String>,
}

impl fmt::Display for NetworkRouterList {
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

impl From<&NetworkRouterList> for RequestBuilder<'_> {
    fn from(value: &NetworkRouterList) -> Self {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &value.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &value.enable_ndp_proxy {
            ep_builder.enable_ndp_proxy(*val);
        }
        if let Some(val) = &value.revision_number {
            ep_builder.revision_number(val.clone());
        }
        if let Some(val) = &value.tags {
            ep_builder.tags(val.iter().cloned());
        }
        if let Some(val) = &value.tags_any {
            ep_builder.tags_any(val.iter().cloned());
        }
        if let Some(val) = &value.not_tags {
            ep_builder.not_tags(val.iter().cloned());
        }
        if let Some(val) = &value.not_tags_any {
            ep_builder.not_tags_any(val.iter().cloned());
        }
        if let Some(val) = &value.description {
            ep_builder.description(val.clone());
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

impl ExecuteApiRequest for NetworkRouterList {
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
