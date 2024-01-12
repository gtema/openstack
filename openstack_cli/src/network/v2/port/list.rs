//! Lists ports to which the user has access.
//!
//! Default policy settings return only those ports that are owned by
//! the project of the user who submits the request, unless the request is
//! submitted
//! by a user with administrative rights.
//!
//! Use the `fields` query parameter to control which fields are
//! returned in the response body. Additionally, you can filter results
//! by using query string parameters. For information, see [Filtering
//! and Column Selection](https://wiki.openstack.org/wiki/Neutron/APIv2-
//! specification#Filtering_and_Column_Selection).
//!
//! If the `ip-substring-filtering` extension is enabled, the Neutron API
//! supports IP address substring filtering on the `fixed\_ips` attribute.
//! If you specify an IP address substring (`ip\_address\_substr`) in
//! an entry of the `fixed\_ips` attribute, the Neutron API will list all
//! ports that have an IP address matching the substring.
//!
//! Normal response codes: 200
//!
//! Error response codes: 401
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::BoolString;
use openstack_sdk::api::network::v2::port::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct PortsArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    /// id query parameter for /v2.0/ports API
    #[arg(long)]
    id: Option<String>,

    /// name query parameter for /v2.0/ports API
    #[arg(long)]
    name: Option<String>,

    /// network_id query parameter for /v2.0/ports API
    #[arg(long)]
    network_id: Option<String>,

    /// admin_state_up query parameter for /v2.0/ports API
    #[arg(long)]
    admin_state_up: Option<bool>,

    /// mac_address query parameter for /v2.0/ports API
    #[arg(long)]
    mac_address: Option<String>,

    /// fixed_ips query parameter for /v2.0/ports API
    #[arg(long)]
    fixed_ips: Option<Vec<String>>,

    /// device_id query parameter for /v2.0/ports API
    #[arg(long)]
    device_id: Option<String>,

    /// device_owner query parameter for /v2.0/ports API
    #[arg(long)]
    device_owner: Option<String>,

    /// tenant_id query parameter for /v2.0/ports API
    #[arg(long)]
    tenant_id: Option<String>,

    /// status query parameter for /v2.0/ports API
    #[arg(long)]
    status: Option<String>,

    /// ip_allocation query parameter for /v2.0/ports API
    #[arg(long)]
    ip_allocation: Option<String>,

    /// binding:host_id query parameter for /v2.0/ports API
    #[arg(long)]
    binding_host_id: Option<String>,

    /// revision_number query parameter for /v2.0/ports API
    #[arg(long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/ports API
    #[arg(long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/ports API
    #[arg(long)]
    tags_any: Option<Vec<String>>,

    /// not-tags query parameter for /v2.0/ports API
    #[arg(long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/ports API
    #[arg(long)]
    not_tags_any: Option<Vec<String>>,

    /// description query parameter for /v2.0/ports API
    #[arg(long)]
    description: Option<String>,

    /// security_groups query parameter for /v2.0/ports API
    #[arg(long)]
    security_groups: Option<Vec<String>>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Ports list command
pub struct PortsCmd {
    pub args: PortsArgs,
}
/// Ports response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the resource.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the attached network.
    #[serde()]
    #[structable(optional, wide)]
    network_id: Option<String>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<BoolString>,

    /// The MAC address of the port. If the port uses the `direct-physical`
    /// `vnic\_type` then the value of this field is overwritten with the MAC
    /// address provided in the active binding:profile if any.
    #[serde()]
    #[structable(optional, wide)]
    mac_address: Option<String>,

    /// The IP addresses for the port. If the port has multiple IP addresses,
    /// this field has multiple entries. Each entry consists of IP address
    /// (`ip\_address`) and the subnet ID from which the IP address
    /// is assigned (`subnet\_id`).
    #[serde()]
    #[structable(optional, wide)]
    fixed_ips: Option<VecResponseFixedIps>,

    /// The ID of the device that uses this port.
    /// For example, a server instance or a logical router.
    #[serde()]
    #[structable(optional, wide)]
    device_id: Option<String>,

    /// The entity type that uses this port.
    /// For example, `compute:nova` (server instance), `network:dhcp`
    /// (DHCP agent) or `network:router\_interface` (router interface).
    #[serde()]
    #[structable(optional, wide)]
    device_owner: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The port status. Values are `ACTIVE`, `DOWN`,
    /// `BUILD` and `ERROR`.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// A set of zero or more allowed address pair objects each where address
    /// pair
    /// object contains an `ip\_address` and `mac\_address`. While the
    /// `ip\_address` is required, the `mac\_address` will be taken from the
    /// port if not specified. The value of `ip\_address` can be an IP Address
    /// or a CIDR (if supported by the underlying extension plugin).
    /// A server connected to the port can send a packet with source address
    /// which
    /// matches one of the specified allowed address pairs.
    #[serde()]
    #[structable(optional, wide)]
    allowed_address_pairs: Option<VecResponseAllowedAddressPairs>,

    /// Status of the underlying data plane of a port.
    #[serde()]
    #[structable(optional, wide)]
    data_plane_status: Option<String>,

    /// A set of zero or more extra DHCP option pairs. An
    /// option pair consists of an option value and name.
    #[serde()]
    #[structable(optional, wide)]
    extra_dhcp_opts: Option<VecHashMapStringValue>,

    /// Indicates when ports use either `deferred`, `immediate` or no IP
    /// allocation (`none`).
    #[serde()]
    #[structable(optional, wide)]
    ip_allocation: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    device_profile: Option<String>,

    /// Admin-only. The following values control Open vSwitch’s Userspace Tx
    /// packet steering feature:
    ///
    ///
    /// * `{"openvswitch": {"other\_config": {"tx-steering": "hash|thread"}}}`
    #[serde()]
    #[structable(optional, wide)]
    hints: Option<HashMapStringValue>,

    /// The port NUMA affinity policy requested during the virtual machine
    /// scheduling. Values: `None`, `required`, `preferred` or `legacy`.
    #[serde()]
    #[structable(optional, wide)]
    numa_affinity_policy: Option<String>,

    /// Expose Placement resources (i.e.: `minimum-bandwidth`) and
    /// traits (i.e.: `vnic-type`, `physnet`) requested by a port to
    /// Nova and Placement. A `resource\_request` object contains
    /// `request\_groups` and `same\_subtree` keys. `request\_groups` is a list
    /// of dicts, where each dict represents one group of resources and traits
    /// that needs to be fulfilled from a single resource provider. Every dict
    /// in
    /// the list must contain `id`, `required` and `resources` keys. The
    /// `id` field is a string which represents a unique UUID that is generated
    /// for each group by combining the `port\_id` and UUIDs of the QoS rules
    /// contributing to the group via the UUID5 method. `required` key contains
    /// the traits (generated from the `vnic\_type` and the `physnet`) required
    /// by the port, and a `resources` key contains a mapping of requested
    /// resource class name and requested amount from the QoS policy.
    /// `same\_subtree` key contains a list of `id` values from every resource
    /// group.
    #[serde()]
    #[structable(optional, wide)]
    resource_request: Option<String>,

    /// The type of which mechanism is used for the port.
    /// An API consumer like nova can use this to determine an appropriate way
    /// to
    /// attach a device (for example an interface of a virtual server) to the
    /// port.
    /// Available values currently defined includes
    /// `ovs`, `bridge`, `macvtap`, `hw\_veb`, `hostdev\_physical`,
    /// `vhostuser`, `distributed` and `other`.
    /// There are also special values: `unbound` and `binding\_failed`.
    /// `unbound` means the port is
    /// not bound to a networking back-end. `binding\_failed` means an error
    /// that the port failed to be bound to a networking back-end.
    #[serde(rename = "binding:vif_type")]
    #[structable(optional, title = "binding:vif_type", wide)]
    binding_vif_type: Option<String>,

    /// A dictionary which contains additional information on the port.
    /// Currently the following fields are defined: `port\_filter` and
    /// `ovs\_hybrid\_plug`.
    /// `port\_filter` is a boolean indicating the networking service
    /// provides port filtering features such as security group and/or
    /// anti MAC/IP spoofing.
    /// `ovs\_hybrid\_plug` is a boolean used to inform an API consumer
    /// like nova that the hybrid plugging strategy for OVS should be used.
    #[serde(rename = "binding:vif_details")]
    #[structable(optional, title = "binding:vif_details", wide)]
    binding_vif_details: Option<String>,

    /// The type of vNIC which this port should be attached to. This is used to
    /// determine which mechanism driver(s) to be used to bind the port.
    /// The valid values are `normal`, `macvtap`, `direct`, `baremetal`,
    /// `direct-physical`, `virtio-forwarder`, `smart-nic`
    /// and `remote-managed`.
    /// What type of vNIC is actually available depends on deployments.
    #[serde(rename = "binding:vnic_type")]
    #[structable(optional, title = "binding:vnic_type", wide)]
    binding_vnic_type: Option<String>,

    /// The ID of the host where the port resides.
    #[serde(rename = "binding:host_id")]
    #[structable(optional, title = "binding:host_id", wide)]
    binding_host_id: Option<String>,

    /// A dictionary that enables the application running on the specific host
    /// to
    /// pass and receive vif port information specific to the networking back-
    /// end.
    /// The networking API does not define a specific format of this field.
    /// If the update request is null this response field will be {}.
    #[serde(rename = "binding:profile")]
    #[structable(optional, title = "binding:profile", wide)]
    binding_profile: Option<HashMapStringValue>,

    /// The port security status. A valid value is
    /// enabled (`true`) or disabled (`false`).
    /// If port security is enabled for the port,
    /// security group rules and anti-spoofing rules are applied to
    /// the traffic on the port. If disabled, no such rules are applied.
    #[serde()]
    #[structable(optional, wide)]
    port_security_enabled: Option<BoolString>,

    /// The ID of the QoS policy associated with the port.
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

    /// The ID of the QoS policy of the network where this port is plugged.
    #[serde()]
    #[structable(optional, wide)]
    qos_network_policy_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

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

    /// The uplink status propagation of the port. Valid values are
    /// enabled (`true`) and disabled (`false`).
    #[serde()]
    #[structable(optional, wide)]
    propagate_uplink_status: Option<BoolString>,

    /// A valid DNS name.
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// Data assigned to a port by the Networking internal DNS including the
    /// `hostname`, `ip\_address` and `fqdn`.
    #[serde()]
    #[structable(optional, wide)]
    dns_assignment: Option<String>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The IDs of security groups applied to the port.
    #[serde()]
    #[structable(optional, wide)]
    security_groups: Option<VecString>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFixedIps {
    ip_address: Option<String>,
    subnet_id: Option<String>,
}

impl fmt::Display for ResponseFixedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "subnet_id={}",
                self.subnet_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseFixedIps(Vec<ResponseFixedIps>);
impl fmt::Display for VecResponseFixedIps {
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
struct ResponseAllowedAddressPairs {
    ip_address: Option<String>,
    max_address: Option<String>,
}

impl fmt::Display for ResponseAllowedAddressPairs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "max_address={}",
                self.max_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAllowedAddressPairs(Vec<ResponseAllowedAddressPairs>);
impl fmt::Display for VecResponseAllowedAddressPairs {
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
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecHashMapStringValue(Vec<HashMapStringValue>);
impl fmt::Display for VecHashMapStringValue {
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

#[async_trait]
impl Command for PortsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Ports with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set query parameters
        if let Some(val) = &self.args.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.args.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.query.network_id {
            ep_builder.network_id(val);
        }
        if let Some(val) = &self.args.query.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &self.args.query.mac_address {
            ep_builder.mac_address(val);
        }
        if let Some(val) = &self.args.query.fixed_ips {
            ep_builder.fixed_ips(val.iter());
        }
        if let Some(val) = &self.args.query.device_id {
            ep_builder.device_id(val);
        }
        if let Some(val) = &self.args.query.device_owner {
            ep_builder.device_owner(val);
        }
        if let Some(val) = &self.args.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.args.query.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.query.ip_allocation {
            ep_builder.ip_allocation(val);
        }
        if let Some(val) = &self.args.query.binding_host_id {
            ep_builder.binding_host_id(val);
        }
        if let Some(val) = &self.args.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.args.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.args.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.args.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.args.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.args.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.query.security_groups {
            ep_builder.security_groups(val.iter());
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
