//! Lists router conntrack helpers associated with a router.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body.
//! Additionally, you can filter results by using query string parameters.
//! For information, see [Filtering and Column Selection](https://wiki.openstac
//! k.org/wiki/Neutron/APIv2-specification#Filtering_and_Column_Selection).
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

use openstack_sdk::api::network::v2::router::conntrack_helper::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ConntrackHelpersArgs {
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
    /// id query parameter for /v2.0/routers/{router_id}/conntrack_helpers API
    #[arg(long)]
    id: Option<String>,

    /// protocol query parameter for
    /// /v2.0/routers/{router_id}/conntrack_helpers API
    #[arg(long)]
    protocol: Option<String>,

    /// port query parameter for /v2.0/routers/{router_id}/conntrack_helpers
    /// API
    #[arg(long)]
    port: Option<f32>,

    /// helper query parameter for /v2.0/routers/{router_id}/conntrack_helpers
    /// API
    #[arg(long)]
    helper: Option<String>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
    #[arg()]
    router_id: String,
}

/// ConntrackHelpers list command
pub struct ConntrackHelpersCmd {
    pub args: ConntrackHelpersArgs,
}
/// ConntrackHelpers response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the conntrack helper.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The network protocol for the netfilter conntrack target rule.
    #[serde()]
    #[structable(optional, wide)]
    protocol: Option<String>,

    /// The network port for the netfilter conntrack target rule.
    #[serde()]
    #[structable(optional, wide)]
    port: Option<f32>,

    /// The netfilter conntrack helper module.
    #[serde()]
    #[structable(optional, wide)]
    helper: Option<String>,
}

#[async_trait]
impl Command for ConntrackHelpersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ConntrackHelpers with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = list::Request::builder();
        // Set path parameters
        ep_builder.router_id(&self.args.path.router_id);
        // Set query parameters
        if let Some(val) = &self.args.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.args.query.protocol {
            ep_builder.protocol(val);
        }
        if let Some(val) = &self.args.query.port {
            ep_builder.port(*val);
        }
        if let Some(val) = &self.args.query.helper {
            ep_builder.helper(val);
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
