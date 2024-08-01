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

//! Action InheritedToProject command
//!
//! Wraps invoking of the `v3/OS-INHERIT/projects/{project_id}/groups/{group_id}/roles/{role_id}/inherited_to_projects` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::identity::v3::os_inherit::project::group::role::inherited_to_project::inherited_to_projects;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Request of the
/// OS-INHERIT/projects/project_id/groups/group_id/roles/role_id/inherited_to_projects:put
/// operation
///
#[derive(Args)]
#[command(about = "Assign role to group on projects in a subtree")]
pub struct InheritedToProjectCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// project_id parameter for
    /// /v3/OS-INHERIT/projects/{project_id}/groups/{group_id}/roles/{role_id}/inherited_to_projects
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_project_id",
        value_name = "PROJECT_ID"
    )]
    project_id: String,

    /// group_id parameter for
    /// /v3/OS-INHERIT/projects/{project_id}/groups/{group_id}/roles/{role_id}/inherited_to_projects
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_group_id",
        value_name = "GROUP_ID"
    )]
    group_id: String,

    /// role_id parameter for
    /// /v3/OS-INHERIT/projects/{project_id}/groups/{group_id}/roles/{role_id}/inherited_to_projects
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_role_id",
        value_name = "ROLE_ID"
    )]
    role_id: String,
}
/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
struct ResponseData(HashMap<String, Value>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(self.0.iter().map(|(k, v)| {
            Vec::from([
                k.clone(),
                serde_json::to_string(&v).expect("Is a valid data"),
            ])
        }));
        (headers, rows)
    }
}

impl InheritedToProjectCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action InheritedToProject");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = inherited_to_projects::Request::builder();

        // Set path parameters
        ep_builder.project_id(&self.path.project_id);
        ep_builder.group_id(&self.path.group_id);
        ep_builder.role_id(&self.path.role_id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}