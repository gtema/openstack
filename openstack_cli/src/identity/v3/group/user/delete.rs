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

//! Delete User command
//!
//! Wraps invoking of the `v3/groups/{group_id}/users/{user_id}` with `DELETE` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::group::user::delete;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;
use tracing::warn;

/// Removes a user from a group.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/group_user`
///
#[derive(Args)]
#[command(about = "Remove user from group")]
pub struct UserCommand {
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
    /// group_id parameter for /v3/groups/{group_id}/users/{user_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group_id",
        value_name = "GROUP_ID"
    )]
    group_id: String,

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
/// User response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl UserCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete User");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.group_id(&self.path.group_id);

        // Process path parameter `id`
        if let Some(id) = &self.path.user.user_id {
            // user_id is passed. No need to lookup
            ep_builder.id(id);
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
                        ep_builder.id(id_str.to_owned());
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

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}
