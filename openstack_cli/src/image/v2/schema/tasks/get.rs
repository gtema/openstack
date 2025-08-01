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

//! Json Tasks command
//!
//! Wraps invoking of the `v2/schemas/tasks` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::image::v2::schema::tasks::get;
use openstack_types::image::v2::schema::tasks::response::get::TasksResponse;

/// *(Since Images v2.2)*
///
/// Shows a JSON schema document that represents a list of *tasks*.
///
/// An tasks list entity is a container of entities containing abbreviated
/// information about individual tasks.
///
/// The following schema is solely an example. Consider only the response to
/// the API call as authoritative.
///
/// Normal response codes: 200
///
/// Error response codes: 401
#[derive(Args)]
#[command(about = "Show tasks schema")]
pub struct TasksCommand {
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
struct PathParameters {}

impl TasksCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Json Tasks");

        let op = OutputProcessor::from_args(parsed_args, Some("image.schema/tasks"), Some("get"));
        op.validate_args(parsed_args)?;

        let ep_builder = get::Request::builder();

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: serde_json::Value = ep.query_async(client).await?;
        op.output_machine(data)?;
        Ok(())
    }
}
