//! List Ports
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
use openstack_sdk::api::network::v2::ports::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// List Ports
#[derive(Args, Clone, Debug)]
pub struct PortsArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// binding:host_id filter parameter
    #[arg(long)]
    binding_host_id: Option<String>,

    /// binding:profile filter parameter
    #[arg(long)]
    binding_profile: Option<String>,

    /// binding:vif_details filter parameter
    #[arg(long)]
    binding_vif_details: Option<String>,

    /// binding:vif_type filter parameter
    #[arg(long)]
    binding_vif_type: Option<String>,

    /// binding:vnic_type filter parameter
    #[arg(long)]
    binding_vnic_type: Option<String>,

    /// description filter parameter
    #[arg(long)]
    description: Option<String>,

    /// device_id filter parameter
    #[arg(long)]
    device_id: Option<String>,

    /// device_owner filter parameter
    #[arg(long)]
    device_owner: Option<String>,

    /// fields filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    fields: Option<Vec<String>>,

    /// fixed_ips filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    fixed_ips: Option<Vec<String>>,

    /// id filter parameter
    #[arg(long)]
    id: Option<String>,

    /// ip_address filter parameter
    #[arg(long)]
    ip_address: Option<String>,

    /// mac_address filter parameter
    #[arg(long)]
    mac_address: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// network_id filter parameter
    #[arg(long)]
    network_id: Option<String>,

    /// status filter parameter
    #[arg(long)]
    status: Option<String>,

    /// subnet_id filter parameter
    #[arg(long)]
    subnet_id: Option<String>,

    /// project_id filter parameter
    #[arg(long)]
    project_id: Option<String>,

    /// security_groups filter parameter
    #[arg(long)]
    security_groups: Option<String>,

    /// is_admin_state_up filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_admin_state_up: Option<bool>,

    /// is_port_security_enabled filter parameter
    #[arg(long, action=clap::ArgAction::Set)]
    is_port_security_enabled: Option<bool>,

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

pub struct PortsCmd {
    pub args: PortsArgs,
}

/// Ports
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Ports {
    /// The administrative state of the port, which is up ``True`` or down
    /// ``False``.
    #[serde(rename = "admin_state_up")]
    #[structable(optional, wide)]
    is_admin_state_up: Option<bool>,

    /// Allowed address pairs list. Dictionary key ``ip_address`` is required
    /// and key ``mac_address`` is optional.
    #[structable(optional, wide)]
    allowed_address_pairs: Option<VecString>,

    /// The ID of the host where the port is allocated. In some cases,
    /// different implementations can run on different hosts.
    #[serde(rename = "binding:host_id")]
    #[structable(optional, wide)]
    binding_host_id: Option<String>,

    /// A dictionary the enables the application running on the specified host
    /// to pass and receive vif port-specific information to the plug-in.
    #[serde(rename = "binding:profile")]
    #[structable(optional, wide)]
    binding_profile: Option<Value>,

    /// Read-only. A dictionary that enables the application to pass
    /// information about functions that the Networking API provides. To enable
    /// or disable port filtering features such as security group and anti-
    /// MAC/IP spoofing, specify ``port_filter: True`` or ``port_filter:
    /// False``.
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, wide)]
    binding_vif_details: Option<Value>,

    /// Read-only. The vif type for the specified port.
    #[serde(rename = "binding:vif_type")]
    #[structable(optional, wide)]
    binding_vif_type: Option<String>,

    /// The vnic type that is bound to the neutron port.  In POST and PUT
    /// operations, specify a value of ``normal`` (virtual nic), ``direct``
    /// (pci passthrough), or ``macvtap`` (virtual interface with a tap-like
    /// software interface). These values support SR-IOV PCI passthrough
    /// networking. The ML2 plug-in supports the vnic_type.  In GET operations,
    /// the binding:vnic_type extended attribute is visible to only port owners
    /// and administrative users.
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, wide)]
    binding_vnic_type: Option<String>,

    /// Timestamp when the port was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// Underlying data plane status of this port.
    #[structable(optional, wide)]
    data_plane_status: Option<String>,

    /// The port description.
    #[structable(optional, wide)]
    description: Option<String>,

    /// Device ID of this port.
    #[structable(optional, wide)]
    device_id: Option<String>,

    /// Device owner of this port (e.g. ``network:dhcp``).
    #[structable(optional, wide)]
    device_owner: Option<String>,

    /// None
    #[structable(optional, wide)]
    device_profile: Option<String>,

    /// DNS assignment for the port.
    #[structable(optional, wide)]
    dns_assignment: Option<String>,

    /// DNS domain assigned to the port.
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// DNS name for the port.
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// Extra DHCP options.
    #[structable(optional, wide)]
    extra_dhcp_opts: Option<VecString>,

    /// IP addresses for the port. Includes the IP address and subnet ID.
    #[structable(optional, wide)]
    fixed_ips: Option<VecValue>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// None
    #[structable(optional, wide)]
    ip_allocation: Option<String>,

    /// The MAC address of an allowed address pair.
    #[structable(optional, wide)]
    mac_address: Option<String>,

    /// The port name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The NUMA affinity policy defined for this port.
    #[structable(optional, wide)]
    numa_affinity_policy: Option<String>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.
    #[serde(rename = "port_security_enabled")]
    #[structable(optional, wide)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project who owns the network. Only administrative users
    /// can specify a project ID other than their own.
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// Whether to propagate uplink status of the port.
    #[structable(optional, wide)]
    propagate_uplink_status: Option<bool>,

    /// None
    #[structable(optional, wide)]
    qos_network_policy_id: Option<String>,

    /// The ID of the QoS policy attached to the port.
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// None
    #[structable(optional, wide)]
    resource_request: Option<Value>,

    /// None
    #[structable(optional, wide)]
    revision_number: Option<u32>,

    /// The IDs of any attached security groups.
    #[serde(rename = "security_groups")]
    #[structable(optional, wide)]
    security_group_ids: Option<VecString>,

    /// The port status. Value is ``ACTIVE`` or ``DOWN``.
    #[structable(optional, wide)]
    status: Option<String>,

    /// Port Tags.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Read-only. The trunk referring to this parent port and its subports.
    /// Present for trunk parent ports if ``trunk-details`` extension is
    /// loaded.
    #[structable(optional, wide)]
    trunk_details: Option<Value>,

    /// Timestamp when the port was last updated.
    #[structable(optional)]
    updated_at: Option<String>,
}

#[async_trait]
impl Command for PortsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Ports with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Ports::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.binding_host_id {
            ep_builder.binding_host_id(val);
        }
        if let Some(val) = &self.args.binding_profile {
            ep_builder.binding_profile(val);
        }
        if let Some(val) = &self.args.binding_vif_details {
            ep_builder.binding_vif_details(val);
        }
        if let Some(val) = &self.args.binding_vif_type {
            ep_builder.binding_vif_type(val);
        }
        if let Some(val) = &self.args.binding_vnic_type {
            ep_builder.binding_vnic_type(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.device_id {
            ep_builder.device_id(val);
        }
        if let Some(val) = &self.args.device_owner {
            ep_builder.device_owner(val);
        }
        if let Some(val) = &self.args.fields {
            ep_builder.fields(val.iter());
        }
        if let Some(val) = &self.args.fixed_ips {
            ep_builder.fixed_ips(val.iter());
        }
        if let Some(val) = &self.args.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.args.ip_address {
            ep_builder.ip_address(val);
        }
        if let Some(val) = &self.args.mac_address {
            ep_builder.mac_address(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.network_id {
            ep_builder.network_id(val);
        }
        if let Some(val) = &self.args.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.subnet_id {
            ep_builder.subnet_id(val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.security_groups {
            ep_builder.security_groups(val);
        }
        if let Some(val) = &self.args.is_admin_state_up {
            ep_builder.is_admin_state_up(*val);
        }
        if let Some(val) = &self.args.is_port_security_enabled {
            ep_builder.is_port_security_enabled(*val);
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

        op.output_list::<Ports>(data)?;
        Ok(())
    }
}
