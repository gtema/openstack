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

//! Set PortForwarding command
//!
//! Wraps invoking of the `v2.0/floatingips/{floatingip_id}/port_forwardings/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::floatingip::port_forwarding::set;
use structable_derive::StructTable;

/// Updates a floating IP port forwarding.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 404
///
#[derive(Args)]
#[command(about = "Update a port forwarding")]
pub struct PortForwardingCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `floating IP port forwarding` object.
    ///
    #[command(flatten)]
    port_forwarding: PortForwarding,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// floatingip_id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_floatingip_id",
        value_name = "FLOATINGIP_ID"
    )]
    floatingip_id: String,

    /// id parameter for
    /// /v2.0/floatingips/{floatingip_id}/port_forwardings/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Dccp,
    Icmp,
    Ipv6Icmp,
    Sctp,
    Tcp,
    Udp,
}

/// PortForwarding Body data
#[derive(Args, Clone)]
struct PortForwarding {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    ///
    #[arg(help_heading = "Body parameters", long)]
    external_port: Option<Option<f32>>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    ///
    #[arg(help_heading = "Body parameters", long)]
    external_port_range: Option<f32>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    ///
    #[arg(help_heading = "Body parameters", long)]
    internal_ip_address: Option<String>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[arg(help_heading = "Body parameters", long)]
    internal_port: Option<Option<f32>>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    ///
    #[arg(help_heading = "Body parameters", long)]
    internal_port_id: Option<String>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[arg(help_heading = "Body parameters", long)]
    internal_port_range: Option<f32>,

    /// The IP protocol used in the floating IP port forwarding.
    ///
    #[arg(help_heading = "Body parameters", long)]
    protocol: Option<Protocol>,
}

/// PortForwarding response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A text describing the rule, which helps users to manage/find easily
    /// theirs rules.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The TCP/UDP/other protocol port number of the port forwarding’s
    /// floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    external_port: Option<f32>,

    /// The TCP/UDP/other protocol port range of the port forwarding’s floating
    /// IP address.
    ///
    #[serde()]
    #[structable(optional)]
    external_port_range: Option<f32>,

    /// The ID of the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The fixed IPv4 address of the Neutron port associated to the floating
    /// IP port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    internal_ip_address: Option<String>,

    /// The TCP/UDP/other protocol port number of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    internal_port: Option<f32>,

    /// The ID of the Neutron port associated to the floating IP port
    /// forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    internal_port_id: Option<String>,

    /// The TCP/UDP/other protocol port range of the Neutron port fixed IP
    /// address associated to the floating ip port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    internal_port_range: Option<f32>,

    /// The IP protocol used in the floating IP port forwarding.
    ///
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,
}

impl PortForwardingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set PortForwarding");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.floatingip_id(&self.path.floatingip_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.port_forwarding data
        let args = &self.port_forwarding;
        let mut port_forwarding_builder = set::PortForwardingBuilder::default();
        if let Some(val) = &args.external_port {
            port_forwarding_builder.external_port(*val);
        }

        if let Some(val) = &args.internal_port {
            port_forwarding_builder.internal_port(*val);
        }

        if let Some(val) = &args.internal_ip_address {
            port_forwarding_builder.internal_ip_address(val);
        }

        if let Some(val) = &args.protocol {
            let tmp = match val {
                Protocol::Dccp => set::Protocol::Dccp,
                Protocol::Icmp => set::Protocol::Icmp,
                Protocol::Ipv6Icmp => set::Protocol::Ipv6Icmp,
                Protocol::Sctp => set::Protocol::Sctp,
                Protocol::Tcp => set::Protocol::Tcp,
                Protocol::Udp => set::Protocol::Udp,
            };
            port_forwarding_builder.protocol(tmp);
        }

        if let Some(val) = &args.internal_port_id {
            port_forwarding_builder.internal_port_id(val);
        }

        if let Some(val) = &args.description {
            port_forwarding_builder.description(val);
        }

        if let Some(val) = &args.external_port_range {
            port_forwarding_builder.external_port_range(*val);
        }

        if let Some(val) = &args.internal_port_range {
            port_forwarding_builder.internal_port_range(*val);
        }

        ep_builder.port_forwarding(port_forwarding_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
