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

use crate::utils::OutputConfig;
use crate::utils::StructTable;
use openstack_sdk::api::network::v2::security_group::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::types::BoolString;
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkSecurityGroupList {
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub id: Option<String>,
    #[builder(default)]
    pub limit: Option<i32>,
    #[builder(default)]
    pub marker: Option<String>,
    #[builder(default)]
    pub name: Option<String>,
    #[builder(default)]
    pub not_tags: Option<Vec<String>>,
    #[builder(default)]
    pub not_tags_any: Option<Vec<String>>,
    #[builder(default)]
    pub page_reverse: Option<bool>,
    #[builder(default)]
    pub revision_number: Option<String>,
    #[builder(default)]
    pub shared: Option<bool>,
    #[builder(default)]
    pub sort_dir: Option<Vec<String>>,
    #[builder(default)]
    pub sort_key: Option<Vec<String>>,
    #[builder(default)]
    pub tags: Option<Vec<String>>,
    #[builder(default)]
    pub tags_any: Option<Vec<String>>,
    #[builder(default)]
    pub tenant_id: Option<String>,
    #[builder(default)]
    pub tenant_name: Option<String>,
}

impl fmt::Display for NetworkSecurityGroupList {
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

impl TryFrom<&NetworkSecurityGroupList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &NetworkSecurityGroupList) -> Result<Self, Self::Error> {
        let mut ep_builder = Self::default();
        if let Some(val) = &value.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &value.tenant_id {
            ep_builder.tenant_id(val.clone());
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
        if let Some(val) = &value.shared {
            ep_builder.shared(*val);
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

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for NetworkSecurityGroupList {
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
/// NetworkSecurityGroup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct NetworkSecurityGroup {
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    pub created_at: Option<String>,

    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// The ID of the security group.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    #[serde(default)]
    #[structable(optional, title = "REVISION_NUMBER", wide)]
    pub revision_number: Option<i32>,

    /// A list of `security_group_rule` objects. Refer to
    /// [Security group rules](#security-group-rules) for details.
    ///
    #[serde(default)]
    #[structable(optional, title = "SECURITY_GROUP_RULES", wide)]
    pub security_group_rules: Option<Value>,

    /// Indicates whether this security group is shared to the requester’s
    /// project.
    ///
    #[serde(default)]
    #[structable(optional, title = "SHARED", wide)]
    pub shared: Option<BoolString>,

    /// Indicates if the security group is stateful or stateless.
    ///
    #[serde(default)]
    #[structable(optional, title = "STATEFUL", wide)]
    pub stateful: Option<BoolString>,

    /// The list of tags on the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "TAGS", wide)]
    pub tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde(default)]
    #[structable(optional, title = "TENANT_ID", wide)]
    pub tenant_id: Option<String>,

    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,
}
