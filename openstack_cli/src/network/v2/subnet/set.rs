//! Updates a subnet.
//!
//! Some attributes, such as IP version (ip\_version), CIDR (cidr), and
//! segment (segment\_id) cannot be updated. Attempting to update these
//! attributes results in a `400 Bad Request` error.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404, 412
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_json;
use crate::common::BoolString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::subnet::find;
use openstack_sdk::api::network::v2::subnet::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct SubnetArgs {
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
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// subnet_id parameter for /v2.0/subnets/{subnet_id} API
    #[arg()]
    id: String,
}
/// Subnet Body data
#[derive(Args, Debug, Clone)]
struct Subnet {
    /// Human-readable name of the resource.
    #[arg(long)]
    name: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet. If the gateway\_ip is not
    /// specified, OpenStack Networking allocates an address from the CIDR
    /// for the gateway for the subnet by default.
    #[arg(long)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet. If allocation\_pools are not specified, OpenStack
    /// Networking automatically allocates pools for covering all IP addresses
    /// in the CIDR, excluding the address reserved for the subnet gateway by
    /// default.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    allocation_pools: Option<Vec<Value>>,

    /// List of dns name servers associated with the subnet. Default is an
    /// empty list.
    #[arg(action=clap::ArgAction::Append, long)]
    dns_nameservers: Option<Vec<String>>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters. Default value is
    /// an empty list.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    host_routes: Option<Vec<Value>>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet. Default is `true`.
    #[arg(action=clap::ArgAction::Set, long)]
    enable_dhcp: Option<bool>,

    /// The service types associated with the subnet.
    #[arg(action=clap::ArgAction::Append, long)]
    service_types: Option<Vec<String>>,

    /// Whether to publish DNS records for IPs from this subnet. Default
    /// is `false`.
    #[arg(action=clap::ArgAction::Set, long)]
    dns_publish_fixed_ip: Option<bool>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[arg(long)]
    segment_id: Option<String>,
}

/// Subnet set command
pub struct SubnetCmd {
    pub args: SubnetArgs,
}
/// Subnet response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
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
    #[structable(optional)]
    ip_version: Option<i32>,

    /// The ID of the network to which the subnet belongs.
    #[serde()]
    #[structable(optional)]
    network_id: Option<String>,

    /// The ID of the subnet pool associated with the subnet.
    #[serde()]
    #[structable(optional)]
    subnetpool_id: Option<String>,

    /// The CIDR of the subnet.
    #[serde()]
    #[structable(optional)]
    cidr: Option<String>,

    /// Gateway IP of this subnet. If the value is `null` that implies no
    /// gateway is associated with the subnet.
    #[serde()]
    #[structable(optional)]
    gateway_ip: Option<String>,

    /// Allocation pools with `start` and `end` IP addresses
    /// for this subnet.
    #[serde()]
    #[structable(optional)]
    allocation_pools: Option<VecResponseAllocationPools>,

    /// List of dns name servers associated with the subnet.
    #[serde()]
    #[structable(optional)]
    dns_nameservers: Option<VecString>,

    /// Additional routes for the subnet. A list of dictionaries with
    /// `destination` and `nexthop` parameters.
    #[serde()]
    #[structable(optional)]
    host_routes: Option<VecResponseHostRoutes>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Indicates whether dhcp is enabled or disabled
    /// for the subnet.
    #[serde()]
    #[structable(optional)]
    enable_dhcp: Option<BoolString>,

    /// The IPv6 router advertisement specifies whether the networking service
    /// should transmit ICMPv6 packets, for a subnet. Value is `slaac`,
    /// `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional)]
    ipv6_ra_mode: Option<String>,

    /// The IPv6 address modes specifies mechanisms for assigning IP addresses.
    /// Value is `slaac`, `dhcpv6-stateful`, `dhcpv6-stateless` or `null`.
    #[serde()]
    #[structable(optional)]
    ipv6_address_mode: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The service types associated with the subnet.
    #[serde()]
    #[structable(optional)]
    service_types: Option<VecString>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    dns_publish_fixed_ip: Option<BoolString>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of a network segment the subnet is associated with.
    /// It is available when `segment` extension is enabled.
    #[serde()]
    #[structable(optional)]
    segment_id: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
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
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
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

#[async_trait]
impl OSCCommand for SubnetCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Subnet with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
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
        let args = &self.args.subnet;
        let mut subnet_builder = set::SubnetBuilder::default();
        if let Some(val) = &args.name {
            subnet_builder.name(val.clone());
        }

        if let Some(val) = &args.gateway_ip {
            subnet_builder.gateway_ip(val.clone());
        }

        if let Some(val) = &args.allocation_pools {
            let allocation_pools_builder: Vec<set::AllocationPools> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::AllocationPools>(v.clone()))
                .collect::<Vec<set::AllocationPools>>();
            subnet_builder.allocation_pools(allocation_pools_builder);
        }

        if let Some(val) = &args.dns_nameservers {
            subnet_builder.dns_nameservers(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.host_routes {
            let host_routes_builder: Vec<set::HostRoutes> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::HostRoutes>(v.clone()))
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
            subnet_builder.description(val.clone());
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
