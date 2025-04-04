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

//! Set Floatingip command
//!
//! Wraps invoking of the `v2.0/floatingips/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::floatingip::set;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a floating IP and its association with an internal port.
///
/// The association process is the same as the process for the create floating
/// IP operation.
///
/// To disassociate a floating IP from a port, set the `port_id` attribute to
/// null or omit it from the request body.
///
/// This example updates a floating IP:
///
/// Depending on the request body that you submit, this request associates a
/// port with or disassociates a port from a floating IP.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404, 409, 412
///
#[derive(Args)]
#[command(about = "Update floating IP")]
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
    ///
    #[command(flatten)]
    floatingip: Floatingip,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/floatingips/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Floatingip Body data
#[derive(Args, Clone)]
struct Floatingip {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The fixed IP address that is associated with the floating IP. If an
    /// internal port has multiple associated IP addresses, the service chooses
    /// the first IP address unless you explicitly define a fixed IP address in
    /// the `fixed_ip_address` parameter.
    ///
    #[arg(help_heading = "Body parameters", long)]
    fixed_ip_address: Option<String>,

    /// The ID of a port associated with the floating IP. To associate the
    /// floating IP with a fixed IP, you must specify the ID of the internal
    /// port. To disassociate the floating IP, `null` should be specified.
    ///
    #[arg(help_heading = "Body parameters", long)]
    port_id: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    qos_policy_id: Option<String>,
}

/// Floatingip response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// A valid DNS domain.
    ///
    #[serde()]
    #[structable(optional)]
    dns_domain: Option<String>,

    /// A valid DNS name.
    ///
    #[serde()]
    #[structable(optional)]
    dns_name: Option<String>,

    /// The fixed IP address that is associated with the floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    fixed_ip_address: Option<String>,

    /// The floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional)]
    floating_network_id: Option<String>,

    /// The ID of the floating IP address.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The information of the port that this floating IP associates with. In
    /// particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network_id`, `mac_address`, `admin_state_up`, `status`, `device_id`
    /// and `device_owner`. If the floating IP is not associated with a port,
    /// this field is `null`.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    port_details: Option<Value>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal_ip_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal_port`) or (`internal_port_range`) and
    /// the TCP or UDP port or port range used by floating IP (`external_port`)
    /// or (`external_port_range`).
    ///
    #[serde()]
    #[structable(optional, pretty)]
    port_forwardings: Option<Value>,

    /// The ID of a port associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    ///
    #[serde()]
    #[structable(optional)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The ID of the router for the floating IP.
    ///
    #[serde()]
    #[structable(optional)]
    router_id: Option<String>,

    /// The status of the floating IP. Values are `ACTIVE`, `DOWN` and `ERROR`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl FloatingipCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Floatingip");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.floatingip data
        let args = &self.floatingip;
        let mut floatingip_builder = set::FloatingipBuilder::default();
        if let Some(val) = &args.port_id {
            floatingip_builder.port_id(Some(val.into()));
        }

        if let Some(val) = &args.fixed_ip_address {
            floatingip_builder.fixed_ip_address(val);
        }

        if let Some(val) = &args.qos_policy_id {
            floatingip_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.description {
            floatingip_builder.description(val);
        }

        ep_builder.floatingip(floatingip_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
