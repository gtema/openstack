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

//! List Systems command
//!
//! Wraps invoking of the `v3/auth/system` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::auth::system::list;
use openstack_types::identity::v3::auth::system::response::list::SystemResponse;

/// New in version 3.10
///
/// This call returns the list of systems that are available to be scoped to
/// based on the X-Auth-Token provided in the request.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/auth_system`
#[derive(Args)]
#[command(about = "Get available system scopes")]
pub struct SystemsCommand {
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
struct PathParameters {}

impl SystemsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Systems");

        let op =
            OutputProcessor::from_args(parsed_args, Some("identity.auth/system"), Some("list"));
        op.validate_args(parsed_args)?;

        let ep_builder = list::Request::builder();

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<SystemResponse>(data)?;
        Ok(())
    }
}
