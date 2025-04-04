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

//! Get Version command
//!
//! Wraps invoking of the `v1` with `GET` method

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
use openstack_sdk::api::container_infrastructure_management::v1::version::get;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct VersionCommand {
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
/// Version response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, pretty)]
    certificates: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    clusters: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    clustertemplates: Option<Value>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    federations: Option<Value>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    media_types: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    mservices: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    nodegroups: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    quotas: Option<Value>,

    #[serde()]
    #[structable(optional, pretty)]
    stats: Option<Value>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl VersionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Version");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = get::Request::builder();

        // Set path parameters
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
