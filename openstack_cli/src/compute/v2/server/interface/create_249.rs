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

//! Create Interface command [microversion = 2.49]
//!
//! Wraps invoking of the `v2.1/servers/{server_id}/os-interface` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server::interface::create_249;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a port interface and uses it to attach a port to a server.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409), computeFault(500), NotImplemented(501)
///
#[derive(Args)]
#[command(about = "Create Interface (microversion = 2.49)")]
pub struct InterfaceCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Specify the `interfaceAttachment` action in the request body.
    ///
    #[command(flatten)]
    interface_attachment: InterfaceAttachment,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/os-interface/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_server_id",
        value_name = "SERVER_ID"
    )]
    server_id: String,
}
/// InterfaceAttachment Body data
#[derive(Args, Clone)]
struct InterfaceAttachment {
    /// Fixed IP addresses. If you request a specific fixed IP address without
    /// a `net_id`, the request returns a `Bad Request (400)` response code.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    fixed_ips: Option<Vec<String>>,

    /// The ID of the network for which you want to create a port interface.
    /// The `net_id` and `port_id` parameters are mutually exclusive. If you do
    /// not specify the `net_id` parameter, the OpenStack Networking API v2.0
    /// uses the network information cache that is associated with the
    /// instance.
    ///
    #[arg(help_heading = "Body parameters", long)]
    net_id: Option<String>,

    /// The ID of the port for which you want to create an interface. The
    /// `net_id` and `port_id` parameters are mutually exclusive. If you do not
    /// specify the `port_id` parameter, the OpenStack Networking API v2.0
    /// allocates a port and creates an interface for it on the network.
    ///
    #[arg(help_heading = "Body parameters", long)]
    port_id: Option<String>,

    /// A device role tag that can be applied to a network interface when
    /// attaching it to the VM. The guest OS of a server that has devices
    /// tagged in this manner can access hardware metadata about the tagged
    /// devices from the metadata API and on the config drive, if enabled.
    ///
    /// **New in version 2.49**
    ///
    #[arg(help_heading = "Body parameters", long)]
    tag: Option<String>,
}

/// Interface response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Fixed IP addresses with subnet IDs.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    fixed_ips: Option<Value>,

    /// The MAC address.
    ///
    #[serde()]
    #[structable(optional)]
    mac_addr: Option<String>,

    /// The network ID.
    ///
    #[serde()]
    #[structable(optional)]
    net_id: Option<String>,

    /// The port ID.
    ///
    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    /// The port state.
    ///
    #[serde()]
    #[structable(optional)]
    port_state: Option<String>,

    /// The device tag applied to the virtual network interface or `null`.
    ///
    /// **New in version 2.70**
    ///
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,
}

impl InterfaceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Interface");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_249::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.49");

        // Set path parameters
        ep_builder.server_id(&self.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.interface_attachment data
        let args = &self.interface_attachment;
        let mut interface_attachment_builder = create_249::InterfaceAttachmentBuilder::default();
        if let Some(val) = &args.net_id {
            interface_attachment_builder.net_id(val);
        }

        if let Some(val) = &args.port_id {
            interface_attachment_builder.port_id(val);
        }

        if let Some(val) = &args.fixed_ips {
            let fixed_ips_builder: Vec<create_249::FixedIps> = val
                .iter()
                .flat_map(|v| create_249::FixedIpsBuilder::default().ip_address(v).build())
                .collect();
            interface_attachment_builder.fixed_ips(fixed_ips_builder);
        }

        if let Some(val) = &args.tag {
            interface_attachment_builder.tag(val);
        }

        ep_builder.interface_attachment(interface_attachment_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
