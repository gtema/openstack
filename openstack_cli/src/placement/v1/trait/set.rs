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

//! Set Trait command
//!
//! Wraps invoking of the `traits/{name}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::r#trait::set;

/// Insert a new custom trait. If traits already exists 204 will be returned.
///
/// There are two kinds of traits: the standard traits and the custom traits.
/// The standard traits are interoperable across different OpenStack cloud
/// deployments. The definition of standard traits comes from the os-traits
/// library. The standard traits are read-only in the placement API which means
/// that the user can’t modify any standard traits through API. The custom
/// traits are used by admin users to manage the non-standard qualitative
/// information of resource providers.
///
/// Normal Response Codes: 201, 204
///
/// Error response codes: badRequest(400)
#[derive(Args)]
#[command(about = "Update traits")]
pub struct TraitCommand {
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
    /// name parameter for /traits/{name} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_name",
        value_name = "NAME"
    )]
    name: String,
}

impl TraitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Trait");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.name(&self.path.name);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
