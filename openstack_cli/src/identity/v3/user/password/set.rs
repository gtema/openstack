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

//! Set Password command
//!
//! Wraps invoking of the `v3/users/{user_id}/password` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use dialoguer::Password;
use eyre::OptionExt;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::identity::v3::user::password::set;
use tracing::warn;

/// Changes the password for a user.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/user_change_password`
#[derive(Args)]
#[command(about = "Change password for user")]
pub struct PasswordCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `user` object
    #[command(flatten)]
    user: User,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

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
/// User Body data
#[derive(Args, Clone)]
struct User {
    /// The original password for the user.
    #[arg(help_heading = "Body parameters", long)]
    original_password: Option<String>,

    /// The new password for the user.
    #[arg(help_heading = "Body parameters", long)]
    password: Option<String>,
}

impl PasswordCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Password");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

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
        // Set body parameters
        // Set Request.user data
        let args = &self.user;
        let mut user_builder = set::UserBuilder::default();
        if let Some(val) = &args.original_password {
            user_builder.original_password(val.clone());
        } else {
            let secret = Password::new()
                .with_prompt("The original password for the user")
                .interact()
                .unwrap();
            user_builder.original_password(secret.to_string());
        }

        if let Some(val) = &args.password {
            user_builder.password(val.clone());
        } else {
            let secret = Password::new()
                .with_prompt("The new password for the user")
                .interact()
                .unwrap();
            user_builder.password(secret.to_string());
        }

        ep_builder.user(user_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        openstack_sdk::api::ignore(ep).query_async(client).await?;
        Ok(())
    }
}
