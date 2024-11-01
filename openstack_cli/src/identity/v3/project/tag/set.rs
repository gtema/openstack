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

//! Set Tag command
//!
//! Wraps invoking of the `v3/projects/{project_id}/tags/{value}` with `PUT` method

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
use eyre::eyre;
use eyre::OptionExt;
use http::Response;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::project::find as find_project;
use openstack_sdk::api::identity::v3::project::tag::set;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;
use tracing::warn;

/// Creates the specified tag and adds it to the list of tags in the project.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/projects`
///
#[derive(Args)]
#[command(about = "Add single tag to a project")]
pub struct TagCommand {
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
    /// Project resource for which the operation should be performed.
    #[command(flatten)]
    project: ProjectInput,

    /// value parameter for /v3/projects/{project_id}/tags/{value} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_value",
        value_name = "VALUE"
    )]
    value: String,
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
/// Tag response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl TagCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Tag");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters

        // Process path parameter `project_id`
        if let Some(id) = &self.path.project.project_id {
            // project_id is passed. No need to lookup
            ep_builder.project_id(id);
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
                        ep_builder.project_id(id_str.to_owned());
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
        } else if self.path.project.current_project {
            let token = client
                .get_auth_info()
                .ok_or_eyre("Cannot determine current authentication information")?
                .token;
            if let Some(project) = token.project {
                ep_builder.project_id(
                    project
                        .id
                        .ok_or_eyre("Project ID is missing in the project auth info")?,
                );
            } else {
                return Err(eyre!("Current project information can not be identified").into());
            }
        }
        ep_builder.value(&self.path.value);
        // Set query parameters
        // Set body parameters

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
