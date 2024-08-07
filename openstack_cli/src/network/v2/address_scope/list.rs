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

//! List AddressScopes command
//!
//! Wraps invoking of the `v2.0/address-scopes` with `GET` method

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
use openstack_sdk::api::network::v2::address_scope::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists address scopes that the project has access to.
///
/// Default policy settings return only the address scopes owned by the project
/// of the user submitting the request, unless the user has administrative
/// role.
///
/// Standard query parameters are supported on the URI. For more information,
/// see [Filtering and Column Selection](#filtering).
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Pagination query parameters are supported if Neutron configuration supports
/// it by overriding `allow_pagination=false`. For more information, see
/// [Pagination](#pagination).
///
/// Sorting query parameters are supported if Neutron configuration supports it
/// with `allow_sorting=true`. For more information, see [Sorting](#sorting).
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List address scopes")]
pub struct AddressScopesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// id query parameter for /v2.0/address-scopes API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// ip_version query parameter for /v2.0/address-scopes API
    ///
    #[arg(help_heading = "Query parameters", long)]
    ip_version: Option<i32>,

    /// name query parameter for /v2.0/address-scopes API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// shared query parameter for /v2.0/address-scopes API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    shared: Option<bool>,

    /// tenant_id query parameter for /v2.0/address-scopes API
    ///
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// AddressScopes response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The ID of the address scope.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ip_version: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Indicates whether this resource is shared across all projects.
    ///
    #[serde()]
    #[structable(optional, wide)]
    shared: Option<BoolString>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,
}

impl AddressScopesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AddressScopes");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.query.shared {
            ep_builder.shared(*val);
        }
        if let Some(val) = &self.query.ip_version {
            ep_builder.ip_version(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
