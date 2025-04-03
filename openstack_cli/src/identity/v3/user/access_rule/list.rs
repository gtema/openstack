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

//! List AccessRules command
//!
//! Wraps invoking of the `v3/users/{user_id}/access_rules` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use eyre::OptionExt;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::user::access_rule::list;
use openstack_sdk::api::identity::v3::user::find as find_user;
use structable_derive::StructTable;
use tracing::warn;

/// List all access rules for a user.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/access_rules`
///
#[derive(Args)]
#[command(about = "List access rules")]
pub struct AccessRulesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// The request method that the application credential is permitted to use
    /// for a given API endpoint.
    ///
    #[arg(help_heading = "Query parameters", long)]
    method: Option<String>,

    /// The API path that the application credential is permitted to access.
    ///
    #[arg(help_heading = "Query parameters", long)]
    path: Option<String>,

    /// The service type identifier for the service that the application is
    /// permitted to access.
    ///
    #[arg(help_heading = "Query parameters", long)]
    service: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {
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
    /// Current authenticated user.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_user: bool,
}
/// AccessRules response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The UUID of the access rule
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The request method that the application credential is permitted to use
    /// for a given API endpoint.
    ///
    #[serde()]
    #[structable(optional, wide)]
    method: Option<String>,

    /// The API path that the application credential is permitted to access.
    ///
    #[serde()]
    #[structable(optional, wide)]
    path: Option<String>,

    /// The service type identifier for the service that the application
    /// credential is permitted to access. Must be a service type that is
    /// listed in the service catalog and not a code name for a service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    service: Option<String>,
}

impl AccessRulesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AccessRules");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters

        // Process path parameter `user_id`
        if let Some(id) = &self.path.user.user_id {
            // user_id is passed. No need to lookup
            ep_builder.user_id(id);
        } else if let Some(name) = &self.path.user.user_name {
            // user_name is passed. Need to lookup resource
            let mut sub_find_builder = find_user::Request::builder();
            warn!(
                "Querying user by name (because of `--user-name` parameter passed) may not be definite. This may fail in which case parameter `--user-id` should be used instead."
            );

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
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
                        ));
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ));
                }
            };
        } else if self.path.user.current_user {
            ep_builder.user_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        // Set query parameters
        if let Some(val) = &self.query.service {
            ep_builder.service(val);
        }
        if let Some(val) = &self.query.path {
            ep_builder.path(val);
        }
        if let Some(val) = &self.query.method {
            ep_builder.method(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
