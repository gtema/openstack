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

//! Show Project command
//!
//! Wraps invoking of the `v3/projects/{project_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::find;
use openstack_sdk::api::identity::v3::project::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Shows details for a project.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/project`
///
#[derive(Args)]
#[command(about = "Show project details")]
pub struct ProjectCommand {
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
    /// project_id parameter for
    /// /v3/projects/{project_id}/groups/{group_id}/roles API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Project response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the project.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    #[serde()]
    #[structable(optional)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional)]
    enabled: Option<bool>,

    /// The ID for the project.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional)]
    is_domain: Option<bool>,

    /// The name of the project.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The resource options for the project. Available resource options are
    /// `immutable`.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    options: Option<Value>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    ///
    #[serde()]
    #[structable(optional)]
    parent_id: Option<String>,

    /// A list of simple strings assigned to a project.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,
}

impl ProjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Project");

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
