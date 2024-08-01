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

//! Delete Role command
//!
//! Wraps invoking of the `v3/system/users/{user_id}/roles/{role_id}` with `DELETE` method

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
use openstack_sdk::api::identity::v3::system::user::role::delete;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Remove a system role assignment from a user.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/system_user_role`
///
#[derive(Args)]
#[command(about = "Delete a system role assignment from a user")]
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
    /// user_id parameter for /v3/system/users/{user_id}/roles/{role_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_user_id",
        value_name = "USER_ID"
    )]
    user_id: String,

    /// role_id parameter for /v3/system/users/{user_id}/roles/{role_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Role response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Delete Role");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = delete::Request::builder();

        // Set path parameters
        ep_builder.user_id(&self.path.user_id);
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        Ok(())
    }
}