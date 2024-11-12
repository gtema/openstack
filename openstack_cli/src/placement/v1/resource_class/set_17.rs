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

//! Set ResourceClass command [microversion = 1.7]
//!
//! Wraps invoking of the `resource_classes/{name}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::resource_class::set_17;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Create or validate the existence of single resource class identified by
/// {name}.
///
/// Normal Response Codes: 201, 204
///
/// A 201 Created response code will be returned if the new resource class is
/// successfully created. A 204 No Content response code will be returned if
/// the resource class already exists.
///
/// Error response codes: badRequest(400)
///
#[derive(Args)]
#[command(about = "Update resource class (microversion = 1.7)")]
pub struct ResourceClassCommand {
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
    /// name parameter for /resource_classes/{name} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_name",
        value_name = "NAME"
    )]
    name: String,
}
/// ResourceClass response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A list of links associated with one resource class.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// The name of one resource class.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl ResourceClassCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ResourceClass");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_17::Request::builder();
        ep_builder.header("OpenStack-API-Version", "placement 1.7");

        // Set path parameters
        ep_builder.name(&self.path.name);
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