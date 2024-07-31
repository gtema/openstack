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

//! Delete AssistedVolumeSnapshot command
//!
//! Wraps invoking of the `v2.1/os-assisted-volume-snapshots/{id}` with `DELETE` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::compute::v2::assisted_volume_snapshot::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Deletes an assisted volume snapshot.
///
/// To make this request, add the `delete_info` query parameter to the URI, as
/// follows:
///
/// DELETE
/// /os-assisted-volume-snapshots/421752a6-acf6-4b2d-bc7a-119f9148cd8c?delete_info=’{“volume_id”:
/// “521752a6-acf6-4b2d-bc7a-119f9148cd8c”}’
///
/// Normal response codes: 204
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
///
#[derive(Args)]
#[command(about = "Delete Assisted Volume Snapshot")]
pub struct AssistedVolumeSnapshotCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    delete_info: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-assisted-volume-snapshots/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// AssistedVolumeSnapshot response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl AssistedVolumeSnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete AssistedVolumeSnapshot");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        if let Some(val) = &self.query.delete_info {
            ep_builder.delete_info(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
