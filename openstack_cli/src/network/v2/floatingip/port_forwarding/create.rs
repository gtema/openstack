//! Creates a floating IP port forwarding.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 404
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use clap::ValueEnum;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::create;
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
    #[arg(long)]
    project_id: Option<String>,

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

    /// A text describing the rule, which helps users to
    /// manage/find easily theirs rules.
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

/// PortForwarding create command
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
impl OSCCommand for PortForwardingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create PortForwarding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.floatingip_id(&self.args.path.floatingip_id);
        // Set query parameters
        // Set body parameters
        // Set Request.port_forwarding data
        let args = &self.args.port_forwarding;
        let mut port_forwarding_builder = create::PortForwardingBuilder::default();
        if let Some(val) = &args.project_id {
            port_forwarding_builder.project_id(val.clone());
        }

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
                Protocol::Dccp => create::Protocol::Dccp,
                Protocol::Icmp => create::Protocol::Icmp,
                Protocol::Ipv6Icmp => create::Protocol::Ipv6Icmp,
                Protocol::Sctp => create::Protocol::Sctp,
                Protocol::Tcp => create::Protocol::Tcp,
                Protocol::Udp => create::Protocol::Udp,
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
