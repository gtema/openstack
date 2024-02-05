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

//! List Subnets command
//!
//! Wraps invoking of the `v2.0/subnets` with `GET` method

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

use openstack_sdk::api::network::v2::subnet::list;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Lists subnets that the project has access to.
///
/// Default policy settings return only subnets owned by the
/// project of the user submitting the request, unless the
/// user has administrative role. You can control which attributes
/// are returned by using the fields query parameter. You can filter
/// results by using query string parameters.
///
/// Use the `fields` query parameter to control which fields are
/// returned in the response body. Additionally, you can filter results
/// by using query string parameters. For information, see [Filtering
/// and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-
/// specification#Filtering_and_Column_Selection).
///
/// Normal response codes: 200
///
/// Error response codes: 401
#[derive(Args)]
#[command(about = "List subnets")]
pub struct SubnetsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {
    /// id query parameter for /v2.0/subnets API
    #[arg(long)]
    id: Option<String>,

    /// name query parameter for /v2.0/subnets API
    #[arg(long)]
    name: Option<String>,

    /// ip_version query parameter for /v2.0/subnets API
    #[arg(long)]
    ip_version: Option<String>,

    /// network_id query parameter for /v2.0/subnets API
    #[arg(long)]
    network_id: Option<String>,

    /// subnetpool_id query parameter for /v2.0/subnets API
    #[arg(long)]
    subnetpool_id: Option<String>,

    /// cidr query parameter for /v2.0/subnets API
    #[arg(long)]
    cidr: Option<String>,

    /// gateway_ip query parameter for /v2.0/subnets API
    #[arg(long)]
    gateway_ip: Option<String>,

    /// tenant_id query parameter for /v2.0/subnets API
    #[arg(long)]
    tenant_id: Option<String>,

    /// enable_dhcp query parameter for /v2.0/subnets API
    #[arg(long)]
    enable_dhcp: Option<bool>,

    /// ipv6_ra_mode query parameter for /v2.0/subnets API
    #[arg(long)]
    ipv6_ra_mode: Option<String>,

    /// ipv6_address_mode query parameter for /v2.0/subnets API
    #[arg(long)]
    ipv6_address_mode: Option<String>,

    /// shared query parameter for /v2.0/subnets API
    #[arg(long)]
    shared: Option<bool>,

    /// revision_number query parameter for /v2.0/subnets API
    #[arg(long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/subnets API
    #[arg(long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/subnets API
    #[arg(long)]
    tags_any: Option<Vec<String>>,

    /// not-tags query parameter for /v2.0/subnets API
    #[arg(long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/subnets API
    #[arg(long)]
    not_tags_any: Option<Vec<String>>,

    /// description query parameter for /v2.0/subnets API
    #[arg(long)]
    description: Option<String>,

    /// segment_id query parameter for /v2.0/subnets API
    #[arg(long)]
    segment_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {}
/// Subnets response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The ID of the subnet.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The IP protocol version. Value is `4` or `6`.
    #[serde()]
    #[structable(optional, wide)]
    ip_version: Option<i32>,

    /// The ID of the network to which the subnet belongs.
    #[serde()]
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The ID of the subnet pool associated with the subnet.
    #[serde()]
    #[structable(optional, wide)]
    subnetpool_id: Option<String>,

    /// The CIDR of the subnet.
    #[serde()]
    #[structable(optional, wide)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    #[serde()]
    #[structable(optional, wide)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet.
    #[serde()]
    #[structable(optional, wide)]
    allocation_pools: Option<VecResponseAllocationPools>,

    /// List of dns name servers associated with the subnet.
    #[serde()]
    #[structable(optional, wide)]
    dns_nameservers: Option<VecString>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    #[serde()]
    #[structable(optional, wide)]
    host_routes: Option<VecResponseHostRoutes>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet.
    #[serde()]
    #[structable(optional, wide)]
    enable_dhcp: Option<bool>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional, wide)]
    ipv6_ra_mode: Option<String>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional, wide)]
    ipv6_address_mode: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// The service types associated with the subnet.
    #[serde()]
    #[structable(optional, wide)]
    service_types: Option<VecString>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Whether to publish DNS records for IPs from this subnet.
    #[serde()]
    #[structable(optional, wide)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[serde()]
    #[structable(optional, wide)]
    segment_id: Option<String>,
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseAllocationPools {
    start: Option<String>,
    end: Option<String>,
}

impl fmt::Display for ResponseAllocationPools {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "start={}",
                self.start
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "end={}",
                self.end
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseAllocationPools response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseAllocationPools(Vec<ResponseAllocationPools>);
impl fmt::Display for VecResponseAllocationPools {
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
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecString(Vec<String>);
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
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseHostRoutes {
    destination: Option<String>,
    nexthop: Option<String>,
}

impl fmt::Display for ResponseHostRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "destination={}",
                self.destination
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "nexthop={}",
                self.nexthop
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseHostRoutes response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseHostRoutes(Vec<ResponseHostRoutes>);
impl fmt::Display for VecResponseHostRoutes {
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

impl SubnetsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Subnets");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val.clone());
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val.clone());
        }
        if let Some(val) = &self.query.ip_version {
            ep_builder.ip_version(val.clone());
        }
        if let Some(val) = &self.query.network_id {
            ep_builder.network_id(val.clone());
        }
        if let Some(val) = &self.query.subnetpool_id {
            ep_builder.subnetpool_id(val.clone());
        }
        if let Some(val) = &self.query.cidr {
            ep_builder.cidr(val.clone());
        }
        if let Some(val) = &self.query.gateway_ip {
            ep_builder.gateway_ip(val.clone());
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val.clone());
        }
        if let Some(val) = &self.query.enable_dhcp {
            ep_builder.enable_dhcp(*val);
        }
        if let Some(val) = &self.query.ipv6_ra_mode {
            ep_builder.ipv6_ra_mode(val.clone());
        }
        if let Some(val) = &self.query.ipv6_address_mode {
            ep_builder.ipv6_address_mode(val.clone());
        }
        if let Some(val) = &self.query.shared {
            ep_builder.shared(*val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val.clone());
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
            ep_builder.description(val.clone());
        }
        if let Some(val) = &self.query.segment_id {
            ep_builder.segment_id(val.clone());
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
