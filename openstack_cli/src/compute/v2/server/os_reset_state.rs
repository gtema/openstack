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

//! Action Server command
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server::os_reset_state;

/// Resets the state of a server.
///
/// Specify the `os-resetState` action and the `state` in the request body.
///
/// Policy defaults enable only users with the administrative role to perform
/// this operation. Cloud providers can change these permissions through the
/// `policy.json` file.
///
/// Normal response codes: 202
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "Reset Server State (os-resetState Action)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action.
    #[command(flatten)]
    os_reset_state: OsResetState,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum State {
    Active,
    Error,
}

/// OsResetState Body data
#[derive(Args, Clone)]
struct OsResetState {
    /// The state of the server to be set, `active` or `error` are valid.
    #[arg(help_heading = "Body parameters", long)]
    state: State,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op =
            OutputProcessor::from_args(parsed_args, Some("compute.server"), Some("os_reset_state"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_reset_state::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.os_reset_state data
        let args = &self.os_reset_state;
        let mut os_reset_state_builder = os_reset_state::OsResetStateBuilder::default();

        let tmp = match &args.state {
            State::Active => os_reset_state::State::Active,
            State::Error => os_reset_state::State::Error,
        };
        os_reset_state_builder.state(tmp);

        ep_builder.os_reset_state(os_reset_state_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
