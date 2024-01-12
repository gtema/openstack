//! Uploads binary image data.
//! *(Since Image API v2.0)*
//!
//! Set the `Content-Type` request header to `application/octet-stream`.
//!
//! A multiple store backend support is introduced in the Rocky release
//! as a part of the EXPERIMENTAL Image API v2.8.
//!
//! Beginning with API version 2.8, an optional `X-Image-Meta-Store`
//! header may be added to the request. When present, the image data will be
//! placed into the backing store whose identifier is the value of this
//! header. If the store identifier specified is not recognized, a 400 (Bad
//! Request) response is returned. When the header is not present, the image
//! data is placed into the default backing store.
//!
//! Example call:
//!
//! **Preconditions**
//!
//! Before you can store binary image data, you must meet the following
//! preconditions:
//!
//! **Synchronous Postconditions**
//!
//! **Troubleshooting**
//!
//! Normal response codes: 204
//!
//! Error response codes: 400, 401, 403, 404, 409, 410, 413, 415, 503
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

use crate::common::build_upload_asyncread;
use openstack_sdk::api::image::v2::image::file::upload;
use openstack_sdk::api::RawQueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct FileArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Source filename (using "-" will read object from stdout)
    #[arg(long)]
    file: Option<String>,
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

/// File upload command
pub struct FileCmd {
    pub args: FileArgs,
}
/// File response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for FileCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Upload File with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = upload::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
        // Set query parameters
        // Set body parameters
        // The only supported media type
        ep_builder.header("content-type", "application/octet-stream");

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let dst = self.args.file.clone();
        let data = build_upload_asyncread(dst).await?;

        let _rsp: Response<Bytes> = ep.raw_query_read_body_async(client, data).await?;
        // TODO: what if there is an interesting response
        Ok(())
    }
}
