//! Shows information for a router conntrack helper.
//!
//! Use the `fields` query parameter to control which fields are returned in
//! the response body.
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

use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::router::conntrack_helper::find;
use openstack_sdk::api::network::v2::router::conntrack_helper::get;
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

/// ConntrackHelper show command
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
        info!("Show ConntrackHelper with {:?}", self.args);

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

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
