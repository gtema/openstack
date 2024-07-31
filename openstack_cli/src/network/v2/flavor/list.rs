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

//! List Flavors command
//!
//! Wraps invoking of the `v2.0/flavors` with `GET` method

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
use openstack_sdk::api::network::v2::flavor::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists all flavors visible to the project.
///
/// The list can be empty.
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
#[command(about = "List flavors")]
pub struct FlavorsCommand {
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
    /// description query parameter for /v2.0/flavors API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// enabled query parameter for /v2.0/flavors API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    enabled: Option<bool>,

    /// id query parameter for /v2.0/flavors API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// name query parameter for /v2.0/flavors API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// service_type query parameter for /v2.0/flavors API
    ///
    #[arg(help_heading = "Query parameters", long)]
    service_type: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Flavors response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The human-readable description for the flavor.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// Indicates whether the flavor is enabled or not. Default is true.
    ///
    #[serde()]
    #[structable(optional, wide)]
    enabled: Option<BoolString>,

    /// The ID of the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Name of the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// Service profile UUIDs associated with this flavor.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    service_profiles: Option<Value>,

    /// Service type for the flavor. Example: FIREWALL.
    ///
    #[serde()]
    #[structable(optional, wide)]
    service_type: Option<String>,
}

impl FlavorsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Flavors");

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
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.service_type {
            ep_builder.service_type(val);
        }
        if let Some(val) = &self.query.enabled {
            ep_builder.enabled(*val);
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
