//! Creates a floating IP, and, if you specify port information, associates the
//! floating IP with an internal port.
//!
//! To associate the floating IP with an internal port, specify the
//! port ID attribute in the request body. If you do not specify a
//! port ID in the request, you can issue a PUT request instead of a
//! POST request.
//!
//! Default policy settings enable only administrative users to set
//! floating IP addresses and some non-administrative users might
//! require a floating IP address. If you do not specify a floating IP
//! address in the request, the operation automatically allocates one.
//!
//! By default, this operation associates the floating IP address with
//! a single fixed IP address that is configured on an OpenStack
//! Networking port. If a port has multiple IP addresses, you must
//! specify the `fixed\_ip\_address` attribute in the request body to
//! associate a fixed IP address with the floating IP address.
//!
//! You can create floating IPs on only external networks. When you
//! create a floating IP, you must specify the ID of the network on
//! which you want to create the floating IP. Alternatively, you can
//! create a floating IP on a subnet in the external network, based on
//! the costs and quality of that subnet.
//!
//! You must configure an IP address with the internal OpenStack
//! Networking port that is associated with the floating IP address.
//!
//! The operation returns the `Bad Request (400)` response code for one of
//! reasons:
//!
//! If the port ID is not valid, this operation returns `404` response code.
//!
//! The operation returns the `Conflict (409)` response code for one of
//! reasons:
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 404, 409
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

use openstack_sdk::api::network::v2::floatingip::create;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FloatingipArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    floatingip: Floatingip,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}
/// Floatingip Body data
#[derive(Args, Debug, Clone)]
struct Floatingip {
    /// The floating IP address.
    #[arg(long)]
    floating_ip_address: Option<String>,

    /// The subnet ID on which you want to create the floating IP.
    #[arg(long)]
    subnet_id: Option<String>,

    /// The ID of the network associated with the
    /// floating IP.
    #[arg(long)]
    floating_network_id: Option<String>,

    /// The ID of a port associated with the floating IP.
    /// To associate the floating IP with a fixed IP at creation time,
    /// you must specify the identifier of the internal port.
    #[arg(long)]
    port_id: Option<String>,

    /// The fixed IP address that is associated with the floating IP.
    /// If an internal port has multiple associated IP addresses,
    /// the service chooses the first IP address unless you explicitly
    /// define a fixed IP address in the `fixed\_ip\_address` parameter.
    #[arg(long)]
    fixed_ip_address: Option<String>,

    /// The ID of the project.
    #[arg(long)]
    tenant_id: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    #[arg(long)]
    qos_policy_id: Option<String>,

    /// A valid DNS name.
    #[arg(long)]
    dns_name: Option<String>,

    /// A valid DNS domain.
    #[arg(long)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,
}

/// Floatingip create command
pub struct FloatingipCmd {
    pub args: FloatingipArgs,
}
/// Floatingip response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the floating IP address.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The floating IP address.
    #[serde()]
    #[structable(optional)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the
    /// floating IP.
    #[serde()]
    #[structable(optional, wide)]
    floating_network_id: Option<String>,

    /// The ID of the router for the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    router_id: Option<String>,

    /// The ID of a port associated with the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    port_id: Option<String>,

    /// The fixed IP address that is associated with the
    /// floating IP address.
    #[serde()]
    #[structable(optional, wide)]
    fixed_ip_address: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The status of the floating IP. Values are
    /// `ACTIVE`, `DOWN` and `ERROR`.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    #[serde()]
    #[structable(optional, wide)]
    qos_policy_id: Option<String>,

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

    /// A valid DNS name.
    #[serde()]
    #[structable(optional, wide)]
    dns_name: Option<String>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional, wide)]
    dns_domain: Option<String>,

    /// The information of the port that this floating IP associates with.
    /// In particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network\_id`, `mac\_address`, `admin\_state\_up`, `status`,
    /// `device\_id` and `device\_owner`. If the floating IP is not associated
    /// with a port, this field is `null`.
    #[serde()]
    #[structable(optional, wide)]
    port_details: Option<String>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal\_ip\_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal\_port`) or (`internal\_port\_range`)
    /// and the TCP or UDP port or port range used by floating IP
    /// (`external\_port`) or (`external\_port\_range`).
    #[serde()]
    #[structable(optional, wide)]
    port_forwardings: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

#[async_trait]
impl Command for FloatingipCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Floatingip with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = create::Request::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters

        // Set Request.floatingip data
        let args = &self.args.floatingip;
        let mut floatingip_builder = create::FloatingipBuilder::default();
        if let Some(val) = &args.floating_ip_address {
            floatingip_builder.floating_ip_address(val);
        }

        if let Some(val) = &args.subnet_id {
            floatingip_builder.subnet_id(Some(val.into()));
        }

        if let Some(val) = &args.floating_network_id {
            floatingip_builder.floating_network_id(val);
        }

        if let Some(val) = &args.port_id {
            floatingip_builder.port_id(Some(val.into()));
        }

        if let Some(val) = &args.fixed_ip_address {
            floatingip_builder.fixed_ip_address(val);
        }

        if let Some(val) = &args.tenant_id {
            floatingip_builder.tenant_id(val);
        }

        if let Some(val) = &args.qos_policy_id {
            floatingip_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.dns_name {
            floatingip_builder.dns_name(val);
        }

        if let Some(val) = &args.dns_domain {
            floatingip_builder.dns_domain(val);
        }

        if let Some(val) = &args.description {
            floatingip_builder.description(val);
        }

        ep_builder.floatingip(floatingip_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
