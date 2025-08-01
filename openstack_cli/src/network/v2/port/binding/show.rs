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

//! Show Binding command
//!
//! Wraps invoking of the `v2.0/ports/{port_id}/bindings/{id}` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::port::binding::get;
use openstack_types::network::v2::port::binding::response::get::BindingResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct BindingCommand {
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
    /// id parameter for /v2.0/ports/{port_id}/bindings/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,

    /// port_id parameter for /v2.0/ports/{port_id}/bindings/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_port_id",
        value_name = "PORT_ID"
    )]
    port_id: String,
}

impl BindingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Binding");

        let op =
            OutputProcessor::from_args(parsed_args, Some("network.port/binding"), Some("show"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        ep_builder.id(&self.path.id);
        ep_builder.port_id(&self.path.port_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<BindingResponse>(data)?;
        Ok(())
    }
}
