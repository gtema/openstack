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
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use dialoguer::Password;
use http::Response;
use openstack_sdk::api::identity::v3::user::password::set;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Changes the password for a user.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/user_change_password`
///
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
    ///
    #[command(flatten)]
    user: User,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// user_id parameter for /v3/users/{user_id}/access_rules/{access_rule_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_user_id",
        value_name = "USER_ID"
    )]
    user_id: String,
}
/// User Body data
#[derive(Args, Clone)]
struct User {
    /// The original password for the user.
    ///
    #[arg(help_heading = "Body parameters", long)]
    original_password: Option<String>,

    /// The new password for the user.
    ///
    #[arg(help_heading = "Body parameters", long)]
    password: Option<String>,
}

/// Password response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

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
        ep_builder.user_id(&self.path.user_id);
        // Set query parameters
        // Set body parameters
        // Set Request.user data
        let args = &self.user;
        let mut user_builder = set::UserBuilder::default();
        if let Some(val) = &args.original_password {
            user_builder.original_password(val);
        } else {
            let secret = Password::new()
                .with_prompt("The original password for the user")
                .interact()
                .unwrap();
            user_builder.original_password(secret.to_string());
        }

        if let Some(val) = &args.password {
            user_builder.password(val);
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

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
