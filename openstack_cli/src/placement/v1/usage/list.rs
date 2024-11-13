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

//! List Usages command
//!
//! Wraps invoking of the `usages` with `GET` method

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
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::project::find as find_project;
use openstack_sdk::api::identity::v3::user::find as find_user;
use openstack_sdk::api::placement::v1::usage::list;
use openstack_sdk::api::QueryAsync;
use std::collections::HashMap;
use std::fmt;
use tracing::warn;

/// Return a report of usage information for resources associated with the
/// project identified by project_id and user identified by user_id. The value
/// is a dictionary of resource classes paired with the sum of the allocations
/// of that resource class for provided parameters.
///
/// Normal Response Codes: 200
///
/// Error response codes: badRequest(400)
///
#[derive(Args)]
#[command(about = "List usages")]
pub struct UsagesCommand {
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
    /// A string that consists of numbers, A-Z, and _ describing the consumer
    /// type by which to filter usage results. For example, to retrieve only
    /// usage information for ‘INSTANCE’ type consumers a parameter of
    /// consumer_type=INSTANCE should be provided. The all query parameter may
    /// be specified to group all results under one key, all. The unknown query
    /// parameter may be specified to group all results under one key, unknown.
    ///
    #[arg(help_heading = "Query parameters", long)]
    consumer_type: Option<String>,

    /// Project resource for which the operation should be performed.
    #[command(flatten)]
    project: ProjectInput,

    /// User resource for which the operation should be performed.
    #[command(flatten)]
    user: UserInput,
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
struct PathParameters {}
/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
struct ResponseData(HashMap<String, ResponseItem>);

impl StructTable for ResponseData {
    fn build(&self, _options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Name".to_string(), "Value".to_string()]);
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.extend(
            self.0
                .iter()
                .map(|(k, v)| Vec::from([k.clone(), v.to_string()])),
        );
        (headers, rows)
    }
}
/// `struct` response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseItem {
    consumer_count: Option<i32>,
}

impl fmt::Display for ResponseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "consumer_count={}",
            self.consumer_count.map_or(String::new(), |v| v.to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}

impl UsagesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Usages");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(id) = &self.query.project.project_id {
            // project_id is passed. No need to lookup
            ep_builder.project_id(id);
        } else if let Some(name) = &self.query.project.project_name {
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
        } else if self.query.project.current_project {
            ep_builder.project_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        if let Some(id) = &self.query.user.user_id {
            // user_id is passed. No need to lookup
            ep_builder.user_id(id);
        } else if let Some(name) = &self.query.user.user_name {
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
                        ep_builder.user_id(id_str.to_owned());
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
        } else if self.query.user.current_user {
            ep_builder.user_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        if let Some(val) = &self.query.consumer_type {
            ep_builder.consumer_type(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
