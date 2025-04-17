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

//! Set Role command
//!
//! Wraps invoking of the `v3/domains/{domain_id}/users/{user_id}/roles/{role_id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use eyre::OptionExt;
use eyre::eyre;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::domain::find as find_domain;
use openstack_sdk::api::identity::v3::domain::user::role::set;
use openstack_sdk::api::identity::v3::user::find as find_user;
use tracing::warn;

/// Assigns a role to a user on a domain.
///
/// Relationship:
/// `https://developer.openstack.org/api-ref/identity/v3/index.html#assign-role-to-user-on-domain`
#[derive(Args)]
#[command(about = "Assign role to user on domain")]
pub struct RoleCommand {
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
    /// Domain resource for which the operation should be performed.
    #[command(flatten)]
    domain: DomainInput,

    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,

    /// role_id parameter for
    /// /v3/domains/{domain_id}/users/{user_id}/roles/{role_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

/// Domain input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
struct DomainInput {
    /// Domain Name.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_NAME")]
    domain_name: Option<String>,
    /// Domain ID.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_ID")]
    domain_id: Option<String>,
    /// Current domain.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_domain: bool,
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

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Role");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters

        // Process path parameter `domain_id`
        if let Some(id) = &self.path.domain.domain_id {
            // domain_id is passed. No need to lookup
            ep_builder.domain_id(id);
        } else if let Some(name) = &self.path.domain.domain_name {
            // domain_name is passed. Need to lookup resource
            let mut sub_find_builder = find_domain::Request::builder();
            warn!(
                "Querying domain by name (because of `--domain-name` parameter passed) may not be definite. This may fail in which case parameter `--domain-id` should be used instead."
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
                        ep_builder.domain_id(id_str.to_owned());
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
        } else if self.path.domain.current_domain {
            let token = client
                .get_auth_info()
                .ok_or_eyre("Cannot determine current authentication information")?
                .token;
            if let Some(domain) = token.domain {
                ep_builder.domain_id(domain.id.ok_or_eyre("Domain ID is missing in the auth")?);
            } else if let Some(project) = token.project {
                ep_builder.domain_id(
                    project
                        .domain
                        .ok_or_eyre("Domain information is missing in the project auth info")?
                        .id
                        .ok_or_eyre("Domain ID is missing in the project.domain auth info")?,
                );
            } else {
                return Err(eyre!("Current domain information can not be identified").into());
            }
        }

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
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
