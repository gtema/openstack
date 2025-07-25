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

//! Set Aggregate command [microversion = 1.19]
//!
//! Wraps invoking of the `resource_providers/{uuid}/aggregates` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::resource_provider::aggregate::set_119;
use openstack_types::placement::v1::resource_provider::aggregate::response::set::AggregateResponse;

/// Associate a list of aggregates with the resource provider identified by
/// {uuid}.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400), itemNotFound(404), conflict(409)
#[derive(Args)]
#[command(about = "Update resource provider aggregates (microversion = 1.19)")]
pub struct AggregateCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    aggregates: Vec<String>,

    #[arg(help_heading = "Body parameters", long)]
    resource_provider_generation: i32,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// uuid parameter for /resource_providers/{uuid}/aggregates API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_uuid",
        value_name = "UUID"
    )]
    uuid: String,
}

impl AggregateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Aggregate");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("placement.resource_provider/aggregate"),
            Some("set"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_119::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("placement 1.19"),
        );

        ep_builder.uuid(&self.path.uuid);

        // Set body parameters
        // Set Request.aggregates data

        ep_builder.aggregates(self.aggregates.iter().map(Into::into).collect::<Vec<_>>());

        // Set Request.resource_provider_generation data
        ep_builder.resource_provider_generation(self.resource_provider_generation);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<AggregateResponse>(data)?;
        Ok(())
    }
}
