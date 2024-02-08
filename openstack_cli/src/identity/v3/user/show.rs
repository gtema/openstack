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

//! Show User command
//!
//! Wraps invoking of the `v3/users/{user_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::user::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::fmt;
use structable_derive::StructTable;

/// Shows details for a user.
///
/// Relationship: `https://docs.openstack.org/api/openstack-
/// identity/3/rel/user`
#[derive(Args)]
#[command(about = "Show user details")]
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
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}
/// User response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The user ID.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the default project for the user.
    #[serde()]
    #[structable(optional)]
    default_project_id: Option<String>,

    /// The new description of the group.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain.
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`.
    /// If the user is disabled, this value is `false`.
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list
    /// contains the `idp\_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol\_id` and `unique\_id` of
    /// the protocol and user respectively. For example:
    ///
    ///
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp\_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol\_id": "mapped", "unique\_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[serde()]
    #[structable(optional)]
    federated: Option<Value>,

    /// The user name. Must be unique within the owning domain.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The new password for the user.
    #[serde()]
    #[structable(optional)]
    password: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore\_change\_password\_upon\_first\_use`,
    /// `ignore\_password\_expiry`,
    /// `ignore\_lockout\_failure\_attempts`, `lock\_password`,
    /// `multi\_factor\_auth\_enabled`, and `multi\_factor\_auth\_rules`
    /// `ignore\_user\_inactivity`.
    #[serde()]
    #[structable(optional)]
    options: Option<ResponseOptions>,
}
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// Vector of VecString response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct VecVecString(Vec<VecString>);
impl fmt::Display for VecVecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseOptions {
    ignore_change_password_upon_first_use: Option<bool>,
    ignore_password_expiry: Option<bool>,
    ignore_lockout_failure_attempts: Option<bool>,
    lock_password: Option<bool>,
    ignore_user_inactivity: Option<bool>,
    multi_factor_auth_rules: Option<VecVecString>,
    multi_factor_auth_enabled: Option<bool>,
}

impl fmt::Display for ResponseOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ignore_change_password_upon_first_use={}",
                self.ignore_change_password_upon_first_use
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_password_expiry={}",
                self.ignore_password_expiry
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_lockout_failure_attempts={}",
                self.ignore_lockout_failure_attempts
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "lock_password={}",
                self.lock_password
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "ignore_user_inactivity={}",
                self.ignore_user_inactivity
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_rules={}",
                self.multi_factor_auth_rules
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "multi_factor_auth_enabled={}",
                self.multi_factor_auth_enabled
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

impl UserCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show User");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
