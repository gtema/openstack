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

//! Create Flavor command
//!
//! Wraps invoking of the `v2.0/flavors` with `POST` method

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
use openstack_sdk::api::network::v2::flavor::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a flavor.
///
/// This operation establishes a new flavor.
///
/// The service_type to which the flavor applies is a required parameter. The
/// corresponding service plugin must have been activated as part of the
/// configuration. Check [Service providers](#list-service-providers) for how
/// to see currently loaded service types. Additionally the service plugin
/// needs to support the use of flavors.
///
/// Creation currently limited to administrators. Other users will receive a
/// `Forbidden 403` response code with a response body NeutronError message
/// expressing that creation is disallowed by policy.
///
/// Until one or more service profiles are associated with the flavor by the
/// operator, attempts to use the flavor during resource creations will
/// currently return a `Not Found 404` with a response body that indicates no
/// service profile could be found.
///
/// If the API cannot fulfill the request due to insufficient data or data that
/// is not valid, the service returns the HTTP `Bad Request (400)` response
/// code with information about the failure in the response body. Validation
/// errors require that you correct the error and submit the request again.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 404
///
#[derive(Args)]
#[command(about = "Create flavor")]
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
struct PathParameters {}
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

    /// Service type for the flavor. Example: FIREWALL.
    ///
    #[arg(help_heading = "Body parameters", long)]
    service_type: Option<String>,
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
        info!("Create Flavor");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.flavor data
        let args = &self.flavor;
        let mut flavor_builder = create::FlavorBuilder::default();
        if let Some(val) = &args.name {
            flavor_builder.name(val);
        }

        if let Some(val) = &args.description {
            flavor_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.service_type {
            flavor_builder.service_type(val);
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
