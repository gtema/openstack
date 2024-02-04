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

//! Show Floatingip command
//!
//! Wraps invoking of the `v2.0/floatingips/{id}` with `GET` method

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

use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::floatingip::find;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Shows details for a floating IP.
///
/// Use the `fields` query parameter to control which fields are
/// returned in the response body. For information, see [Filtering and
/// Column Selection](http://specs.openstack.org/openstack/neutron-
/// specs/specs/api/networking_general_api_information.html#filtering-and-
/// column-selection).
///
/// This example request shows details for a floating IP in JSON
/// format. This example also filters the result by the
/// `fixed\_ip\_address` and `floating\_ip\_address` fields.
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403, 404
#[derive(Args)]
#[command(about = "Show floating IP details")]
pub struct FloatingipCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// id parameter for /v2.0/floatingips/{id} API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// Floatingip response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// The ID of the floating IP address.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The floating IP address.
    #[serde()]
    #[structable(optional)]
    floating_ip_address: Option<String>,

    /// The ID of the network associated with the
    /// floating IP.
    #[serde()]
    #[structable(optional)]
    floating_network_id: Option<String>,

    /// The ID of the router for the floating IP.
    #[serde()]
    #[structable(optional)]
    router_id: Option<String>,

    /// The ID of a port associated with the floating IP.
    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    /// The fixed IP address that is associated with the
    /// floating IP address.
    #[serde()]
    #[structable(optional)]
    fixed_ip_address: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The status of the floating IP. Values are
    /// `ACTIVE`, `DOWN` and `ERROR`.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the QoS policy associated with the floating IP.
    #[serde()]
    #[structable(optional)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// A valid DNS name.
    #[serde()]
    #[structable(optional)]
    dns_name: Option<String>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional)]
    dns_domain: Option<String>,

    /// The information of the port that this floating IP associates with.
    /// In particular, if the floating IP is associated with a port, this field
    /// contains some attributes of the associated port, including `name`,
    /// `network\_id`, `mac\_address`, `admin\_state\_up`, `status`,
    /// `device\_id` and `device\_owner`. If the floating IP is not associated
    /// with a port, this field is `null`.
    #[serde()]
    #[structable(optional)]
    port_details: Option<String>,

    /// The associated port forwarding resources for the floating IP. If the
    /// floating IP has multiple port forwarding resources, this field has
    /// multiple entries. Each entry consists of network IP protocol
    /// (`protocol`), the fixed IP address of internal neutron port
    /// (`internal\_ip\_address`), the TCP or UDP port or port range used by
    /// internal neutron port (`internal\_port`) or (`internal\_port\_range`)
    /// and the TCP or UDP port or port range used by floating IP
    /// (`external\_port`) or (`external\_port\_range`).
    #[serde()]
    #[structable(optional)]
    port_forwardings: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,
}
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl FloatingipCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Floatingip");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
