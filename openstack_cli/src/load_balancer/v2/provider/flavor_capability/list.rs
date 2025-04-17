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

//! List FlavorCapabilities command
//!
//! Wraps invoking of the `v2/lbaas/providers/{provider}/flavor_capabilities` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::load_balancer::v2::provider::flavor_capability::list;
use openstack_types::load_balancer::v2::provider::flavor_capability::response::list::FlavorCapabilityResponse;

/// Shows the provider driver flavor capabilities. These are the features of
/// the provider driver that can be configured in an Octavia flavor. This API
/// returns a list of dictionaries with the name and description of each flavor
/// capability of the provider.
///
/// The list might be empty and a provider driver may not implement this
/// feature.
///
/// **New in version 2.6**
#[derive(Args)]
#[command(about = "Show Provider Flavor Capabilities")]
pub struct FlavorCapabilitiesCommand {
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
    /// provider parameter for
    /// /v2/lbaas/providers/{provider}/flavor_capabilities API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_provider",
        value_name = "PROVIDER"
    )]
    provider: String,
}

impl FlavorCapabilitiesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List FlavorCapabilities");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.provider(&self.path.provider);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<FlavorCapabilityResponse>(data)?;
        Ok(())
    }
}
