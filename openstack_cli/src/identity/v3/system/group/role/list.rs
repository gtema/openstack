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

//! List Roles command
//!
//! Wraps invoking of the `v3/system/groups/{group_id}/roles` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::system::group::role::list;
use openstack_types::identity::v3::system::group::role::response::list::RoleResponse;

/// Lists all system role assignment a group has.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/system_group_roles`
#[derive(Args)]
#[command(about = "List system role assignments for a group")]
pub struct RolesCommand {
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
    /// group_id parameter for /v3/system/groups/{group_id}/roles/{role_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group_id",
        value_name = "GROUP_ID"
    )]
    group_id: String,
}

impl RolesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Roles");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("identity.system/group/role"),
            Some("list"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        ep_builder.group_id(&self.path.group_id);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<RoleResponse>(data)?;
        Ok(())
    }
}
