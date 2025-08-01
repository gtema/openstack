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

//! Set Quota command
//!
//! Wraps invoking of the `v2/quotas/{project_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use eyre::OptionExt;
use eyre::eyre;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::dns::v2::quota::set;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::project::find as find_project;
use openstack_types::dns::v2::quota::response::set::QuotaResponse;
use tracing::warn;

/// Set a projects quotas
///
/// The request should be a key:value set of quotas to be set
///
/// This returns a key:value set of quotas on the system.
#[derive(Args)]
#[command(about = "Set Quotas")]
pub struct QuotaCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(help_heading = "Body parameters", long)]
    api_export_size: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    recordset_records: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    zone_records: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    zone_recordsets: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    zones: Option<i32>,
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

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Quota");

        let op = OutputProcessor::from_args(parsed_args, Some("dns.quota"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Process path parameter `project_id`
        if let Some(id) = &self.path.project.project_id {
            // project_id is passed. No need to lookup
            ep_builder.project_id(id);
        } else if let Some(name) = &self.path.project.project_name {
            // project_name is passed. Need to lookup resource
            let mut sub_find_builder = find_project::Request::builder();
            warn!(
                "Querying project by name (because of `--project-name` parameter passed) may not be definite. This may fail in which case parameter `--project-id` should be used instead."
            );

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
                        ));
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ));
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

        // Set body parameters
        // Set Request.api_export_size data
        if let Some(arg) = &self.api_export_size {
            ep_builder.api_export_size(*arg);
        }

        // Set Request.recordset_records data
        if let Some(arg) = &self.recordset_records {
            ep_builder.recordset_records(*arg);
        }

        // Set Request.zone_records data
        if let Some(arg) = &self.zone_records {
            ep_builder.zone_records(*arg);
        }

        // Set Request.zone_recordsets data
        if let Some(arg) = &self.zone_recordsets {
            ep_builder.zone_recordsets(*arg);
        }

        // Set Request.zones data
        if let Some(arg) = &self.zones {
            ep_builder.zones(*arg);
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<QuotaResponse>(data)?;
        Ok(())
    }
}
