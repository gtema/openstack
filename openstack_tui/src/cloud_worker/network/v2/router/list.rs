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
use openstack_sdk::api::network::v2::router::list::RequestBuilder;
use openstack_sdk::api::{paged, Pagination};
use openstack_sdk::{api::QueryAsync, AsyncOpenStack};
use serde_json::Value;
use structable_derive::StructTable;

#[derive(Builder, Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[builder(setter(strip_option))]
pub struct NetworkRouterList {
    #[builder(default)]
    pub admin_state_up: Option<bool>,
    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub enable_ndp_proxy: Option<bool>,
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

impl TryFrom<&NetworkRouterList> for RequestBuilder<'_> {
    type Error = Report;
    fn try_from(value: &NetworkRouterList) -> Result<Self, Self::Error> {
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

        Ok(ep_builder)
    }
}

impl ExecuteApiRequest for NetworkRouterList {
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
/// NetworkRouter response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct NetworkRouter {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde(default)]
    #[structable(optional, title = "ADMIN_STATE_UP", wide)]
    pub admin_state_up: Option<bool>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "AVAILABILITY_ZONE_HINTS", wide)]
    pub availability_zone_hints: Option<Value>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "AVAILABILITY_ZONES", wide)]
    pub availability_zones: Option<Value>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    #[serde(default)]
    #[structable(optional, title = "CONNTRACK_HELPERS", wide)]
    pub conntrack_helpers: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "CREATED_AT")]
    pub created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "DESCRIPTION", wide)]
    pub description: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "DISTRIBUTED", wide)]
    pub distributed: Option<bool>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be published to external by create `ndp_proxy`. `false` means NDP proxy
    /// is disabled, the IPv6 address of internal subnets attached to the
    /// router can not be published to external by `ndp_proxy`. It is available
    /// when `router-extend-ndp-proxy` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "ENABLE_NDP_PROXY", wide)]
    pub enable_ndp_proxy: Option<bool>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    #[serde(default)]
    #[structable(optional, title = "EXTERNAL_GATEWAY_INFO", wide)]
    pub external_gateway_info: Option<Value>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde(default)]
    #[structable(optional, title = "FLAVOR_ID", wide)]
    pub flavor_id: Option<String>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "HA", wide)]
    pub ha: Option<bool>,

    /// The ID of the router.
    ///
    #[serde(default)]
    #[structable(optional, title = "ID", wide)]
    pub id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "NAME")]
    pub name: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde(default)]
    #[structable(optional, title = "REVISION_NUMBER", wide)]
    pub revision_number: Option<i32>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde(default)]
    #[structable(optional, title = "ROUTES", wide)]
    pub routes: Option<Value>,

    /// The router status.
    ///
    #[serde(default)]
    #[structable(optional, title = "STATUS")]
    pub status: Option<String>,

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

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde(default)]
    #[structable(optional, title = "UPDATED_AT")]
    pub updated_at: Option<String>,
}
