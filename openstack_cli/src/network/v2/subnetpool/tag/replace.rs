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

//! Set Tag command
//!
//! Wraps invoking of the `v2.0/subnetpools/{subnetpool_id}/tags` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::subnetpool::tag::replace;
use openstack_types::network::v2::subnetpool::tag::response::replace::TagResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct TagCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Vec<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// subnetpool_id parameter for /v2.0/subnetpools/{subnetpool_id}/tags/{id}
    /// API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_subnetpool_id",
        value_name = "SUBNETPOOL_ID"
    )]
    subnetpool_id: String,
}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Tag");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = replace::Request::builder();

        // Set path parameters
        ep_builder.subnetpool_id(&self.path.subnetpool_id);
        // Set query parameters
        // Set body parameters
        // Set Request.tags data

        ep_builder.tags(self.tags.iter().map(Into::into).collect::<Vec<_>>());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<TagResponse>(data)?;
        Ok(())
    }
}
