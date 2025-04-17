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

//! Set ResourceProvider command [microversion = 1.0]
//!
//! Wraps invoking of the `resource_providers/{uuid}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::resource_provider::set_10;
use openstack_types::placement::v1::resource_provider::response::set::ResourceProviderResponse;

/// Update the name of the resource provider identified by {uuid}.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400), itemNotFound(404), conflict(409)
///
/// A 409 Conflict response code will be returned if another resource provider
/// exists with the provided name.
#[derive(Args)]
#[command(about = "Update resource provider (microversion = 1.0)")]
pub struct ResourceProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The name of one resource provider.
    #[arg(help_heading = "Body parameters", long)]
    name: String,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// uuid parameter for /resource_providers/{uuid} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_uuid",
        value_name = "UUID"
    )]
    uuid: String,
}

impl ResourceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ResourceProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_10::Request::builder();
        ep_builder.header("OpenStack-API-Version", "placement 1.0");

        // Set path parameters
        ep_builder.uuid(&self.path.uuid);
        // Set query parameters
        // Set body parameters
        // Set Request.name data
        ep_builder.name(&self.name);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResourceProviderResponse>(data)?;
        Ok(())
    }
}
