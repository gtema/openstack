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

//! Set Flavor command
//!
//! Wraps invoking of the `v2.0/flavors/{id}` with `PUT` method

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
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::flavor::find;
use openstack_sdk::api::network::v2::flavor::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Updates a flavor.
///
/// The service_type cannot be updated as there may be associated service
/// profiles and consumers depending on the value.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404
///
#[derive(Args)]
#[command(about = "Update flavor")]
pub struct FlavorCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `flavor` object.
    ///
    #[command(flatten)]
    flavor: Flavor,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/flavors/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Flavor Body data
#[derive(Args, Clone)]
struct Flavor {
    /// The human-readable description for the flavor.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Indicates whether the flavor is enabled or not. Default is true.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    enabled: Option<Option<bool>>,

    /// Name of the flavor.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    service_profiles: Option<Vec<String>>,
}

/// Flavor response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The human-readable description for the flavor.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Indicates whether the flavor is enabled or not. Default is true.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional, pretty)]
    service_profiles: Option<Value>,

    /// Service type for the flavor. Example: FIREWALL.
    ///
    #[serde()]
    #[structable(optional)]
    service_type: Option<String>,
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Flavor");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.flavor data
        let args = &self.flavor;
        let mut flavor_builder = set::FlavorBuilder::default();
        if let Some(val) = &args.name {
            flavor_builder.name(val);
        }

        if let Some(val) = &args.description {
            flavor_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.service_profiles {
            flavor_builder.service_profiles(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.enabled {
            flavor_builder.enabled(*val);
        }

        ep_builder.flavor(flavor_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
