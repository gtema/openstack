// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Upload File command
//!
//! Wraps invoking of the `v2/images/{image_id}/file` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::build_upload_asyncread;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::image::v2::image::file::upload;

/// Uploads binary image data. *(Since Image API v2.0)*
///
/// Set the `Content-Type` request header to `application/octet-stream`.
///
/// A multiple store backend support is introduced in the Rocky release as a
/// part of the EXPERIMENTAL Image API v2.8.
///
/// Beginning with API version 2.8, an optional `X-Image-Meta-Store` header may
/// be added to the request. When present, the image data will be placed into
/// the backing store whose identifier is the value of this header. If the
/// store identifier specified is not recognized, a 400 (Bad Request) response
/// is returned. When the header is not present, the image data is placed into
/// the default backing store.
///
/// Example call:
///
/// **Preconditions**
///
/// Before you can store binary image data, you must meet the following
/// preconditions:
///
/// **Synchronous Postconditions**
///
/// **Troubleshooting**
///
/// Normal response codes: 204
///
/// Error response codes: 400, 401, 403, 404, 409, 410, 413, 415, 503
#[derive(Args)]
#[command(about = "Upload binary image data")]
pub struct FileCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Request Headers parameters
    #[command(flatten)]
    headers: HeaderParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Source filename (using "-" will read object from stdout)
    #[arg(long)]
    file: Option<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Header parameters
#[derive(Args)]
struct HeaderParameters {
    /// The media type descriptor of the body, namely application/octet-stream
    #[arg(long)]
    content_type: Option<String>,

    /// A store identifier to upload or import image data. Should only be
    /// included when making a request to a cloud that supports multiple
    /// backing stores. Use the Store Discovery call to determine an
    /// appropriate store identifier. Simply omit this header to use the
    /// default store.
    #[arg(long)]
    x_image_meta_store: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/file API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_image_id",
        value_name = "IMAGE_ID"
    )]
    image_id: String,
}

impl FileCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Upload File");

        let op = OutputProcessor::from_args(parsed_args, Some("image.image/file"), Some("upload"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = upload::Request::builder();
        // Set path parameters
        ep_builder.image_id(&self.path.image_id);

        // Set header parameters
        if let Some(val) = &self.headers.content_type {
            ep_builder.header(
                http::header::HeaderName::from_static("Content-Type"),
                http::header::HeaderValue::from_str(val)?,
            );
        }
        if let Some(val) = &self.headers.x_image_meta_store {
            ep_builder.header(
                http::header::HeaderName::from_static("X-Image-Meta-Store"),
                http::header::HeaderValue::from_str(val)?,
            );
        }

        // The only supported media type
        ep_builder.header(
            http::header::CONTENT_TYPE,
            http::header::HeaderValue::from_static("application/octet-stream"),
        );

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let dst = self.file.clone();
        let data = build_upload_asyncread(dst).await?;

        let _rsp = ep.raw_query_read_body_async(client, data).await?;
        // TODO: what if there is an interesting response
        Ok(())
    }
}
