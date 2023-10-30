//! Get single Port
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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::port::find;
use openstack_sdk::api::network::v2::port::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Get single Port
#[derive(Args, Clone, Debug)]
pub struct PortArgs {
    /// Port ID
    #[arg()]
    id: String,
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
        info!("Get Port with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = find::Port::builder();
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
        op.output_single::<Port>(data)?;
        Ok(())
    }
}
