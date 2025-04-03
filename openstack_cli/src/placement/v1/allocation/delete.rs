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

//! Delete Allocation command
//!
//! Wraps invoking of the `allocations/{consumer_uuid}` with `DELETE` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::RawQueryAsync;
use openstack_sdk::api::placement::v1::allocation::delete;
use structable_derive::StructTable;

/// Delete all allocation records for the consumer identified by
/// {consumer_uuid} on all resource providers it is consuming.
///
/// Normal Response Codes: 204
///
/// Error response codes: itemNotFound(404)
///
#[derive(Args)]
#[command(about = "Delete allocations")]
pub struct AllocationCommand {
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
        info!("Delete Allocation");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.consumer_uuid(&self.path.consumer_uuid);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
