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
//! Wraps invoking of the `v3/volumes/{volume_id}/metadata` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::volume::metadata::create;
use openstack_types::block_storage::v3::volume::metadata::response::create::MetadataResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct MetadataCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// One or more metadata key and value pairs that are associated with the
    /// volume.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Vec<(String, String)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// volume_id parameter for /v3/volumes/{volume_id}/metadata API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_volume_id",
        value_name = "VOLUME_ID"
    )]
    volume_id: String,
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
        ep_builder.volume_id(&self.path.volume_id);
        // Set query parameters
        // Set body parameters
        // Set Request.metadata data

        ep_builder.metadata(self.metadata.iter().cloned());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<MetadataResponse>(data)?;
        Ok(())
    }
}
