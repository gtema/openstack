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
//! Wraps invoking of the `v3/OS-EP-FILTER/endpoint_groups/{endpoint_group_id}/projects/{project_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use eyre::eyre;
use eyre::OptionExt;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::os_ep_filter::endpoint_group::project::get;
use openstack_sdk::api::identity::v3::project::find as find_project;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use tracing::warn;

/// GET operation on
/// /v3/OS-EP-FILTER/endpoint_groups/{endpoint_group_id}/projects/{project_id}
///
#[derive(Args)]
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
    /// endpoint_group_id parameter for
    /// /v3/OS-EP-FILTER/endpoint_groups/{endpoint_group_id}/projects/{project_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_endpoint_group_id",
        value_name = "ENDPOINT_GROUP_ID"
    )]
    endpoint_group_id: String,

    /// Project resource for which the operation should be performed.
    #[command(flatten)]
    project: ProjectInput,
}

/// Project input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
struct ProjectInput {
    /// Project Name.
    #[arg(long, help_heading = "Path parameters", value_name = "PROJECT_NAME")]
    project_name: Option<String>,
    /// Project ID.
    #[arg(long, help_heading = "Path parameters", value_name = "PROJECT_ID")]
    project_id: Option<String>,
    /// Current project.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_project: bool,
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

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.endpoint_group_id(&self.path.endpoint_group_id);

        // Process path parameter `id`
        if let Some(id) = &self.path.project.project_id {
            // project_id is passed. No need to lookup
            ep_builder.id(id);
        } else if let Some(name) = &self.path.project.project_name {
            // project_name is passed. Need to lookup resource
            let mut sub_find_builder = find_project::Request::builder();
            warn!("Querying project by name (because of `--project-name` parameter passed) may not be definite. This may fail in which case parameter `--project-id` should be used instead.");

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.id(id_str.to_owned());
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
        }
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
