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

//! List Routers command
//!
//! Wraps invoking of the `v2.0/routers` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::router::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Lists logical routers that the project who submits the request can access.
///
/// Default policy settings return only those routers that the project who
/// submits the request owns, unless an administrative user submits the
/// request.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List routers")]
pub struct RoutersCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// name query parameter for /v2.0/routers API
    ///
    #[arg(long)]
    name: Option<String>,

    /// admin_state_up query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// tenant_id query parameter for /v2.0/routers API
    ///
    #[arg(long)]
    tenant_id: Option<String>,

    /// enable_ndp_proxy query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    enable_ndp_proxy: Option<bool>,

    /// revision_number query parameter for /v2.0/routers API
    ///
    #[arg(long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    tags_any: Option<Vec<String>>,

    /// not-tags query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/routers API
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    not_tags_any: Option<Vec<String>>,

    /// description query parameter for /v2.0/routers API
    ///
    #[arg(long)]
    description: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Routers response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the router.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The router status.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    external_gateway_info: Option<Value>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ha: Option<BoolString>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be published to external by create `ndp_proxy`. `false` means NDP proxy
    /// is disabled, the IPv6 address of internal subnets attached to the
    /// router can not be published to external by `ndp_proxy`. It is available
    /// when `router-extend-ndp-proxy` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enable_ndp_proxy: Option<BoolString>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde()]
    #[structable(optional, wide)]
    flavor_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    availability_zones: Option<Value>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    availability_zone_hints: Option<Value>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    distributed: Option<BoolString>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    conntrack_helpers: Option<String>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, wide)]
    routes: Option<Value>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseExternalGatewayInfo {
    network_id: String,
    enable_snat: Option<bool>,
    external_fixed_ips: Option<Value>,
}

impl fmt::Display for ResponseExternalGatewayInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("network_id={}", self.network_id),
            format!(
                "enable_snat={}",
                self.enable_snat
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "external_fixed_ips={}",
                self.external_fixed_ips
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl RoutersCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Routers");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.query.enable_ndp_proxy {
            ep_builder.enable_ndp_proxy(*val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
