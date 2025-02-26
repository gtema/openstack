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

//! Action Server command [microversion = 2.1]
//!
//! Wraps invoking of the `v2.1/servers/{id}/action` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::os_get_serial_console_21;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Gets a serial console for a server.
///
/// Specify the `os-getSerialConsole` action in the request body.
///
/// The only supported connection type is `serial`. The `type` parameter should
/// be set as `serial`.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409), notImplemented(501)
///
#[derive(Args)]
#[command(
    about = "Get Serial Console (os-getSerialConsole Action) (DEPRECATED) (microversion = 2.1)"
)]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The action.
    ///
    #[command(flatten)]
    os_get_serial_console: OsGetSerialConsole,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Serial,
}

/// OsGetSerialConsole Body data
#[derive(Args, Clone)]
struct OsGetSerialConsole {
    /// The type of serial console. The only valid value is `serial`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    _type: Type,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The type of the remote console
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The URL used to connect to the console.
    ///
    #[serde()]
    #[structable(optional)]
    url: Option<String>,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_get_serial_console_21::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.1");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_get_serial_console data
        let args = &self.os_get_serial_console;
        let mut os_get_serial_console_builder =
            os_get_serial_console_21::OsGetSerialConsoleBuilder::default();

        let tmp = match &args._type {
            Type::Serial => os_get_serial_console_21::Type::Serial,
        };
        os_get_serial_console_builder._type(tmp);

        ep_builder.os_get_serial_console(os_get_serial_console_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
