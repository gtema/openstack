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

//! Create Metadata command
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/metadata` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::metadata::create;
use std::collections::HashMap;

/// Create or update one or more metadata items for a server.
///
/// Creates any metadata items that do not already exist in the server,
/// replaces exists metadata items that match keys. Does not modify items that
/// are not in the request.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Create or Update Metadata Items")]
pub struct MetadataCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Metadata key and value pairs. The maximum size for each metadata key
    /// and value pair is 255 bytes.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Vec<(String, String)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/metadata/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}
/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
struct ResponseData(HashMap<String, String>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(
            self.0
                .iter()
                .map(|(k, v)| Vec::from([k.clone(), v.clone()])),
        );
        (headers, rows)
    }
}

impl MetadataCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Metadata");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.metadata data

        ep_builder.metadata(self.metadata.iter().cloned());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
