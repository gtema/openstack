//! Updates a floating IP port forwarding.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 404
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

use clap::ValueEnum;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::find;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::set;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct PortForwardingArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    port_forwarding: PortForwarding,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// floatingip_id parameter for /v2.0/floatingips/{floatingip_id}/tags/{id}
    /// API
    #[arg()]
    floatingip_id: String,

    /// id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    #[arg()]
    id: String,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Dccp,
    Icmp,
    Ipv6Icmp,
    Sctp,
    Tcp,
    Udp,
}

/// PortForwarding Body data
#[derive(Args, Debug, Clone)]
struct PortForwarding {
    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP
    /// address.
    #[arg(long)]
    external_port: Option<Option<f32>>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[arg(long)]
    internal_port: Option<Option<f32>>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP
    /// port forwarding.
    #[arg(long)]
    internal_ip_address: Option<String>,

    /// The IP protocol used in the floating IP port forwarding.
    #[arg(long)]
    protocol: Option<Protocol>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[arg(long)]
    internal_port_id: Option<String>,

    #[arg(long)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP
    /// address.
    #[arg(long)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[arg(long)]
    internal_port_range: Option<f32>,
}

/// PortForwarding set command
pub struct PortForwardingCmd {
    pub args: PortForwardingArgs,
}
/// PortForwarding response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the floating IP port forwarding.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP
    /// address.
    #[serde()]
    #[structable(optional)]
    external_port: Option<f32>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port: Option<f32>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP
    /// port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_ip_address: Option<String>,

    /// The IP protocol used in the floating IP port forwarding.
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port_id: Option<String>,

    /// A text describing the rule, which helps users to
    /// manage/find easily theirs rules.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP
    /// address.
    #[serde()]
    #[structable(optional)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional)]
    internal_port_range: Option<f32>,
}

#[async_trait]
impl Command for PortForwardingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set PortForwarding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.floatingip_id(&self.args.path.floatingip_id);
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
        ep_builder.floatingip_id(resource_id.clone());
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.port_forwarding data
        let args = &self.args.port_forwarding;
        let mut port_forwarding_builder = set::PortForwardingBuilder::default();
        if let Some(val) = &args.external_port {
            port_forwarding_builder.external_port(val.clone().map(|v| v.into()));
        }

        if let Some(val) = &args.internal_port {
            port_forwarding_builder.internal_port(val.clone().map(|v| v.into()));
        }

        if let Some(val) = &args.internal_ip_address {
            port_forwarding_builder.internal_ip_address(val.clone());
        }

        if let Some(val) = &args.protocol {
            let tmp = match val {
                Protocol::Dccp => set::Protocol::Dccp,
                Protocol::Icmp => set::Protocol::Icmp,
                Protocol::Ipv6Icmp => set::Protocol::Ipv6Icmp,
                Protocol::Sctp => set::Protocol::Sctp,
                Protocol::Tcp => set::Protocol::Tcp,
                Protocol::Udp => set::Protocol::Udp,
            };
            port_forwarding_builder.protocol(tmp);
        }

        if let Some(val) = &args.internal_port_id {
            port_forwarding_builder.internal_port_id(val.clone());
        }

        if let Some(val) = &args.description {
            port_forwarding_builder.description(val.clone());
        }

        if let Some(val) = &args.external_port_range {
            port_forwarding_builder.external_port_range(*val);
        }

        if let Some(val) = &args.internal_port_range {
            port_forwarding_builder.internal_port_range(*val);
        }

        ep_builder.port_forwarding(port_forwarding_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
