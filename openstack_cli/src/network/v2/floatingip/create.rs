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

//! Create Floatingip command
//!
//! Wraps invoking of the `v2.0/floatingips` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::floatingip::create;
use openstack_types::network::v2::floatingip::response::create::FloatingipResponse;

/// Creates a floating IP, and, if you specify port information, associates the
/// floating IP with an internal port.
///
/// To associate the floating IP with an internal port, specify the port ID
/// attribute in the request body. If you do not specify a port ID in the
/// request, you can issue a PUT request instead of a POST request.
///
/// Default policy settings enable only administrative users to set floating IP
/// addresses and some non-administrative users might require a floating IP
/// address. If you do not specify a floating IP address in the request, the
/// operation automatically allocates one.
///
/// By default, this operation associates the floating IP address with a single
/// fixed IP address that is configured on an OpenStack Networking port. If a
/// port has multiple IP addresses, you must specify the `fixed_ip_address`
/// attribute in the request body to associate a fixed IP address with the
/// floating IP address.
///
/// You can create floating IPs on only external networks. When you create a
/// floating IP, you must specify the ID of the network on which you want to
/// create the floating IP. Alternatively, you can create a floating IP on a
/// subnet in the external network, based on the costs and quality of that
/// subnet.
///
/// You must configure an IP address with the internal OpenStack Networking
/// port that is associated with the floating IP address.
///
/// The operation returns the `Bad Request (400)` response code for one of
/// reasons:
///
/// If the port ID is not valid, this operation returns `404` response code.
///
/// The operation returns the `Conflict (409)` response code for one of
/// reasons:
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 404, 409
#[derive(Args)]
#[command(about = "Create floating IP")]
pub struct FloatingipCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `floatingip` object. When you associate a floating IP address with a
    /// VM, the instance has the same public IP address each time that it
    /// boots, basically to maintain a consistent IP address for maintaining
    /// DNS assignment.
    #[command(flatten)]
    floatingip: Floatingip,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Floatingip Body data
#[derive(Args, Clone)]
struct Floatingip {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// A valid DNS domain.
    #[arg(help_heading = "Body parameters", long)]
    dns_domain: Option<String>,

    /// A valid DNS name.
    #[arg(help_heading = "Body parameters", long)]
    dns_name: Option<String>,

    /// The fixed IP address that is associated with the floating IP. If an
    /// internal port has multiple associated IP addresses, the service chooses
    /// the first IP address unless you explicitly define a fixed IP address in
    /// the `fixed_ip_address` parameter.
    #[arg(help_heading = "Body parameters", long)]
    fixed_ip_address: Option<String>,

    /// The floating IP address.
    #[arg(help_heading = "Body parameters", long)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the floating IP.
    #[arg(help_heading = "Body parameters", long)]
    floating_network_id: String,

    /// The ID of a port associated with the floating IP. To associate the
    /// floating IP with a fixed IP at creation time, you must specify the
    /// identifier of the internal port.
    #[arg(help_heading = "Body parameters", long)]
    port_id: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    #[arg(help_heading = "Body parameters", long)]
    qos_policy_id: Option<String>,

    /// The subnet ID on which you want to create the floating IP.
    #[arg(help_heading = "Body parameters", long)]
    subnet_id: Option<String>,

    /// The ID of the project.
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

impl FloatingipCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Floatingip");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.floatingip data
        let args = &self.floatingip;
        let mut floatingip_builder = create::FloatingipBuilder::default();
        if let Some(val) = &args.floating_ip_address {
            floatingip_builder.floating_ip_address(val);
        }

        if let Some(val) = &args.subnet_id {
            floatingip_builder.subnet_id(Some(val.into()));
        }

        floatingip_builder.floating_network_id(&args.floating_network_id);

        if let Some(val) = &args.port_id {
            floatingip_builder.port_id(Some(val.into()));
        }

        if let Some(val) = &args.fixed_ip_address {
            floatingip_builder.fixed_ip_address(val);
        }

        if let Some(val) = &args.tenant_id {
            floatingip_builder.tenant_id(val);
        }

        if let Some(val) = &args.qos_policy_id {
            floatingip_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.dns_name {
            floatingip_builder.dns_name(val);
        }

        if let Some(val) = &args.dns_domain {
            floatingip_builder.dns_domain(val);
        }

        if let Some(val) = &args.description {
            floatingip_builder.description(val);
        }

        ep_builder.floatingip(floatingip_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<FloatingipResponse>(data)?;
        Ok(())
    }
}
