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

//! Create ExtraSpec command
//!
//! Wraps invoking of the `v2.1/flavors/{flavor_id}/os-extra_specs` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::flavor::extra_spec::create;
use openstack_types::compute::v2::flavor::extra_spec::response::create::ExtraSpecResponse;

/// Creates extra specs for a flavor, by ID.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
#[derive(Args)]
#[command(about = "Create Extra Specs For A Flavor")]
pub struct ExtraSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A dictionary of the flavor’s extra-specs key-and-value pairs. It
    /// appears in the os-extra-specs’ “create” REQUEST body, as well as the
    /// os-extra-specs’ “create” and “list” RESPONSE body.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    extra_specs: Vec<(String, String)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// flavor_id parameter for /v2.1/flavors/{flavor_id}/os-extra_specs/{id}
    /// API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_flavor_id",
        value_name = "FLAVOR_ID"
    )]
    flavor_id: String,
}

impl ExtraSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ExtraSpec");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.flavor/extra_spec"),
            Some("create"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        ep_builder.flavor_id(&self.path.flavor_id);

        // Set body parameters
        // Set Request.extra_specs data

        ep_builder.extra_specs(self.extra_specs.iter().cloned());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ExtraSpecResponse>(data)?;
        Ok(())
    }
}
