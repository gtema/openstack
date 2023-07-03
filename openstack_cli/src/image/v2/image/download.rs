//! Downloads binary image data. (Since Image API v2.0)
//! Example call: `curl -i -X GET -H "X-Auth-Token: $token"
//!   $image_url/v2/images/{image_id}/file`
//!
//! The response body contains the raw binary data that represents the actual
//! virtual disk. The Content-Type header contains the application/octet-stream
//! value. The Content-MD5 header contains an MD5 checksum of the image data.
//! Use this checksum to verify the integrity of the image data.
//! Preconditions:
//!
//!   - The image must exist.
//!
//! Synchronous Postconditions:
//!
//!   - You can download the binary image data in your machine if the image
//!     has image data.
//!
//!   - If image data exists, the call returns the HTTP 200 response code
//!     for a full image download request.
//!
//!   - If image data exists, the call returns the HTTP 206 response code
//!     for a partial download request.
//!
//!   - If no image data exists, the call returns the HTTP 204 (No Content)
//!     response code.
//!
//!   - If no image record exists, the call returns the HTTP 404 response
//!     code for an attempted full image download request.
//!
//!   - For an unsatisfiable partial download request, the call returns the
//!     HTTP 416 response code.
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::download_file;
use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::file::get;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::RawQueryAsync;

/// Downloads binary image data. (Since Image API v2.0)
/// Example call: `curl -i -X GET -H "X-Auth-Token: $token"
///   $image_url/v2/images/{image_id}/file`
///
/// The response body contains the raw binary data that represents the actual
/// virtual disk. The Content-Type header contains the application/octet-stream
/// value. The Content-MD5 header contains an MD5 checksum of the image data.
/// Use this checksum to verify the integrity of the image data.
/// Preconditions:
///
///   - The image must exist.
///
/// Synchronous Postconditions:
///
///   - You can download the binary image data in your machine if the image
///     has image data.
///
///   - If image data exists, the call returns the HTTP 200 response code
///     for a full image download request.
///
///   - If image data exists, the call returns the HTTP 206 response code
///     for a partial download request.
///
///   - If no image data exists, the call returns the HTTP 204 (No Content)
///     response code.
///
///   - If no image record exists, the call returns the HTTP 404 response
///     code for an attempted full image download request.
///
///   - For an unsatisfiable partial download request, the call returns the
///     HTTP 416 response code.
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// Image ID
    #[arg()]
    id: String,

    /// Destination filename (using "-" will print object to stdout)
    #[arg(long)]
    file: Option<String>,
}

pub struct ImageCmd {
    pub args: ImageArgs,
}

/// Image
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Image {}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Image::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client.discover_service_endpoint("image").await?;

        let (headers, data) = ep.download_async(client).await?;

        let size: u64 = headers
            .get("content-length")
            .map(|x| x.to_str().expect("Header is a string"))
            .unwrap_or("0")
            .parse()
            .unwrap();
        download_file(
            self.args.file.clone().unwrap_or(self.args.id.clone()),
            size,
            data,
        )
        .await?;
        Ok(())
    }
}
