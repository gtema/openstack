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
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::placement::v1::resource_class::set_17;
use openstack_types::placement::v1::resource_class::response::set::ResourceClassResponse;

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
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_name",
        value_name = "NAME"
    )]
    name: String,
}

impl ResourceClassCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set ResourceClass");

        let op =
            OutputProcessor::from_args(parsed_args, Some("placement.resource_class"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set_17::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("placement 1.7"),
        );

        ep_builder.name(&self.path.name);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<ResourceClassResponse>(data)?;
        Ok(())
    }
}
