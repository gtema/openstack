//! Updates a router conntrack helper.
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
use openstack_sdk::api::network::v2::router::conntrack_helper::find;
use openstack_sdk::api::network::v2::router::conntrack_helper::set;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ConntrackHelperArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    conntrack_helper: ConntrackHelper,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
    #[arg()]
    router_id: String,

    /// id parameter for /v2.0/routers/{router_id}/conntrack_helpers/{id} API
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

/// ConntrackHelper Body data
#[derive(Args, Debug, Clone)]
struct ConntrackHelper {
    /// The network protocol for the netfilter conntrack target rule.
    #[arg(long)]
    protocol: Option<Protocol>,

    /// The network port for the netfilter conntrack target rule.
    #[arg(long)]
    port: Option<f32>,

    /// The netfilter conntrack helper module.
    #[arg(long)]
    helper: Option<String>,
}

/// ConntrackHelper set command
pub struct ConntrackHelperCmd {
    pub args: ConntrackHelperArgs,
}
/// ConntrackHelper response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the conntrack helper.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The network protocol for the netfilter conntrack target rule.
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    #[serde()]
    #[structable(optional)]
    port: Option<f32>,

    /// The netfilter conntrack helper module.
    #[serde()]
    #[structable(optional)]
    helper: Option<String>,
}

#[async_trait]
impl Command for ConntrackHelperCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ConntrackHelper with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.router_id(&self.args.path.router_id);
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
        ep_builder.router_id(resource_id.clone());
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.conntrack_helper data
        let args = &self.args.conntrack_helper;
        let mut conntrack_helper_builder = set::ConntrackHelperBuilder::default();
        if let Some(val) = &args.protocol {
            let tmp = match val {
                Protocol::Dccp => set::Protocol::Dccp,
                Protocol::Icmp => set::Protocol::Icmp,
                Protocol::Ipv6Icmp => set::Protocol::Ipv6Icmp,
                Protocol::Sctp => set::Protocol::Sctp,
                Protocol::Tcp => set::Protocol::Tcp,
                Protocol::Udp => set::Protocol::Udp,
            };
            conntrack_helper_builder.protocol(tmp);
        }

        if let Some(val) = &args.port {
            conntrack_helper_builder.port(*val);
        }

        if let Some(val) = &args.helper {
            conntrack_helper_builder.helper(val.clone());
        }

        ep_builder.conntrack_helper(conntrack_helper_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
