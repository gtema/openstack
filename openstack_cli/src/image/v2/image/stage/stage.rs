//! Places the binary image data in a staging area. It is not stored in
//! the storage backend and is not accessible for download until after
//! the [Image Import](#image-import-call) call is made.
//! *(Since Image API v2.6)*
//!
//! Set the `Content-Type` request header to `application/octet-stream`.
//!
//! Example call:
//!
//! **Preconditions**
//!
//! Before you can stage binary image data, you must meet the following
//! preconditions:
//!
//! **Synchronous Postconditions**
//!
//! **Troubleshooting**
//!
//! Normal response codes: 204
//!
//! Error response codes: 400, 401, 403, 404, 405, 409, 410, 413, 415, 503
//!
//! If the image import process is not enabled in your cloud, this request
//! will result in a 404 response code with an appropriate message.
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

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::image::v2::image::stage::stage;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct StageArgs {
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
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    image_id: String,
}

/// Stage action command
pub struct StageCmd {
    pub args: StageArgs,
}
/// Stage response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for StageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Stage with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = stage::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
        // Set query parameters
        // Set body parameters

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
