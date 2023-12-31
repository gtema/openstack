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

/// Updates a floating IP port forwarding.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 404
#[derive(Args, Clone, Debug)]
pub struct PortForwardingArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    port_forwarding: Option<PortForwarding>,
}
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Protocol {
    Icmp,
    Udp,
    Tcp,
    Ipv6Icmp,
    Dccp,
    Sctp,
}

impl fmt::Debug for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

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

pub struct PortForwardingCmd {
    pub args: PortForwardingArgs,
}
/// PortForwarding
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
    #[structable(optional, wide)]
    external_port: Option<Option<f32>>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional, wide)]
    internal_port: Option<Option<f32>>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP
    /// port forwarding.
    #[serde()]
    #[structable(optional, wide)]
    internal_ip_address: Option<String>,

    /// The IP protocol used in the floating IP port forwarding.
    #[serde()]
    #[structable(optional, wide)]
    protocol: Option<String>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    #[serde()]
    #[structable(optional, wide)]
    internal_port_id: Option<String>,

    /// A text describing the rule, which helps users to
    /// manage/find easily theirs rules.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP
    /// address.
    #[serde()]
    #[structable(optional, wide)]
    external_port_range: Option<f32>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    #[serde()]
    #[structable(optional, wide)]
    internal_port_range: Option<f32>,
}

#[async_trait]
impl Command for PortForwardingCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Put PortForwarding with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = find::Request::builder();
        // Set path parameters
        ep_builder.floatingip_id(&self.args.path.floatingip_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        // Set Request.port_forwarding data
        if let Some(args) = &self.args.port_forwarding {
            let mut port_forwarding_builder = set::PortForwardingBuilder::default();
            if let Some(val) = &args.external_port {
                port_forwarding_builder.external_port(val.clone().map(|v| v.into()));
            }

            if let Some(val) = &args.internal_port {
                port_forwarding_builder.internal_port(val.clone().map(|v| v.into()));
            }

            if let Some(val) = &args.internal_ip_address {
                port_forwarding_builder.internal_ip_address(val);
            }

            if let Some(val) = &args.protocol {
                let tmp = match val {
                    Protocol::Icmp => set::Protocol::Icmp,
                    Protocol::Udp => set::Protocol::Udp,
                    Protocol::Tcp => set::Protocol::Tcp,
                    Protocol::Ipv6Icmp => set::Protocol::Ipv6Icmp,
                    Protocol::Dccp => set::Protocol::Dccp,
                    Protocol::Sctp => set::Protocol::Sctp,
                };
                port_forwarding_builder.protocol(tmp);
            }

            if let Some(val) = &args.internal_port_id {
                port_forwarding_builder.internal_port_id(val);
            }

            if let Some(val) = &args.description {
                port_forwarding_builder.description(val);
            }

            if let Some(val) = &args.external_port_range {
                port_forwarding_builder.external_port_range(*val);
            }

            if let Some(val) = &args.internal_port_range {
                port_forwarding_builder.internal_port_range(*val);
            }

            ep_builder.port_forwarding(port_forwarding_builder.build().unwrap());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Network)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
