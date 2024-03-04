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

//! Get Uptime command
//!
//! Wraps invoking of the `v2.1/os-hypervisors/{id}/uptime` with `GET` method

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

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::hypervisor::uptime::get;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Shows the uptime for a given hypervisor.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), NotImplemented(501)
///
#[derive(Args)]
#[command(about = "Show Hypervisor Uptime (DEPRECATED)")]
pub struct UptimeCommand {
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
    /// id parameter for /v2.1/os-hypervisors/{id}/uptime API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Uptime response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl UptimeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Uptime");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

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
