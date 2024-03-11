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

//! Create ExtraSpec command
//!
//! Wraps invoking of the `v3/types/{type_id}/extra_specs` with `POST` method

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

use crate::common::parse_key_val_opt;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::r#type::extra_spec::create;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct ExtraSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val_opt::<String, String>)]
    extra_specs: Vec<(String, Option<String>)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// type_id parameter for /v3/types/{type_id}/encryption/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_type_id",
        value_name = "TYPE_ID"
    )]
    type_id: String,
}
/// ExtraSpec response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ExtraSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ExtraSpec");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.type_id(&self.path.type_id);
        // Set query parameters
        // Set body parameters
        // Set Request.extra_specs data
        let args = &self.extra_specs;

        ep_builder.extra_specs(args.iter().cloned().map(|(k, v)| (k, v.map(Into::into))));

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
