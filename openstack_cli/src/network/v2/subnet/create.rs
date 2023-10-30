//! Create Subnet
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
use openstack_sdk::api::network::v2::subnets::post;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Create Subnet
#[derive(Args, Clone, Debug)]
pub struct SubnetArgs {
    /// List of allocation pools each of which has a start and an end address
    /// for this subnet
    #[arg(long, action=clap::ArgAction::Append, value_parser=parse_json, value_name="JSON_VALUE")]
    allocation_pools: Option<Vec<Value>>,

    /// The CIDR.
    #[arg(long)]
    cidr: Option<String>,

    /// The subnet description.
    #[arg(long)]
    description: Option<String>,

    /// A list of DNS nameservers.
    #[arg(long, action=clap::ArgAction::Append)]
    dns_nameservers: Option<Vec<String>>,

    /// Whether to publish DNS records for fixed IPs
    #[arg(long)]
    dns_publish_fixed_ip: Option<bool>,

    /// Set to ``True`` if DHCP is enabled and ``False`` if DHCP is disabled.
    #[arg(long)]
    is_dhcp_enabled: Option<bool>,

    /// The gateway IP address.
    #[arg(long)]
    gateway_ip: Option<String>,

    /// A list of host routes.
    #[arg(long, action=clap::ArgAction::Append)]
    host_routes: Option<Vec<String>>,

    /// The IP version, which is 4 or 6.
    #[arg(long)]
    ip_version: Option<u32>,

    /// The IPv6 address modes which are 'dhcpv6-stateful', 'dhcpv6-stateless'
    /// or 'slaac'.
    #[arg(long)]
    ipv6_address_mode: Option<String>,

    /// The IPv6 router advertisements modes which can be 'slaac',
    /// 'dhcpv6-stateful', 'dhcpv6-stateless'.
    #[arg(long)]
    ipv6_ra_mode: Option<String>,

    /// The subnet name.
    #[arg(long)]
    name: Option<String>,

    /// The ID of the attached network.
    #[arg(long)]
    network_id: Option<String>,

    /// The prefix length to use for subnet allocation from a subnet pool
    #[arg(long)]
    prefix_length: Option<String>,

    /// The ID of the project this subnet is associated with.
    #[arg(long)]
    project_id: Option<String>,

    /// None
    #[arg(long)]
    revision_number: Option<u32>,

    /// The ID of the segment this subnet is associated with.
    #[arg(long)]
    segment_id: Option<String>,

    /// Service types for this subnet
    #[arg(long, action=clap::ArgAction::Append)]
    service_types: Option<Vec<String>>,

    /// The subnet pool ID from which to obtain a CIDR.
    #[arg(long)]
    subnet_pool_id: Option<String>,

    /// Subnet Tags.
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// Tenant_id (deprecated attribute).
    #[arg(long)]
    tenant_id: Option<String>,

    /// Whether to use the default subnet pool to obtain a CIDR.
    #[arg(long)]
    use_default_subnet_pool: Option<bool>,
}

pub struct SubnetCmd {
    pub args: SubnetArgs,
}

/// Subnet
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Subnet {
    /// List of allocation pools each of which has a start and an end address
    /// for this subnet
    #[structable(optional)]
    allocation_pools: Option<VecValue>,

    /// The CIDR.
    #[structable(optional)]
    cidr: Option<String>,

    /// Timestamp when the subnet was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The subnet description.
    #[structable(optional)]
    description: Option<String>,

    /// A list of DNS nameservers.
    #[structable(optional)]
    dns_nameservers: Option<VecString>,

    /// Whether to publish DNS records for fixed IPs
    #[structable(optional)]
    dns_publish_fixed_ip: Option<bool>,

    /// Set to ``True`` if DHCP is enabled and ``False`` if DHCP is disabled.
    #[serde(rename = "enable_dhcp")]
    #[structable(optional)]
    is_dhcp_enabled: Option<bool>,

    /// The gateway IP address.
    #[structable(optional)]
    gateway_ip: Option<String>,

    /// A list of host routes.
    #[structable(optional)]
    host_routes: Option<VecString>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The IP version, which is 4 or 6.
    #[structable(optional)]
    ip_version: Option<u32>,

    /// The IPv6 address modes which are 'dhcpv6-stateful', 'dhcpv6-stateless'
    /// or 'slaac'.
    #[structable(optional)]
    ipv6_address_mode: Option<String>,

    /// The IPv6 router advertisements modes which can be 'slaac',
    /// 'dhcpv6-stateful', 'dhcpv6-stateless'.
    #[structable(optional)]
    ipv6_ra_mode: Option<String>,

    /// The subnet name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[structable(optional)]
    network_id: Option<String>,

    /// The prefix length to use for subnet allocation from a subnet pool
    #[serde(rename = "prefixlen")]
    #[structable(optional)]
    prefix_length: Option<String>,

    /// The ID of the project this subnet is associated with.
    #[structable(optional)]
    project_id: Option<String>,

    /// None
    #[structable(optional)]
    revision_number: Option<u32>,

    /// The ID of the segment this subnet is associated with.
    #[structable(optional)]
    segment_id: Option<String>,

    /// Service types for this subnet
    #[structable(optional)]
    service_types: Option<VecString>,

    /// The subnet pool ID from which to obtain a CIDR.
    #[serde(rename = "subnetpool_id")]
    #[structable(optional)]
    subnet_pool_id: Option<String>,

    /// Subnet Tags.
    #[structable(optional)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Timestamp when the subnet was last updated.
    #[structable(optional)]
    updated_at: Option<String>,

    /// Whether to use the default subnet pool to obtain a CIDR.
    #[serde(rename = "use_default_subnetpool")]
    #[structable(optional)]
    use_default_subnet_pool: Option<bool>,
}

#[async_trait]
impl Command for SubnetCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Subnet with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Subnet::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        if let Some(val) = &self.args.allocation_pools {
            ep_builder.allocation_pools(val.iter().cloned());
        }
        if let Some(val) = &self.args.cidr {
            ep_builder.cidr(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.dns_nameservers {
            ep_builder.dns_nameservers(val.iter().cloned());
        }
        if let Some(val) = &self.args.dns_publish_fixed_ip {
            ep_builder.dns_publish_fixed_ip(*val);
        }
        if let Some(val) = &self.args.is_dhcp_enabled {
            ep_builder.is_dhcp_enabled(*val);
        }
        if let Some(val) = &self.args.gateway_ip {
            ep_builder.gateway_ip(val);
        }
        if let Some(val) = &self.args.host_routes {
            ep_builder.host_routes(val.iter().cloned());
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
        if let Some(val) = &self.args.prefix_length {
            ep_builder.prefix_length(val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.revision_number {
            ep_builder.revision_number(*val);
        }
        if let Some(val) = &self.args.segment_id {
            ep_builder.segment_id(val);
        }
        if let Some(val) = &self.args.service_types {
            ep_builder.service_types(val.iter().cloned());
        }
        if let Some(val) = &self.args.subnet_pool_id {
            ep_builder.subnet_pool_id(val);
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter().cloned());
        }
        if let Some(val) = &self.args.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.args.use_default_subnet_pool {
            ep_builder.use_default_subnet_pool(*val);
        }
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = ep.query_async(client).await?;
        op.output_single::<Subnet>(data)?;
        Ok(())
    }
}
