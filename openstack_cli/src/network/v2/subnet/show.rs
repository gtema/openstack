//! Get single Subnet
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::subnet::find;
use openstack_sdk::api::network::v2::subnet::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Get single Subnet
#[derive(Args, Clone, Debug)]
pub struct SubnetArgs {
    /// Subnet ID
    #[arg()]
    id: String,
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
        info!("Get Subnet with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = find::Subnet::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<Subnet>(data)?;
        Ok(())
    }
}
