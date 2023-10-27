//! List Networks
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

use crate::common::VecString;
use openstack_sdk::api::network::v2::networks::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// List Networks
#[derive(Args, Clone, Debug)]
pub struct NetworksArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// description filter parameter
    #[arg(long)]
    description: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// status filter parameter
    #[arg(long)]
    status: Option<String>,

    /// project_id filter parameter
    #[arg(long)]
    project_id: Option<String>,

    /// ipv4_address_scope_id filter parameter
    #[arg(long)]
    ipv4_address_scope_id: Option<String>,

    /// ipv6_address_scope_id filter parameter
    #[arg(long)]
    ipv6_address_scope_id: Option<String>,

    /// is_admin_state_up filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_admin_state_up: Option<bool>,

    /// is_port_security_enabled filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_port_security_enabled: Option<bool>,

    /// is_router_external filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_router_external: Option<bool>,

    /// is_shared filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_shared: Option<bool>,

    /// provider_network_type filter parameter
    #[arg(long)]
    provider_network_type: Option<String>,

    /// provider_physical_network filter parameter
    #[arg(long)]
    provider_physical_network: Option<String>,

    /// provider_segmentation_id filter parameter
    #[arg(long)]
    provider_segmentation_id: Option<String>,

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

pub struct NetworksCmd {
    pub args: NetworksArgs,
}

/// Networks
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Networks {
    /// The administrative state of the network, which is up ``True`` or down
    /// ``False``.
    #[serde(rename = "admin_state_up")]
    #[structable(optional, wide)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the network.
    #[structable(optional, wide)]
    availability_zone_hints: Option<String>,

    /// Availability zones for the network.
    #[structable(optional, wide)]
    availability_zones: Option<String>,

    /// Timestamp when the network was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The network description.
    #[structable(optional, wide)]
    description: Option<String>,

    /// The DNS domain associated.
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the IPv4 address scope for the network.
    #[serde(rename = "ipv4_address_scope")]
    #[structable(optional, wide)]
    ipv4_address_scope_id: Option<String>,

    /// The ID of the IPv6 address scope for the network.
    #[serde(rename = "ipv6_address_scope")]
    #[structable(optional, wide)]
    ipv6_address_scope_id: Option<String>,

    /// Whether or not this is the default external network.
    #[structable(optional, wide)]
    is_default: Option<bool>,

    /// Read-only. The maximum transmission unit (MTU) of the network resource.
    #[structable(optional, wide)]
    mtu: Option<u32>,

    /// The network name.
    #[structable(optional)]
    name: Option<String>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.  Available for multiple provider extensions.
    #[serde(rename = "port_security_enabled")]
    #[structable(optional, wide)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project this network is associated with.
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The type of physical network that maps to this network resource. For
    /// example, ``flat``, ``vlan``, ``vxlan``, or ``gre``. Available for
    /// multiple provider extensions.
    #[serde(rename = "provider:network_type")]
    #[structable(optional, wide)]
    provider_network_type: Option<String>,

    /// The physical network where this network object is implemented.
    /// Available for multiple provider extensions.
    #[serde(rename = "provider:physical_network")]
    #[structable(optional, wide)]
    provider_physical_network: Option<String>,

    /// An isolated segment ID on the physical network. The provider network
    /// type defines the segmentation model. Available for multiple provider
    /// extensions.
    #[serde(rename = "provider:segmentation_id")]
    #[structable(optional, wide)]
    provider_segmentation_id: Option<String>,

    /// The ID of the QoS policy attached to the port.
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// None
    #[structable(optional, wide)]
    revision_number: Option<u32>,

    /// Whether or not the router is external.
    #[serde(rename = "router:external")]
    #[structable(optional, wide)]
    is_router_external: Option<bool>,

    /// A list of provider segment objects. Available for multiple provider
    /// extensions.
    #[structable(optional, wide)]
    segments: Option<String>,

    /// Indicates whether this network is shared across all tenants. By
    /// default, only administrative users can change this value.
    #[serde(rename = "shared")]
    #[structable(optional, wide)]
    is_shared: Option<bool>,

    /// The network status.
    #[structable(optional, wide)]
    status: Option<String>,

    /// The associated subnet IDs.
    #[serde(rename = "subnets")]
    #[structable(optional, wide)]
    subnet_ids: Option<VecString>,

    /// Network Tags.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Timestamp when the network was last updated.
    #[structable(optional)]
    updated_at: Option<String>,

    /// Indicates the VLAN transparency mode of the network
    #[serde(rename = "vlan_transparent")]
    #[structable(optional, wide)]
    is_vlan_transparent: Option<bool>,
}

#[async_trait]
impl Command for NetworksCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Networks with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Networks::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.ipv4_address_scope_id {
            ep_builder.ipv4_address_scope_id(val);
        }
        if let Some(val) = &self.args.ipv6_address_scope_id {
            ep_builder.ipv6_address_scope_id(val);
        }
        if let Some(val) = &self.args.is_admin_state_up {
            ep_builder.is_admin_state_up(*val);
        }
        if let Some(val) = &self.args.is_port_security_enabled {
            ep_builder.is_port_security_enabled(*val);
        }
        if let Some(val) = &self.args.is_router_external {
            ep_builder.is_router_external(*val);
        }
        if let Some(val) = &self.args.is_shared {
            ep_builder.is_shared(*val);
        }
        if let Some(val) = &self.args.provider_network_type {
            ep_builder.provider_network_type(val);
        }
        if let Some(val) = &self.args.provider_physical_network {
            ep_builder.provider_physical_network(val);
        }
        if let Some(val) = &self.args.provider_segmentation_id {
            ep_builder.provider_segmentation_id(val);
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

        op.output_list::<Networks>(data)?;
        Ok(())
    }
}
