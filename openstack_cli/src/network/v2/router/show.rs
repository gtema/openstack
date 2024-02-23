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

//! Show Router command
//!
//! Wraps invoking of the `v2.0/routers/{id}` with `GET` method

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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Shows details for a router.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For information, see
/// [Filtering and Column Selection](http://specs.openstack.org/openstack/neutron-specs/specs/api/networking_general_api_information.html#filtering-and-column-selection).
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403, 404
///
#[derive(Args)]
#[command(about = "Show router details")]
pub struct RouterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Router response representation
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
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The router status.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    #[serde()]
    #[structable(optional)]
    external_gateway_info: Option<ResponseExternalGatewayInfo>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    ha: Option<BoolString>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be published to external by create `ndp_proxy`. `false` means NDP proxy
    /// is disabled, the IPv6 address of internal subnets attached to the
    /// router can not be published to external by `ndp_proxy`. It is available
    /// when `router-extend-ndp-proxy` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    enable_ndp_proxy: Option<BoolString>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zones: Option<VecString>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone_hints: Option<VecString>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

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
    #[structable(optional)]
    distributed: Option<BoolString>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    #[serde()]
    #[structable(optional)]
    conntrack_helpers: Option<String>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    routes: Option<Value>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
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
/// Vector of `String` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Router");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
