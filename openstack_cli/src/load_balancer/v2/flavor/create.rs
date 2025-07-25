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

//! Create Flavor command
//!
//! Wraps invoking of the `v2/lbaas/flavors` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::load_balancer::v2::flavor::create;
use openstack_types::load_balancer::v2::flavor::response::create::FlavorResponse;

/// Creates a flavor.
#[derive(Args)]
pub struct FlavorCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines mandatory and optional attributes of a POST request.
    #[command(flatten)]
    flavor: Flavor,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Flavor Body data
#[derive(Args, Clone)]
struct Flavor {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    flavor_profile_id: String,

    #[arg(help_heading = "Body parameters", long)]
    name: String,
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Flavor");

        let op =
            OutputProcessor::from_args(parsed_args, Some("load-balancer.flavor"), Some("create"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set body parameters
        // Set Request.flavor data
        let args = &self.flavor;
        let mut flavor_builder = create::FlavorBuilder::default();
        if let Some(val) = &args.description {
            flavor_builder.description(val);
        }

        if let Some(val) = &args.enabled {
            flavor_builder.enabled(*val);
        }

        flavor_builder.flavor_profile_id(&args.flavor_profile_id);

        flavor_builder.name(&args.name);

        ep_builder.flavor(flavor_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<FlavorResponse>(data)?;
        Ok(())
    }
}
