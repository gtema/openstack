//! Requests that a set of images be pre-cached on compute nodes within the
//! referenced aggregate.
//!
//! This API is available starting with microversion 2.81.
//!
//! Normal response codes: 202
//!
//! Error response codes: badRequest(400), unauthorized(401), forbidden(403),
//! itemNotFound(404)
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

use openstack_sdk::api::compute::v2::aggregate::image::cache_281;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(action=clap::ArgAction::Append, long)]
    cache: Vec<String>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/os-aggregates/{id}/images API
    #[arg()]
    id: String,
}

/// Image action command
pub struct ImageCmd {
    pub args: ImageArgs,
}
/// Image response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = cache_281::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.81");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.cache data
        let args = &self.args.cache;

        let cache_builder: Vec<cache_281::Cache> = args
            .iter()
            .flat_map(|v| cache_281::CacheBuilder::default().id(v).build())
            .collect();
        ep_builder.cache(cache_builder);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
