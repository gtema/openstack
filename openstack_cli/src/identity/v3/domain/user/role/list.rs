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
//! Wraps invoking of the `v3/domains/{domain_id}/users/{user_id}/roles` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::domain::user::role::list;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;
use tracing::warn;

/// Lists role assignments for a user on a domain.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/domain_user_roles`
///
#[derive(Args)]
#[command(about = "List role assignments for user on domain")]
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
    /// domain_id parameter for
    /// /v3/domains/{domain_id}/groups/{group_id}/roles/{role_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_domain_id",
        value_name = "DOMAIN_ID"
    )]
    domain_id: String,

    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,
}

/// User input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
struct UserInput {
    /// User Name.
    #[arg(long, help_heading = "Path parameters", value_name = "USER_NAME")]
    user_name: Option<String>,
    /// User ID.
    #[arg(long, help_heading = "Path parameters", value_name = "USER_ID")]
    user_id: Option<String>,
}
/// Roles response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The role description.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The role ID.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The role name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,
}

impl RolesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Roles");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.domain_id(&self.path.domain_id);

        // Process path parameter `user_id`
        if let Some(id) = &self.path.user.user_id {
            // user_id is passed. No need to lookup
            ep_builder.user_id(id);
        } else if let Some(name) = &self.path.user.user_name {
            // user_name is passed. Need to lookup resource
            let mut find_builder = find_user::Request::builder();
            warn!("Querying user by name (because of `--user-name` parameter passed) may not be definite. This may fail in which case parameter `--user-id` should be used instead.");

            find_builder.id(name);
            let find_ep = find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.user_id(id_str.to_owned());
                    }
                    None => {
                        return Err(OpenStackCliError::ResourceAttributeNotString(
                            serde_json::to_string(&val)?,
                        ))
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ))
                }
            };
        }
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
