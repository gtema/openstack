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

//! Create GroupSpec command [microversion = 3.11]
//!
//! Wraps invoking of the `v3/group_types/{group_type_id}/group_specs` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val_opt;
use openstack_sdk::api::block_storage::v3::group_type::group_spec::create_311;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct GroupSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A set of key and value pairs that contains the specifications for a
    /// group type.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val_opt::<String, String>)]
    group_specs: Vec<(String, Option<String>)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// group_type_id parameter for
    /// /v3/group_types/{group_type_id}/group_specs/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group_type_id",
        value_name = "GROUP_TYPE_ID"
    )]
    group_type_id: String,
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

impl GroupSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create GroupSpec");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_311::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.11");

        // Set path parameters
        ep_builder.group_type_id(&self.path.group_type_id);
        // Set query parameters
        // Set body parameters
        // Set Request.group_specs data

        ep_builder.group_specs(
            self.group_specs
                .iter()
                .cloned()
                .map(|(k, v)| (k, v.map(Into::into))),
        );

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
