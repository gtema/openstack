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

//! Show Keypair command
//!
//! Wraps invoking of the `v2.1/os-keypairs/{id}` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::keypair::find;
use openstack_sdk::api::find;
use openstack_types::compute::v2::keypair::response::get::KeypairResponse;

/// Shows details for a keypair that is associated with the account.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "Show Keypair Details")]
pub struct KeypairCommand {
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
    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,
}

/// User input select group
#[derive(Args)]
#[group(required = false, multiple = false)]
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

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.1/os-keypairs/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

impl KeypairCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Keypair");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<KeypairResponse>(find_data)?;
        Ok(())
    }
}
