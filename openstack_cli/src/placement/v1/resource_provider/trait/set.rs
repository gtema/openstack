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

//! Set Trait command
//!
//! Wraps invoking of the `resource_providers/{uuid}/traits` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::resource_provider::r#trait::set;
use openstack_sdk::api::QueryAsync;

/// Associate traits with the resource provider identified by {uuid}. All the
/// associated traits will be replaced by the traits specified in the request
/// body.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400), itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Update resource provider traits")]
pub struct TraitCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A list of traits.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    traits: Vec<String>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// uuid parameter for /resource_providers/{uuid}/traits API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_uuid",
        value_name = "UUID"
    )]
    uuid: String,
}
/// Trait response representation
#[derive(Deserialize, Serialize, Clone)]
struct ResponseData(String);

impl StructTable for ResponseData {
    fn build(&self, _: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Value".to_string()]);
        let res: Vec<Vec<String>> = Vec::from([Vec::from([self.0.to_string()])]);
        (headers, res)
    }
}

impl StructTable for Vec<ResponseData> {
    fn build(&self, _: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Values".to_string()]);
        let res: Vec<Vec<String>> = Vec::from([Vec::from([self
            .iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")])]);
        (headers, res)
    }
}

impl TraitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Trait");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.uuid(&self.path.uuid);
        // Set query parameters
        // Set body parameters
        // Set Request.traits data

        ep_builder.traits(self.traits.iter().map(Into::into).collect::<Vec<_>>());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}