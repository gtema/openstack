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

//! Action Router command
//!
//! Wraps invoking of the `v2.0/routers/{id}/remove_external_gateways` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_json;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::router::remove_external_gateways;
use openstack_sdk::types::BoolString;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Request body
///
#[derive(Args)]
#[command(about = "Remove external gateways from router")]
pub struct RouterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    router: Router,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Router Body data
#[derive(Args, Clone)]
struct Router {
    /// The list of external gateways of the router.
    ///
    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    external_gateways: Option<Vec<Value>>,
}

/// Router response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The availability zone candidates for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    availability_zone_hints: Option<Value>,

    /// The availability zone(s) for the router. It is available when
    /// `router_availability_zone` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    availability_zones: Option<Value>,

    /// The associated conntrack helper resources for the roter. If the router
    /// has multiple conntrack helper resources, this field has multiple
    /// entries. Each entry consists of netfilter conntrack helper (`helper`),
    /// the network protocol (`protocol`), the network port (`port`).
    ///
    #[serde()]
    #[structable(optional)]
    conntrack_helpers: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// `true` indicates a distributed router. It is available when `dvr`
    /// extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    distributed: Option<BoolString>,

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

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with `network_id`,
    /// `enable_snat`, `external_fixed_ips`, `qos_policy_id`,
    /// `enable_default_route_ecmp` and `enable_default_route_bfd`. Otherwise,
    /// this would be `null`.
    ///
    #[serde()]
    #[structable(optional)]
    external_gateway_info: Option<ResponseExternalGatewayInfo>,

    /// The ID of the flavor associated with the router.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// `true` indicates a highly-available router. It is available when
    /// `l3-ha` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    ha: Option<BoolString>,

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

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The extra routes configuration for L3 router. A list of dictionaries
    /// with `destination` and `nexthop` parameters. It is available when
    /// `extraroute` extension is enabled.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    routes: Option<Value>,

    /// The router status.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseExternalGatewayInfo {
    enable_snat: Option<BoolString>,
    external_fixed_ips: Option<Value>,
    network_id: String,
    qos_policy_id: Option<String>,
}

impl fmt::Display for ResponseExternalGatewayInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "enable_snat={}",
                self.enable_snat
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!(
                "external_fixed_ips={}",
                self.external_fixed_ips
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
            format!("network_id={}", self.network_id),
            format!(
                "qos_policy_id={}",
                self.qos_policy_id
                    .clone()
                    .map_or(String::new(), |v| v.to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Router");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = remove_external_gateways::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.router data
        let args = &self.router;
        let mut router_builder = remove_external_gateways::RouterBuilder::default();
        if let Some(val) = &args.external_gateways {
            let external_gateways_builder: Vec<remove_external_gateways::ExternalGateways> = val
                .iter()
                .flat_map(|v| {
                    serde_json::from_value::<remove_external_gateways::ExternalGateways>(
                        v.to_owned(),
                    )
                })
                .collect::<Vec<remove_external_gateways::ExternalGateways>>();
            router_builder.external_gateways(external_gateways_builder);
        }

        ep_builder.router(router_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
