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

//! List Flavors command
//!
//! Wraps invoking of the `v2/lbaas/flavors` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::load_balancer::v2::flavor::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists all flavors.
///
#[derive(Args)]
pub struct FlavorsCommand {
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
struct PathParameters {}
/// Flavors response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<bool>,

    #[serde()]
    #[structable(optional, wide)]
    flavor_profile_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl FlavorsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Flavors");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
