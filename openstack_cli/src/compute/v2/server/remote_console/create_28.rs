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

//! Create RemoteConsole command [microversion = 2.8]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/remote-consoles` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::remote_console::create_28;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// The API provides a unified request for creating a remote console. The user
/// can
/// get a URL to connect the console from this API. The URL includes the token
/// which is used to get permission to access the console. Servers may support
/// different console protocols. To return a remote console using a specific
/// protocol, such as RDP, set the `protocol` parameter to `rdp`.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404),
/// conflict(409), notImplemented(501)
#[derive(Args)]
#[command(about = "Create Console (microversion = 2.8)")]
pub struct RemoteConsoleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    remote_console: RemoteConsole,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Mks,
    Rdp,
    Serial,
    Spice,
    Vnc,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Novnc,
    RdpHtml5,
    Serial,
    SpiceHtml5,
    Webmks,
    Xvpvnc,
}

/// RemoteConsole Body data
#[derive(Args)]
struct RemoteConsole {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `rdp`, `serial` and `mks`. The protocol `mks` is added since
    /// Microversion `2.8`.
    #[arg(long)]
    protocol: Protocol,

    /// The type of remote console. The valid values are `novnc`,
    /// `rdp-html5`, `spice-html5`, `serial`, and `webmks`. The type
    /// `webmks` is added since Microversion `2.8`.
    #[arg(long)]
    _type: Type,
}

/// RemoteConsole response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `rdp`, `serial` and `mks`. The protocol `mks` is added since
    /// Microversion `2.8`.
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The type of remote console. The valid values are `novnc`,
    /// `rdp-html5`, `spice-html5`, `serial`, and `webmks`. The type
    /// `webmks` is added since Microversion `2.8`.
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The URL is used to connect the console.
    #[serde()]
    #[structable(optional)]
    url: Option<String>,
}

impl RemoteConsoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create RemoteConsole");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_28::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.8");

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.remote_console data
        let args = &self.remote_console;
        let mut remote_console_builder = create_28::RemoteConsoleBuilder::default();

        let tmp = match &args.protocol {
            Protocol::Mks => create_28::Protocol::Mks,
            Protocol::Rdp => create_28::Protocol::Rdp,
            Protocol::Serial => create_28::Protocol::Serial,
            Protocol::Spice => create_28::Protocol::Spice,
            Protocol::Vnc => create_28::Protocol::Vnc,
        };
        remote_console_builder.protocol(tmp);

        let tmp = match &args._type {
            Type::Novnc => create_28::Type::Novnc,
            Type::RdpHtml5 => create_28::Type::RdpHtml5,
            Type::Serial => create_28::Type::Serial,
            Type::SpiceHtml5 => create_28::Type::SpiceHtml5,
            Type::Webmks => create_28::Type::Webmks,
            Type::Xvpvnc => create_28::Type::Xvpvnc,
        };
        remote_console_builder._type(tmp);

        ep_builder.remote_console(remote_console_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
