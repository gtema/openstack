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

//! Action Router command
//!
//! Wraps invoking of the `v2.0/routers/{id}/add_external_gateways` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::router::add_external_gateways;
use openstack_types::network::v2::router::response::add_external_gateways::RouterResponse;
use serde_json::Value;

/// Request body
#[derive(Args)]
#[command(about = "Add external gateways to router")]
pub struct RouterCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    router: Router,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Router Body data
#[derive(Args, Clone)]
struct Router {
    /// The list of external gateways of the router.
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    external_gateways: Option<Vec<Value>>,
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Router");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = add_external_gateways::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.router data
        let args = &self.router;
        let mut router_builder = add_external_gateways::RouterBuilder::default();
        if let Some(val) = &args.external_gateways {
            let external_gateways_builder: Vec<add_external_gateways::ExternalGateways> = val
                .iter()
                .flat_map(|v| {
                    serde_json::from_value::<add_external_gateways::ExternalGateways>(v.to_owned())
                })
                .collect::<Vec<add_external_gateways::ExternalGateways>>();
            router_builder.external_gateways(external_gateways_builder);
        }

        ep_builder.router(router_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<RouterResponse>(data)?;
        Ok(())
    }
}
