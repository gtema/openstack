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

//! Create Mapping command
//!
//! Wraps invoking of the `v3/OS-FEDERATION/mappings/{mapping_id}` with `PUT` method

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

use crate::common::parse_json;
use openstack_sdk::api::identity::v3::os_federation::mapping::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Create a mapping.
///
/// PUT /OS-FEDERATION/mappings/{mapping_id}
///
#[derive(Args)]
pub struct MappingCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    mapping: Mapping,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// mapping_id parameter for /v3/OS-FEDERATION/mappings/{mapping_id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Mapping Body data
#[derive(Args)]
struct Mapping {
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    rules: Vec<Value>,
}

/// Mapping response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The Federation Mapping unique ID
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    rules: Option<Value>,
}

impl MappingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Mapping");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.mapping data
        let args = &self.mapping;
        let mut mapping_builder = create::MappingBuilder::default();

        let rules_builder: Vec<create::Rules> = args
            .rules
            .iter()
            .flat_map(|v| serde_json::from_value::<create::Rules>(v.to_owned()))
            .collect::<Vec<create::Rules>>();
        mapping_builder.rules(rules_builder);

        ep_builder.mapping(mapping_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
