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

//! Action Migration command [microversion = 2.22]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/migrations/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::migration::force_complete_222;

/// Force an in-progress live migration for a given server to complete.
///
/// Specify the `force_complete` action in the request body.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// **Preconditions**
///
/// The server OS-EXT-STS:vm_state value must be `active` and the server
/// OS-EXT-STS:task_state value must be `migrating`.
///
/// If the server is locked, you must have administrator privileges to force
/// the completion of the server migration.
///
/// The migration status must be `running`.
///
/// **Asynchronous Postconditions**
///
/// After you make this request, you typically must keep polling the server
/// status to determine whether the request succeeded.
///
/// **Troubleshooting**
///
/// If the server status remains `MIGRATING` for an inordinate amount of time,
/// the request may have failed. Ensure you meet the preconditions and run the
/// request again. If the request fails again, investigate the compute back
/// end. More details can be found in the
/// [admin guide](https://docs.openstack.org/nova/latest/admin/live-migration-usage.html#what-to-do-when-the-migration-times-out).
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
#[derive(Args)]
#[command(about = "Force Migration Complete Action (force_complete Action) (microversion = 2.22)")]
pub struct MigrationCommand {
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
    /// id parameter for /v2.1/servers/{server_id}/migrations/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,

    /// server_id parameter for
    /// /v2.1/servers/{server_id}/migrations/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}

impl MigrationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Migration");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.server/migration"),
            Some("force_complete"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = force_complete_222::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("compute 2.22"),
        );

        ep_builder.id(&self.path.id);
        ep_builder.server_id(&self.path.server_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
