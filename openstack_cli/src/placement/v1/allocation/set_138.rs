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

//! Set Allocation command [microversion = 1.38]
//!
//! Wraps invoking of the `allocations/{consumer_uuid}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::placement::v1::allocation::set_138;
use serde_json::Value;
use structable_derive::StructTable;

/// Create or update one or more allocation records representing the
/// consumption of one or more classes of resources from one or more resource
/// providers by the consumer identified by {consumer_uuid}. If allocations
/// already exist for this consumer, they are replaced.
///
/// Normal Response Codes: 204
///
/// Error response codes: badRequest(400), itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Update allocations (microversion = 1.38)")]
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
    consumer_generation: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    consumer_type: String,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    mappings: Option<Vec<(String, Value)>>,

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
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_consumer_uuid",
        value_name = "CONSUMER_UUID"
    )]
    consumer_uuid: String,
}
/// Allocation response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl AllocationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Allocation");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_138::Request::builder();
        ep_builder.header("OpenStack-API-Version", "placement 1.38");

        // Set path parameters
        ep_builder.consumer_uuid(&self.path.consumer_uuid);
        // Set query parameters
        // Set body parameters
        // Set Request.allocations data

        ep_builder.allocations(
            self.allocations
                .iter()
                .map(|(k, v)| {
                    serde_json::from_value(v.to_owned()).map(|v: set_138::AllocationsItem| (k, v))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        );

        // Set Request.consumer_generation data
        if let Some(val) = &self.consumer_generation {
            ep_builder.consumer_generation(*val);
        }

        // Set Request.consumer_type data
        ep_builder.consumer_type(&self.consumer_type);

        // Set Request.mappings data
        if let Some(arg) = &self.mappings {
            ep_builder.mappings(
                arg.iter()
                    .map(|(k, v)| {
                        serde_json::from_value::<Vec<String>>(v.to_owned())
                            .map(|v| (k, v.into_iter()))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter(),
            );
        }

        // Set Request.project_id data
        ep_builder.project_id(&self.project_id);

        // Set Request.user_id data
        ep_builder.user_id(&self.user_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
