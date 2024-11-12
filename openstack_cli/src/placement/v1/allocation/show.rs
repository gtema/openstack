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

//! Show Allocation command
//!
//! Wraps invoking of the `allocations/{consumer_uuid}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::allocation::get;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// List all allocation records for the consumer identified by {consumer_uuid}
/// on all the resource providers it is consuming.
///
/// Normal Response Codes: 200
///
#[derive(Args)]
#[command(about = "List allocations")]
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
struct ResponseData {
    /// A dictionary of allocations keyed by resource provider uuid.
    ///
    #[serde()]
    #[structable(pretty)]
    allocations: Value,

    /// The generation of the consumer. Will be absent when listing allocations
    /// for a consumer uuid that has no allocations.
    ///
    /// **New in version 1.28**
    ///
    #[serde()]
    #[structable(optional)]
    consumer_generation: Option<i32>,

    /// A string that consists of numbers, `A-Z`, and `_` describing what kind
    /// of consumer is creating, or has created, allocations using a quantity
    /// of inventory. The string is determined by the client when writing
    /// allocations and it is up to the client to ensure correct choices
    /// amongst collaborating services. For example, the compute service may
    /// choose to type some consumers ‘INSTANCE’ and others ‘MIGRATION’.
    ///
    /// **New in version 1.38**
    ///
    #[serde()]
    #[structable(optional)]
    consumer_type: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    mappings: Option<Value>,

    /// The uuid of a project. Will be absent when listing allocations for a
    /// consumer uuid that has no allocations.
    ///
    /// **New in version 1.12**
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The uuid of a user. Will be absent when listing allocations for a
    /// consumer uuid that has no allocations.
    ///
    /// **New in version 1.12**
    ///
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseAllocations {
    generation: Option<i32>,
    resources: Value,
}

impl fmt::Display for ResponseAllocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "generation={}",
                self.generation.map_or(String::new(), |v| v.to_string())
            ),
            format!("resources={}", self.resources),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl AllocationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Allocation");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.consumer_uuid(&self.path.consumer_uuid);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}