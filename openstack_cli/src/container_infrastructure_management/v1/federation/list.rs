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

//! List Federations command
//!
//! Wraps invoking of the `v1/federations` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::container_infrastructure_management::v1::federation::list;
use serde_json::Value;
use structable_derive::StructTable;

/// Retrieve a list of federations.
///
/// | | | | --- | --- | | param marker: | pagination marker for large data
/// sets. | | param limit: | maximum number of resources to return in a single
/// result. | | param sort_key: | column to sort results by. Default: id. | |
/// param sort_dir: | direction to sort. "asc" or "desc". Default: asc. |
///
#[derive(Args)]
pub struct FederationsCommand {
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
/// Federations response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    hostcluster_id: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    member_ids: Option<Value>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    properties: Option<Value>,

    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    #[serde()]
    #[structable(optional)]
    status_reason: Option<String>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    uuid: Option<String>,
}

impl FederationsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Federations");

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
