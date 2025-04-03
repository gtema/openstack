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

//! Set AvailabilityZoneProfile command
//!
//! Wraps invoking of the `v2/lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id}` with `PUT` method

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
use openstack_sdk::api::load_balancer::v2::availability_zone_profile::find;
use openstack_sdk::api::load_balancer::v2::availability_zone_profile::set;
use structable_derive::StructTable;

/// Updates an Availability Zone Profile.
///
#[derive(Args)]
pub struct AvailabilityZoneProfileCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines the attributes of a PUT request.
    ///
    #[command(flatten)]
    availability_zone_profile: AvailabilityZoneProfile,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// availabilityzoneprofile_id parameter for
    /// /v2/lbaas/availabilityzoneprofiles/{availabilityzoneprofile_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// AvailabilityZoneProfile Body data
#[derive(Args, Clone)]
struct AvailabilityZoneProfile {
    #[arg(help_heading = "Body parameters", long)]
    availability_zone_data: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    provider_name: Option<String>,
}

/// AvailabilityZoneProfile response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    availability_zone_data: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional)]
    provider_name: Option<String>,
}

impl AvailabilityZoneProfileCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set AvailabilityZoneProfile");

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
        // Set Request.availability_zone_profile data
        let args = &self.availability_zone_profile;
        let mut availability_zone_profile_builder = set::AvailabilityZoneProfileBuilder::default();
        if let Some(val) = &args.name {
            availability_zone_profile_builder.name(val);
        }

        if let Some(val) = &args.provider_name {
            availability_zone_profile_builder.provider_name(val);
        }

        if let Some(val) = &args.availability_zone_data {
            availability_zone_profile_builder.availability_zone_data(val);
        }

        ep_builder.availability_zone_profile(availability_zone_profile_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
