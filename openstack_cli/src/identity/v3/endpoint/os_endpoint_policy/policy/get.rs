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

//! Get Policy command
//!
//! Wraps invoking of the `v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::endpoint::os_endpoint_policy::policy::get;
use openstack_types::identity::v3::endpoint::os_endpoint_policy::policy::response::get::PolicyResponse;

/// GET operation on /v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy
#[derive(Args)]
pub struct PolicyCommand {
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
    /// endpoint_id parameter for
    /// /v3/endpoints/{endpoint_id}/OS-ENDPOINT-POLICY/policy API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_endpoint_id",
        value_name = "ENDPOINT_ID"
    )]
    endpoint_id: String,
}

impl PolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Policy");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("identity.endpoint/OS_ENDPOINT_POLICY/policy"),
            Some("get"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        ep_builder.endpoint_id(&self.path.endpoint_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<PolicyResponse>(data)?;
        Ok(())
    }
}
