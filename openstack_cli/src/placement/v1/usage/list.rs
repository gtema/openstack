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

//! List Usages command
//!
//! Wraps invoking of the `usages` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::usage::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Return a report of usage information for resources associated with the
/// project identified by project_id and user identified by user_id. The value
/// is a dictionary of resource classes paired with the sum of the allocations
/// of that resource class for provided parameters.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400)
///
#[derive(Args)]
#[command(about = "List usages")]
pub struct UsagesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// A string that consists of numbers, A-Z, and _ describing the consumer
    /// type by which to filter usage results. For example, to retrieve only
    /// usage information for ‘INSTANCE’ type consumers a parameter of
    /// consumer_type=INSTANCE should be provided. The all query parameter may
    /// be specified to group all results under one key, all. The unknown query
    /// parameter may be specified to group all results under one key, unknown.
    ///
    #[arg(help_heading = "Query parameters", long)]
    consumer_type: Option<String>,

    /// The uuid of a project.
    ///
    #[arg(help_heading = "Query parameters", long)]
    project_id: Option<String>,

    /// The uuid of a user.
    ///
    #[arg(help_heading = "Query parameters", long)]
    user_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Usages response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The number of consumers of a particular consumer_type.
    ///
    #[serde()]
    #[structable()]
    consumer_count: i32,
}

impl UsagesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Usages");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.query.user_id {
            ep_builder.user_id(val);
        }
        if let Some(val) = &self.query.consumer_type {
            ep_builder.consumer_type(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
