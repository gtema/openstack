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

//! Show Endpoint command
//!
//! Wraps invoking of the `v3/policies/{policy_id}/OS-ENDPOINT-POLICY/endpoints/{endpoint_id}` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::policy::os_endpoint_policy::endpoint::get;
use openstack_types::identity::v3::policy::os_endpoint_policy::endpoint::response::get::EndpointResponse;

/// GET operation on
/// /v3/policies/{policy_id}/OS-ENDPOINT-POLICY/endpoints/{endpoint_id}
#[derive(Args)]
pub struct EndpointCommand {
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
    /// policy_id parameter for
    /// /v3/policies/{policy_id}/OS-ENDPOINT-POLICY/endpoints/{endpoint_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_policy_id",
        value_name = "POLICY_ID"
    )]
    policy_id: String,

    /// endpoint_id parameter for
    /// /v3/policies/{policy_id}/OS-ENDPOINT-POLICY/endpoints/{endpoint_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl EndpointCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Endpoint");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.policy_id(&self.path.policy_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<EndpointResponse>(data)?;
        Ok(())
    }
}
