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

//! Delete Image command
//!
//! Wraps invoking of the `v2/images/{image_id}` with `DELETE` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::image::delete;

/// (Since Image API v2.0) Deletes an image.
///
/// You cannot delete images with the `protected` attribute set to `true`
/// (boolean).
///
/// Preconditions
///
/// Synchronous Postconditions
///
/// Normal response codes: 204
///
/// Error response codes: 400, 401, 403, 404, 409
#[derive(Args)]
#[command(about = "Delete image")]
pub struct ImageCommand {
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
    /// image_id parameter for /v2/images/{image_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Image");

        let op = OutputProcessor::from_args(parsed_args, Some("image.image"), Some("delete"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        ep_builder.id(&self.path.id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
