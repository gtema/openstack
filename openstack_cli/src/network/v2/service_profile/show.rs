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

//! Show ServiceProfile command
//!
//! Wraps invoking of the `v2.0/service_profiles/{id}` with `GET` method

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
use openstack_sdk::api::network::v2::service_profile::get;
use openstack_sdk::types::BoolString;
use structable_derive::StructTable;

/// Shows details for a service profile.
///
/// This operation returns a service profile object by ID. If you are not an
/// administrative user and the object is not visible to your tenant account,
/// the service returns the HTTP `Forbidden (403)` response code.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403, 404
///
#[derive(Args)]
#[command(about = "Show service profile details")]
pub struct ServiceProfileCommand {
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
    /// id parameter for /v2.0/flavors/{flavor_id}/service_profiles/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// ServiceProfile response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The human-readable description for the service profile.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Provider driver to use for this profile.
    ///
    #[serde()]
    #[structable(optional)]
    driver: Option<String>,

    /// Indicates whether this service profile is enabled or not. Default is
    /// `true`.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<BoolString>,

    /// The UUID of the service profile.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// JSON-formatted meta information of the service profile.
    ///
    #[serde()]
    #[structable(optional)]
    metainfo: Option<String>,
}

impl ServiceProfileCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ServiceProfile");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
