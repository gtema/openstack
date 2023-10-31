//! Create Port
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
use openstack_sdk::api::network::v2::ports::post;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Create Port
#[derive(Args, Clone, Debug)]
pub struct PortArgs {
    /// The administrative state of the port, which is up ``True`` or down
    /// ``False``.
    #[arg(long)]
    is_admin_state_up: Option<bool>,

    /// Allowed address pairs list. Dictionary key ``ip_address`` is required
    /// and key ``mac_address`` is optional.
    #[arg(long, action=clap::ArgAction::Append)]
    allowed_address_pairs: Option<Vec<String>>,

    /// The ID of the host where the port is allocated. In some cases,
    /// different implementations can run on different hosts.
    #[arg(long)]
    binding_host_id: Option<String>,

    /// A dictionary the enables the application running on the specified host
    /// to pass and receive vif port-specific information to the plug-in.
    #[arg(long, value_parser=parse_json, value_name="JSON_VALUE")]
    binding_profile: Option<Value>,

    /// Read-only. A dictionary that enables the application to pass
    /// information about functions that the Networking API provides. To enable
    /// or disable port filtering features such as security group and anti-
    /// MAC/IP spoofing, specify ``port_filter: True`` or ``port_filter:
    /// False``.
    #[arg(long, value_parser=parse_json, value_name="JSON_VALUE")]
    binding_vif_details: Option<Value>,

    /// Read-only. The vif type for the specified port.
    #[arg(long)]
    binding_vif_type: Option<String>,

    /// The vnic type that is bound to the neutron port.  In POST and PUT
    /// operations, specify a value of ``normal`` (virtual nic), ``direct``
    /// (pci passthrough), or ``macvtap`` (virtual interface with a tap-like
    /// software interface). These values support SR-IOV PCI passthrough
    /// networking. The ML2 plug-in supports the vnic_type.  In GET operations,
    /// the binding:vnic_type extended attribute is visible to only port owners
    /// and administrative users.
    #[arg(long)]
    binding_vnic_type: Option<String>,

    /// Underlying data plane status of this port.
    #[arg(long)]
    data_plane_status: Option<String>,

    /// The port description.
    #[arg(long)]
    description: Option<String>,

    /// Device ID of this port.
    #[arg(long)]
    device_id: Option<String>,

    /// Device owner of this port (e.g. ``network:dhcp``).
    #[arg(long)]
    device_owner: Option<String>,

    /// None
    #[arg(long)]
    device_profile: Option<String>,

    /// DNS assignment for the port.
    #[arg(long)]
    dns_assignment: Option<String>,

    /// DNS domain assigned to the port.
    #[arg(long)]
    dns_domain: Option<String>,

    /// DNS name for the port.
    #[arg(long)]
    dns_name: Option<String>,

    /// Extra DHCP options.
    #[arg(long, action=clap::ArgAction::Append)]
    extra_dhcp_opts: Option<Vec<String>>,

    /// IP addresses for the port. Includes the IP address and subnet ID.
    #[arg(long, action=clap::ArgAction::Append, value_parser=parse_json, value_name="JSON_VALUE")]
    fixed_ips: Option<Vec<Value>>,

    /// None
    #[arg(long)]
    ip_allocation: Option<String>,

    /// The MAC address of an allowed address pair.
    #[arg(long)]
    mac_address: Option<String>,

    /// The port name.
    #[arg(long)]
    name: Option<String>,

    /// The ID of the attached network.
    #[arg(long)]
    network_id: Option<String>,

    /// The NUMA affinity policy defined for this port.
    #[arg(long)]
    numa_affinity_policy: Option<String>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.
    #[arg(long)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project who owns the network. Only administrative users
    /// can specify a project ID other than their own.
    #[arg(long)]
    project_id: Option<String>,

    /// Whether to propagate uplink status of the port.
    #[arg(long)]
    propagate_uplink_status: Option<bool>,

    /// None
    #[arg(long)]
    qos_network_policy_id: Option<String>,

    /// The ID of the QoS policy attached to the port.
    #[arg(long)]
    qos_policy_id: Option<String>,

    /// None
    #[arg(long, value_parser=parse_json, value_name="JSON_VALUE")]
    resource_request: Option<Value>,

    /// None
    #[arg(long)]
    revision_number: Option<u32>,

    /// The IDs of any attached security groups.
    #[arg(long, action=clap::ArgAction::Append)]
    security_group_ids: Option<Vec<String>>,

    /// Port Tags.
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// Tenant_id (deprecated attribute).
    #[arg(long)]
    tenant_id: Option<String>,
}

pub struct PortCmd {
    pub args: PortArgs,
}

/// Port
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Port {
    /// The administrative state of the port, which is up ``True`` or down
    /// ``False``.
    #[serde(rename = "admin_state_up")]
    #[structable(optional)]
    is_admin_state_up: Option<bool>,

    /// Allowed address pairs list. Dictionary key ``ip_address`` is required
    /// and key ``mac_address`` is optional.
    #[structable(optional)]
    allowed_address_pairs: Option<VecString>,

    /// The ID of the host where the port is allocated. In some cases,
    /// different implementations can run on different hosts.
    #[serde(rename = "binding:host_id")]
    #[structable(optional)]
    binding_host_id: Option<String>,

    /// A dictionary the enables the application running on the specified host
    /// to pass and receive vif port-specific information to the plug-in.
    #[serde(rename = "binding:profile")]
    #[structable(optional)]
    binding_profile: Option<Value>,

    /// Read-only. A dictionary that enables the application to pass
    /// information about functions that the Networking API provides. To enable
    /// or disable port filtering features such as security group and anti-
    /// MAC/IP spoofing, specify ``port_filter: True`` or ``port_filter:
    /// False``.
    #[serde(rename = "binding:vif_details")]
    #[structable(optional)]
    binding_vif_details: Option<Value>,

    /// Read-only. The vif type for the specified port.
    #[serde(rename = "binding:vif_type")]
    #[structable(optional)]
    binding_vif_type: Option<String>,

    /// The vnic type that is bound to the neutron port.  In POST and PUT
    /// operations, specify a value of ``normal`` (virtual nic), ``direct``
    /// (pci passthrough), or ``macvtap`` (virtual interface with a tap-like
    /// software interface). These values support SR-IOV PCI passthrough
    /// networking. The ML2 plug-in supports the vnic_type.  In GET operations,
    /// the binding:vnic_type extended attribute is visible to only port owners
    /// and administrative users.
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional)]
    binding_vnic_type: Option<String>,

    /// Timestamp when the port was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// Underlying data plane status of this port.
    #[structable(optional)]
    data_plane_status: Option<String>,

    /// The port description.
    #[structable(optional)]
    description: Option<String>,

    /// Device ID of this port.
    #[structable(optional)]
    device_id: Option<String>,

    /// Device owner of this port (e.g. ``network:dhcp``).
    #[structable(optional)]
    device_owner: Option<String>,

    /// None
    #[structable(optional)]
    device_profile: Option<String>,

    /// DNS assignment for the port.
    #[structable(optional)]
    dns_assignment: Option<String>,

    /// DNS domain assigned to the port.
    #[structable(optional)]
    dns_domain: Option<String>,

    /// DNS name for the port.
    #[structable(optional)]
    dns_name: Option<String>,

    /// Extra DHCP options.
    #[structable(optional)]
    extra_dhcp_opts: Option<VecString>,

    /// IP addresses for the port. Includes the IP address and subnet ID.
    #[structable(optional)]
    fixed_ips: Option<VecValue>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// None
    #[structable(optional)]
    ip_allocation: Option<String>,

    /// The MAC address of an allowed address pair.
    #[structable(optional)]
    mac_address: Option<String>,

    /// The port name.
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[structable(optional)]
    network_id: Option<String>,

    /// The NUMA affinity policy defined for this port.
    #[structable(optional)]
    numa_affinity_policy: Option<String>,

    /// The port security status, which is enabled ``True`` or disabled
    /// ``False``.
    #[serde(rename = "port_security_enabled")]
    #[structable(optional)]
    is_port_security_enabled: Option<bool>,

    /// The ID of the project who owns the network. Only administrative users
    /// can specify a project ID other than their own.
    #[structable(optional)]
    project_id: Option<String>,

    /// Whether to propagate uplink status of the port.
    #[structable(optional)]
    propagate_uplink_status: Option<bool>,

    /// None
    #[structable(optional)]
    qos_network_policy_id: Option<String>,

    /// The ID of the QoS policy attached to the port.
    #[structable(optional)]
    qos_policy_id: Option<String>,

    /// None
    #[structable(optional)]
    resource_request: Option<Value>,

    /// None
    #[structable(optional)]
    revision_number: Option<u32>,

    /// The IDs of any attached security groups.
    #[serde(rename = "security_groups")]
    #[structable(optional)]
    security_group_ids: Option<VecString>,

    /// The port status. Value is ``ACTIVE`` or ``DOWN``.
    #[structable(optional)]
    status: Option<String>,

    /// Port Tags.
    #[structable(optional)]
    tags: Option<VecString>,

    /// Tenant_id (deprecated attribute).
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Read-only. The trunk referring to this parent port and its subports.
    /// Present for trunk parent ports if ``trunk-details`` extension is
    /// loaded.
    #[structable(optional)]
    trunk_details: Option<Value>,

    /// Timestamp when the port was last updated.
    #[structable(optional)]
    updated_at: Option<String>,
}

#[async_trait]
impl Command for PortCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Port with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Port::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        if let Some(val) = &self.args.is_admin_state_up {
            ep_builder.is_admin_state_up(*val);
        }
        if let Some(val) = &self.args.allowed_address_pairs {
            ep_builder.allowed_address_pairs(val.iter().cloned());
        }
        if let Some(val) = &self.args.binding_host_id {
            ep_builder.binding_host_id(val);
        }
        if let Some(val) = &self.args.binding_profile {
            ep_builder.binding_profile(val.clone());
        }
        if let Some(val) = &self.args.binding_vif_details {
            ep_builder.binding_vif_details(val.clone());
        }
        if let Some(val) = &self.args.binding_vif_type {
            ep_builder.binding_vif_type(val);
        }
        if let Some(val) = &self.args.binding_vnic_type {
            ep_builder.binding_vnic_type(val);
        }
        if let Some(val) = &self.args.data_plane_status {
            ep_builder.data_plane_status(val);
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
        if let Some(val) = &self.args.device_profile {
            ep_builder.device_profile(val);
        }
        if let Some(val) = &self.args.dns_assignment {
            ep_builder.dns_assignment(val);
        }
        if let Some(val) = &self.args.dns_domain {
            ep_builder.dns_domain(val);
        }
        if let Some(val) = &self.args.dns_name {
            ep_builder.dns_name(val);
        }
        if let Some(val) = &self.args.extra_dhcp_opts {
            ep_builder.extra_dhcp_opts(val.iter().cloned());
        }
        if let Some(val) = &self.args.fixed_ips {
            ep_builder.fixed_ips(val.iter().cloned());
        }
        if let Some(val) = &self.args.ip_allocation {
            ep_builder.ip_allocation(val);
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
        if let Some(val) = &self.args.numa_affinity_policy {
            ep_builder.numa_affinity_policy(val);
        }
        if let Some(val) = &self.args.is_port_security_enabled {
            ep_builder.is_port_security_enabled(*val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.propagate_uplink_status {
            ep_builder.propagate_uplink_status(*val);
        }
        if let Some(val) = &self.args.qos_network_policy_id {
            ep_builder.qos_network_policy_id(val);
        }
        if let Some(val) = &self.args.qos_policy_id {
            ep_builder.qos_policy_id(val);
        }
        if let Some(val) = &self.args.resource_request {
            ep_builder.resource_request(val.clone());
        }
        if let Some(val) = &self.args.revision_number {
            ep_builder.revision_number(*val);
        }
        if let Some(val) = &self.args.security_group_ids {
            ep_builder.security_group_ids(val.iter().cloned());
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter().cloned());
        }
        if let Some(val) = &self.args.tenant_id {
            ep_builder.tenant_id(val);
        }
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = ep.query_async(client).await?;
        op.output_single::<Port>(data)?;
        Ok(())
    }
}
