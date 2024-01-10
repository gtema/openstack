//! Deletes a router conntrack helper.
//!
//! Normal response codes: 204
//!
//! Error response codes: 404
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
use openstack_sdk::api::network::v2::router::conntrack_helper::delete;
use openstack_sdk::api::network::v2::router::conntrack_helper::find;
use openstack_sdk::api::RawQueryAsync;

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

/// ConntrackHelper delete command
pub struct ConntrackHelperCmd {
    pub args: ConntrackHelperArgs,
}
/// ConntrackHelper response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for ConntrackHelperCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete ConntrackHelper with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);
        let mut ep_builder = delete::Request::builder();
        // Set path parameters
        ep_builder.router_id(&self.args.path.router_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
