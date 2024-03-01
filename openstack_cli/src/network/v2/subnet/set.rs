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

//! Set Subnet command
//!
//! Wraps invoking of the `v2.0/subnets/{subnet_id}` with `PUT` method

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

use crate::common::parse_json;
use crate::common::BoolString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::subnet::find;
use openstack_sdk::api::network::v2::subnet::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Updates a subnet.
///
/// Some attributes, such as IP version (ip_version), CIDR (cidr), and segment
/// (segment_id) cannot be updated. Attempting to update these attributes
/// results in a `400 Bad Request` error.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404, 412
///
#[derive(Args)]
#[command(about = "Update subnet")]
pub struct SubnetCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    subnet: Subnet,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// subnet_id parameter for /v2.0/subnets/{subnet_id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Subnet Body data
#[derive(Args)]
struct Subnet {
    /// Human-readable name of the resource.
    ///
    #[arg(long)]
    name: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet. If the gateway_ip is not
    /// specified, OpenStack Networking allocates an address from the CIDR for
    /// the gateway for the subnet by default.
    ///
    #[arg(long)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    /// If allocation_pools are not specified, OpenStack Networking
    /// automatically allocates pools for covering all IP addresses in the
    /// CIDR, excluding the address reserved for the subnet gateway by default.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    allocation_pools: Option<Vec<Value>>,

    /// List of dns name servers associated with the subnet. Default is an
    /// empty list.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    dns_nameservers: Option<Vec<String>>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters. Default value is an empty list.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    host_routes: Option<Vec<Value>>,

    /// Indicates whether dhcp is enabled or disabled for the subnet. Default
    /// is `true`.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    enable_dhcp: Option<bool>,

    /// The service types associated with the subnet.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    service_types: Option<Vec<String>>,

    /// Whether to publish DNS records for IPs from this subnet. Default is
    /// `false`.
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(long)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    ///
    #[arg(long)]
    segment_id: Option<String>,
}

/// Subnet response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    ///
    #[serde()]
    #[structable(optional)]
    ip_version: Option<i32>,

    /// The ID of the network to which the subnet belongs.
    ///
    #[serde()]
    #[structable(optional)]
    network_id: Option<String>,

    /// The ID of the subnet pool associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    subnetpool_id: Option<String>,

    /// The CIDR of the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses for this subnet.
    ///
    #[serde()]
    #[structable(optional)]
    allocation_pools: Option<Value>,

    /// List of dns name servers associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    dns_nameservers: Option<VecString>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    ///
    #[serde()]
    #[structable(optional)]
    host_routes: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled for the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    enable_dhcp: Option<BoolString>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde()]
    #[structable(optional)]
    ipv6_ra_mode: Option<String>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    ///
    #[serde()]
    #[structable(optional)]
    ipv6_address_mode: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The service types associated with the subnet.
    ///
    #[serde()]
    #[structable(optional)]
    service_types: Option<VecString>,

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

    /// Whether to publish DNS records for IPs from this subnet.
    ///
    #[serde()]
    #[structable(optional)]
    dns_publish_fixed_ip: Option<BoolString>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with. It is
    /// available when `segment` extension is enabled.
    ///
    #[serde()]
    #[structable(optional)]
    segment_id: Option<String>,
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

impl SubnetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Subnet");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.subnet data
        let args = &self.subnet;
        let mut subnet_builder = set::SubnetBuilder::default();
        if let Some(val) = &args.name {
            subnet_builder.name(val);
        }

        if let Some(val) = &args.gateway_ip {
            subnet_builder.gateway_ip(val);
        }

        if let Some(val) = &args.allocation_pools {
            let allocation_pools_builder: Vec<set::AllocationPools> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::AllocationPools>(v.to_owned()))
                .collect::<Vec<set::AllocationPools>>();
            subnet_builder.allocation_pools(allocation_pools_builder);
        }

        if let Some(val) = &args.dns_nameservers {
            subnet_builder.dns_nameservers(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.host_routes {
            let host_routes_builder: Vec<set::HostRoutes> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::HostRoutes>(v.to_owned()))
                .collect::<Vec<set::HostRoutes>>();
            subnet_builder.host_routes(host_routes_builder);
        }

        if let Some(val) = &args.enable_dhcp {
            subnet_builder.enable_dhcp(*val);
        }

        if let Some(val) = &args.service_types {
            subnet_builder.service_types(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.dns_publish_fixed_ip {
            subnet_builder.dns_publish_fixed_ip(*val);
        }

        if let Some(val) = &args.description {
            subnet_builder.description(val);
        }

        if let Some(val) = &args.segment_id {
            subnet_builder.segment_id(Some(val.into()));
        }

        ep_builder.subnet(subnet_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
