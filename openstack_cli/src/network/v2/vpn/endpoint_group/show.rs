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

//! Show EndpointGroup command
//!
//! Wraps invoking of the `v2.0/vpn/endpoint-groups/{id}` with `GET` method

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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::vpn::endpoint_group::find;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows details for a VPN endpoint group.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403, 404
///
#[derive(Args)]
#[command(about = "Show VPN endpoint group")]
pub struct EndpointGroupCommand {
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
    /// id parameter for /v2.0/vpn/endpoint-groups/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// EndpointGroup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// List of endpoints of the same type, for the endpoint group. The values
    /// will depend on type.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    endpoints: Option<Value>,

    /// The ID of the VPN endpoint group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The type of the endpoints in the group. A valid value is `subnet`,
    /// `cidr`, `network`, `router`, or `vlan`. Only `subnet` and `cidr` are
    /// supported at this moment.
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,
}

impl EndpointGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show EndpointGroup");

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
