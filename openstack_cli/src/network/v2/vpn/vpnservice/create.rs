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

//! Create Vpnservice command
//!
//! Wraps invoking of the `v2.0/vpn/vpnservices` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::vpn::vpnservice::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Creates a VPN service.
///
/// The service is associated with a router. After you create the service, it
/// can contain multiple VPN connections.
///
/// An optional `flavor_id` attribute can be passed to enable dynamic selection
/// of an appropriate provider if configured by the operator. It is only
/// available when `vpn-flavors` extension is enabled. The basic selection
/// algorithm chooses the provider in the first service profile currently
/// associated with flavor. This option can only be set in `POST` operation.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401
///
#[derive(Args)]
#[command(about = "Create VPN service")]
pub struct VpnserviceCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `vpnservice` object.
    ///
    #[command(flatten)]
    vpnservice: Vpnservice,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Vpnservice Body data
#[derive(Args, Clone)]
struct Vpnservice {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the flavor.
    ///
    #[arg(help_heading = "Body parameters", long)]
    flavor_id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    router_id: Option<String>,

    /// If you specify only a subnet UUID, OpenStack Networking allocates an
    /// available IP from that subnet to the port. If you specify both a subnet
    /// UUID and an IP address, OpenStack Networking tries to allocate the
    /// address to the port.
    ///
    #[arg(help_heading = "Body parameters", long)]
    subnet_id: Option<String>,

    /// The ID of the project.
    ///
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

/// Vpnservice response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Read-only external (public) IPv4 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv4 interface is
    /// available.
    ///
    #[serde()]
    #[structable(optional)]
    external_v4_ip: Option<String>,

    /// Read-only external (public) IPv6 address that is used for the VPN
    /// service. The VPN plugin sets this address if an IPv6 interface is
    /// available.
    ///
    #[serde()]
    #[structable(optional)]
    external_v6_ip: Option<String>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The ID of the VPN service.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    router_id: Option<String>,

    /// Indicates whether IPsec VPN service is currently operational. Values
    /// are `ACTIVE`, `DOWN`, `BUILD`, `ERROR`, `PENDING_CREATE`,
    /// `PENDING_UPDATE`, or `PENDING_DELETE`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// If you specify only a subnet UUID, OpenStack Networking allocates an
    /// available IP from that subnet to the port. If you specify both a subnet
    /// UUID and an IP address, OpenStack Networking tries to allocate the
    /// address to the port.
    ///
    #[serde()]
    #[structable(optional)]
    subnet_id: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,
}

impl VpnserviceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Vpnservice");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.vpnservice data
        let args = &self.vpnservice;
        let mut vpnservice_builder = create::VpnserviceBuilder::default();
        if let Some(val) = &args.tenant_id {
            vpnservice_builder.tenant_id(val);
        }

        if let Some(val) = &args.name {
            vpnservice_builder.name(val);
        }

        if let Some(val) = &args.description {
            vpnservice_builder.description(val);
        }

        if let Some(val) = &args.subnet_id {
            vpnservice_builder.subnet_id(Some(val.into()));
        }

        if let Some(val) = &args.router_id {
            vpnservice_builder.router_id(val);
        }

        if let Some(val) = &args.admin_state_up {
            vpnservice_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.flavor_id {
            vpnservice_builder.flavor_id(Some(val.into()));
        }

        ep_builder.vpnservice(vpnservice_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
