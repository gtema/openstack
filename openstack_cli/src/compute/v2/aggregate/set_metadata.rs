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

//! Action Aggregate command
//!
//! Wraps invoking of the `v2.1/os-aggregates/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val_opt;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::aggregate::set_metadata;
use openstack_types::compute::v2::aggregate::response::set_metadata::AggregateResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct AggregateCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    set_metadata: SetMetadata,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-aggregates/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// SetMetadata Body data
#[derive(Args, Clone)]
struct SetMetadata {
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val_opt::<String, String>)]
    metadata: Vec<(String, Option<String>)>,
}

impl AggregateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Aggregate");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_metadata::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.set_metadata data
        let args = &self.set_metadata;
        let mut set_metadata_builder = set_metadata::SetMetadataBuilder::default();

        set_metadata_builder.metadata(
            args.metadata
                .iter()
                .cloned()
                .map(|(k, v)| (k, v.map(Into::into))),
        );

        ep_builder.set_metadata(set_metadata_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<AggregateResponse>(data)?;
        Ok(())
    }
}
