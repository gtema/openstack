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

//! Show Member command
//!
//! Wraps invoking of the `v2/images/{image_id}/members/{member_id}` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::image::member::get;
use openstack_types::image::v2::image::member::response::get::MemberResponse;

/// Shows image member details. *(Since Image API v2.1)*
///
/// Response body is a single image member entity.
///
/// Preconditions
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404
#[derive(Args)]
#[command(about = "Show image member details")]
pub struct MemberCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_image_id",
        value_name = "IMAGE_ID"
    )]
    image_id: String,

    /// member_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl MemberCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Member");

        let op = OutputProcessor::from_args(parsed_args, Some("image.image/member"), Some("show"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        ep_builder.image_id(&self.path.image_id);
        ep_builder.id(&self.path.id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<MemberResponse>(data)?;
        Ok(())
    }
}
