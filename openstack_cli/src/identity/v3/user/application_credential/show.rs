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

//! Show ApplicationCredential command
//!
//! Wraps invoking of the `v3/users/{user_id}/application_credentials/{application_credential_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use eyre::OptionExt;
use openstack_sdk::api::find;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::user::application_credential::find;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Show details of an application credential.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/application_credentials`
///
#[derive(Args)]
#[command(about = "Show application credential details")]
pub struct ApplicationCredentialCommand {
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
    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,

    /// application_credential_id parameter for
    /// /v3/users/{user_id}/application_credentials/{application_credential_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
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
/// ApplicationCredential response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, pretty)]
    access_rules: Option<Value>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    expires_at: Option<String>,

    /// The ID of the application credential.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional, pretty)]
    roles: Option<Value>,

    #[serde()]
    #[structable(optional)]
    unrestricted: Option<bool>,
}

impl ApplicationCredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show ApplicationCredential");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        // Process path parameter `user_id`
        if let Some(id) = &self.path.user.user_id {
            // user_id is passed. No need to lookup
            find_builder.user_id(id);
        } else if let Some(name) = &self.path.user.user_name {
            // user_name is passed. Need to lookup resource
            let mut sub_find_builder = find_user::Request::builder();
            warn!("Querying user by name (because of `--user-name` parameter passed) may not be definite. This may fail in which case parameter `--user-id` should be used instead.");

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        find_builder.user_id(id_str.to_owned());
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
        } else if self.path.user.current_user {
            find_builder.user_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
