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

//! Show Ip command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/ips/{id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server::ip::get;
use openstack_sdk::api::QueryAsync;

use structable_derive::StructTable;

/// Shows IP addresses details for a network label of a server instance.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of
/// the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "Show Ip Details")]
pub struct IpCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,

    /// id parameter for /v2.1/servers/{server_id}/ips/{id} API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// Ip response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The IP address.
    #[serde()]
    #[structable(optional)]
    addr: Option<String>,

    /// The IP version of the address associated with server.
    #[serde()]
    #[structable(optional)]
    version: Option<i32>,
}

impl IpCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Ip");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        ep_builder.id(&self.path.id);
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
