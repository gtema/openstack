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
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::server::shelve_offload;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shelf-offloads, or removes, a shelved server.
///
/// Specify the `shelveOffload` action in the request body.
///
/// Data and resource associations are deleted. If an instance is no longer
/// needed, you can remove that instance from the hypervisor to minimize
/// resource usage.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// **Preconditions**
///
/// The server status must be `SHELVED`.
///
/// If the server is locked, you must have administrator privileges to
/// shelve-offload the server.
///
/// **Asynchronous Postconditions**
///
/// After you successfully shelve-offload a server, its status changes to
/// `SHELVED_OFFLOADED`. The server instance data appears on the compute node.
///
/// **Troubleshooting**
///
/// If the server status does not change to `SHELVED_OFFLOADED`, the
/// shelve-offload operation failed. Ensure that you meet the preconditions and
/// run the request again. If the request fails again, investigate whether
/// another operation is running that causes a race condition.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404),
/// conflict(409)
///
#[derive(Args)]
#[command(about = "Shelf-Offload (Remove) Server (shelveOffload Action)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    shelve_offload: Value,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = shelve_offload::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.shelve_offload data
        ep_builder.shelve_offload(self.shelve_offload.clone());

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
