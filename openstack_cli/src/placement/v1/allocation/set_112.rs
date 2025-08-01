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

//! Set Allocation command [microversion = 1.12]
//!
//! Wraps invoking of the `allocations/{consumer_uuid}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::allocation::set_112;
use serde_json::Value;

/// Create or update one or more allocation records representing the
/// consumption of one or more classes of resources from one or more resource
/// providers by the consumer identified by {consumer_uuid}. If allocations
/// already exist for this consumer, they are replaced.
///
/// Normal Response Codes: 204
///
/// Error response codes: badRequest(400), itemNotFound(404), conflict(409)
#[derive(Args)]
#[command(about = "Update allocations (microversion = 1.12)")]
pub struct AllocationCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    allocations: Vec<(String, Value)>,

    #[arg(help_heading = "Body parameters", long)]
    project_id: String,

    #[arg(help_heading = "Body parameters", long)]
    user_id: String,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// consumer_uuid parameter for /allocations/{consumer_uuid} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_consumer_uuid",
        value_name = "CONSUMER_UUID"
    )]
    consumer_uuid: String,
}

impl AllocationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Allocation");

        let op = OutputProcessor::from_args(parsed_args, Some("placement.allocation"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_112::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("placement 1.12"),
        );

        ep_builder.consumer_uuid(&self.path.consumer_uuid);

        // Set body parameters
        // Set Request.allocations data

        ep_builder.allocations(
            self.allocations
                .iter()
                .map(|(k, v)| {
                    serde_json::from_value(v.to_owned()).map(|v: set_112::AllocationsItem| (k, v))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        );

        // Set Request.project_id data
        ep_builder.project_id(&self.project_id);

        // Set Request.user_id data
        ep_builder.user_id(&self.user_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
