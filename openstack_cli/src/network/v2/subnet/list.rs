//! List Subnets
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::VecString;
use crate::common::VecValue;
use openstack_sdk::api::network::v2::subnets::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// List Subnets
#[derive(Args, Clone, Debug)]
pub struct SubnetsArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// cidr filter parameter
    #[arg(long)]
    cidr: Option<String>,

    /// description filter parameter
    #[arg(long)]
    description: Option<String>,

    /// gateway_ip filter parameter
    #[arg(long)]
    gateway_ip: Option<String>,

    /// ip_version filter parameter
    #[arg(long)]
    ip_version: Option<u32>,

    /// ipv6_address_mode filter parameter
    #[arg(long)]
    ipv6_address_mode: Option<String>,

    /// ipv6_ra_mode filter parameter
    #[arg(long)]
    ipv6_ra_mode: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// network_id filter parameter
    #[arg(long)]
    network_id: Option<String>,

    /// segment_id filter parameter
    #[arg(long)]
    segment_id: Option<String>,

    /// dns_publish_fixed_ip filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    dns_publish_fixed_ip: Option<bool>,

    /// project_id filter parameter
    #[arg(long)]
    project_id: Option<String>,

    /// is_dhcp_enabled filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_dhcp_enabled: Option<bool>,

    /// subnet_pool_id filter parameter
    #[arg(long)]
    subnet_pool_id: Option<String>,

    /// use_default_subnet_pool filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    use_default_subnet_pool: Option<bool>,

    /// tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    any_tags: Option<Vec<String>>,

    /// not_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_tags: Option<Vec<String>>,

    /// not_any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_any_tags: Option<Vec<String>>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct SubnetsCmd {
    pub args: SubnetsArgs,
}

/// Subnets
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Subnets {
    /// List of allocation pools each of which has a start and an end address
    /// for this subnet
    #[structable(optional, wide)]
    allocation_pools: Option<VecValue>,

    /// The CIDR.
    #[structable(optional, wide)]
    cidr: Option<String>,

    /// Timestamp when the subnet was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The subnet description.
    #[structable(optional, wide)]
    description: Option<String>,

    /// A list of DNS nameservers.
    #[structable(optional, wide)]
    dns_nameservers: Option<VecString>,

    /// Whether to publish DNS records for fixed IPs
    #[structable(optional, wide)]
    dns_publish_fixed_ip: Option<bool>,

    /// Set to ``True`` if DHCP is enabled and ``False`` if DHCP is disabled.
    #[serde(rename = "enable_dhcp")]
    #[structable(optional, wide)]
    is_dhcp_enabled: Option<bool>,

    /// The gateway IP address.
    #[structable(optional, wide)]
    gateway_ip: Option<String>,

    /// A list of host routes.
    #[structable(optional, wide)]
    host_routes: Option<VecString>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The IP version, which is 4 or 6.
    #[structable(optional, wide)]
    ip_version: Option<u32>,

    /// The IPv6 address modes which are 'dhcpv6-stateful', 'dhcpv6-stateless'
    /// or 'slaac'.
    #[structable(optional, wide)]
    ipv6_address_mode: Option<String>,

    /// The IPv6 router advertisements modes which can be 'slaac',
    /// 'dhcpv6-stateful', 'dhcpv6-stateless'.
    #[structable(optional, wide)]
    ipv6_ra_mode: Option<String>,

    /// The subnet name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The prefix length to use for subnet allocation from a subnet pool
    #[serde(rename = "prefixlen")]
    #[structable(optional, wide)]
    prefix_length: Option<String>,

    /// The ID of the project this subnet is associated with.
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// None
    #[structable(optional, wide)]
    revision_number: Option<u32>,

    /// The ID of the segment this subnet is associated with.
    #[structable(optional, wide)]
    segment_id: Option<String>,

    /// Service types for this subnet
    #[structable(optional, wide)]
    service_types: Option<VecString>,

    /// The subnet pool ID from which to obtain a CIDR.
    #[serde(rename = "subnetpool_id")]
    #[structable(optional, wide)]
    subnet_pool_id: Option<String>,

    /// Subnet Tags.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Timestamp when the subnet was last updated.
    #[structable(optional)]
    updated_at: Option<String>,

    /// Whether to use the default subnet pool to obtain a CIDR.
    #[serde(rename = "use_default_subnetpool")]
    #[structable(optional, wide)]
    use_default_subnet_pool: Option<bool>,
}

#[async_trait]
impl Command for SubnetsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Subnets with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Subnets::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.cidr {
            ep_builder.cidr(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.gateway_ip {
            ep_builder.gateway_ip(val);
        }
        if let Some(val) = &self.args.ip_version {
            ep_builder.ip_version(*val);
        }
        if let Some(val) = &self.args.ipv6_address_mode {
            ep_builder.ipv6_address_mode(val);
        }
        if let Some(val) = &self.args.ipv6_ra_mode {
            ep_builder.ipv6_ra_mode(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.network_id {
            ep_builder.network_id(val);
        }
        if let Some(val) = &self.args.segment_id {
            ep_builder.segment_id(val);
        }
        if let Some(val) = &self.args.dns_publish_fixed_ip {
            ep_builder.dns_publish_fixed_ip(*val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.is_dhcp_enabled {
            ep_builder.is_dhcp_enabled(*val);
        }
        if let Some(val) = &self.args.subnet_pool_id {
            ep_builder.subnet_pool_id(val);
        }
        if let Some(val) = &self.args.use_default_subnet_pool {
            ep_builder.use_default_subnet_pool(*val);
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.args.any_tags {
            ep_builder.any_tags(val.iter());
        }
        if let Some(val) = &self.args.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.args.not_any_tags {
            ep_builder.not_any_tags(val.iter());
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Subnets>(data)?;
        Ok(())
    }
}
