//! Creates extra specs for a flavor, by ID.
//!
//! Normal response codes: 200
//!
//! Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
//! conflict(409)
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

use crate::common::parse_key_val;
use crate::common::NumString;
use openstack_sdk::api::compute::v2::flavor::os_extra_spec::create;
use openstack_sdk::api::RawQueryAsync;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct OsExtraSpecArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    extra_specs: Vec<(String, String)>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-flavor-access API
    #[arg()]
    flavor_id: String,
}

/// OsExtraSpec create command
pub struct OsExtraSpecCmd {
    pub args: OsExtraSpecArgs,
}
/// OsExtraSpec response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for OsExtraSpecCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create OsExtraSpec with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        ep_builder.flavor_id(&self.args.path.flavor_id);
        // Set query parameters

        // Set body parameters

        // Set Request.extra_specs data
        let args = &self.args.extra_specs;

        ep_builder.extra_specs(args.iter().cloned());

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
