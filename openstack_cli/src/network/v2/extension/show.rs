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

//! Show Extension command
//!
//! Wraps invoking of the `v2.0/extensions/{id}` with `GET` method

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

use openstack_sdk::api::network::v2::extension::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows details for an extension, by alias.
/// The response shows the extension name and its alias. To show
/// details for an extension, you specify the alias.
///
/// Normal response codes: 200
///
/// Error response codes: 401, 404
#[derive(Args)]
#[command(about = "Show extension details")]
pub struct ExtensionCommand {
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
    /// id parameter for /v2.0/extensions/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Extension response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The alias for the extension. For example “quotas” or
    /// “security-group”.
    #[serde()]
    #[structable(optional)]
    alias: Option<String>,

    /// The human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A URL pointing to the namespace for this extension.
    #[serde()]
    #[structable(optional)]
    namespace: Option<String>,

    /// The date and timestamp when the extension was
    /// last updated.
    #[serde()]
    #[structable(optional)]
    updated: Option<String>,
}

impl ExtensionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Extension");

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

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
