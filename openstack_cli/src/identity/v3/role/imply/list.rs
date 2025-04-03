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

//! List Implies command
//!
//! Wraps invoking of the `v3/roles/{prior_role_id}/implies` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::role::imply::list;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists implied (inference) roles for a role.
///
/// Relationship:
/// `https://developer.openstack.org/api-ref/identity/v3/#list-implied-roles-for-role`
///
#[derive(Args)]
#[command(about = "List implied (inference) roles for role")]
pub struct ImpliesCommand {
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
    /// prior_role_id parameter for
    /// /v3/roles/{prior_role_id}/implies/{implied_role_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_prior_role_id",
        value_name = "PRIOR_ROLE_ID"
    )]
    prior_role_id: String,
}
/// Implies response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// An array of implied role objects.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    implies: Option<Value>,

    /// A prior role object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    prior_role: Option<Value>,
}

impl ImpliesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Implies");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.prior_role_id(&self.path.prior_role_id);
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
