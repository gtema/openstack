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

//! Action Server command
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::os_stop;
use serde_json::Value;

/// Stops a running server and changes its status to `SHUTOFF`.
///
/// Specify the `os-stop` action in the request body.
///
/// **Preconditions**
///
/// The server status must be `ACTIVE` or `ERROR`.
///
/// If the server is locked, you must have administrator privileges to stop the
/// server.
///
/// **Asynchronous Postconditions**
///
/// After you successfully stop a server, its status changes to `SHUTOFF`. This
/// API operation does not delete the server instance data and the data will be
/// available again after `os-start` action.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
#[derive(Args)]
#[command(about = "Stop Server (os-stop Action)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    os_stop: Value,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args, Some("compute.server"), Some("os_stop"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_stop::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.os_stop data
        ep_builder.os_stop(self.os_stop.clone());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
