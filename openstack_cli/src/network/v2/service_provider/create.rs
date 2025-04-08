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

//! Create ServiceProvider command
//!
//! Wraps invoking of the `v2.0/service-providers` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::service_provider::create;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct ServiceProviderCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    service_provider: Vec<(String, Value)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ServiceProvider response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde(rename = "default")]
    #[structable(optional, title = "default")]
    _default: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    service_type: Option<String>,
}

impl ServiceProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ServiceProvider");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.service_provider data

        ep_builder.service_provider(self.service_provider.iter().cloned());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
