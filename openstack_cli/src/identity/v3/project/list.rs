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

//! List Projects command
//!
//! Wraps invoking of the `v3/projects` with `GET` method

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
use openstack_sdk::api::identity::v3::domain::find as find_domain;
use openstack_sdk::api::identity::v3::project::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Lists projects.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/projects`
///
#[derive(Args)]
#[command(about = "List projects")]
pub struct ProjectsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// Domain resource for which the operation should be performed.
    #[command(flatten)]
    domain: DomainInput,

    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    enabled: Option<bool>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    is_domain: Option<bool>,

    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// ID of the last fetched entry
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// The resource name.
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    not_tags: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    not_tags_any: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    parent_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    tags: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    tags_any: Option<String>,
}

/// Domain input select group
#[derive(Args)]
#[group(required = false, multiple = false)]
struct DomainInput {
    /// Domain Name.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_NAME")]
    domain_name: Option<String>,
    /// Domain ID.
    #[arg(long, help_heading = "Path parameters", value_name = "DOMAIN_ID")]
    domain_id: Option<String>,
    /// Current domain.
    #[arg(long, help_heading = "Path parameters", action = clap::ArgAction::SetTrue)]
    current_domain: bool,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Projects response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the domain for the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    #[serde()]
    #[structable(optional, wide)]
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
    #[structable(optional, wide)]
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
    #[structable(optional, pretty, wide)]
    options: Option<Value>,

    /// The ID of the parent for the project.
    ///
    /// **New in version 3.4**
    ///
    #[serde()]
    #[structable(optional, wide)]
    parent_id: Option<String>,

    /// A list of simple strings assigned to a project.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,
}

impl ProjectsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Projects");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(id) = &self.query.domain.domain_id {
            // domain_id is passed. No need to lookup
            ep_builder.domain_id(id);
        } else if let Some(name) = &self.query.domain.domain_name {
            // domain_name is passed. Need to lookup resource
            let mut sub_find_builder = find_domain::Request::builder();
            warn!("Querying domain by name (because of `--domain-name` parameter passed) may not be definite. This may fail in which case parameter `--domain-id` should be used instead.");

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.domain_id(id_str.to_owned());
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
        } else if self.query.domain.current_domain {
            ep_builder.domain_id(
                client
                    .get_auth_info()
                    .ok_or_eyre("Cannot determine current authentication information")?
                    .token
                    .user
                    .id,
            );
        }
        if let Some(val) = &self.query.enabled {
            ep_builder.enabled(*val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.parent_id {
            ep_builder.parent_id(val);
        }
        if let Some(val) = &self.query.is_domain {
            ep_builder.is_domain(*val);
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val);
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val);
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val);
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
