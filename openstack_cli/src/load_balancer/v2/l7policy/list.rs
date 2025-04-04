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

//! List L7Policies command
//!
//! Wraps invoking of the `v2/lbaas/l7policies` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use eyre::OptionExt;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::identity::v3::project::find as find_project;
use openstack_sdk::api::load_balancer::v2::l7policy::list;
use openstack_sdk::api::{Pagination, paged};
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Lists all L7 policies for the project.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and column selection](#filtering).
///
/// Administrative users can specify a project ID that is different than their
/// own to list L7 policies for other projects.
///
/// The list might be empty.
///
#[derive(Args)]
#[command(about = "List L7 Policies")]
pub struct L7PoliciesCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    action: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    admin_state_up: Option<bool>,

    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// Page size
    ///
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    #[arg(help_heading = "Query parameters", long)]
    listener_id: Option<String>,

    /// ID of the last item in the previous list
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    operating_status: Option<String>,

    /// The page direction.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    #[arg(help_heading = "Query parameters", long)]
    position: Option<String>,

    /// Project resource for which the operation should be performed.
    #[command(flatten)]
    project: ProjectInput,

    #[arg(help_heading = "Query parameters", long)]
    provisioning_status: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    redirect_pool_id: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    redirect_prefix: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    redirect_url: Option<String>,
}

/// Project input select group
#[derive(Args)]
#[group(required = false, multiple = false)]
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

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// L7Policies response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The L7 policy action. One of `REDIRECT_PREFIX`, `REDIRECT_TO_POOL`,
    /// `REDIRECT_TO_URL`, or `REJECT`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    action: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional, wide)]
    admin_state_up: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the L7 policy.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The ID of the listener.
    ///
    #[serde()]
    #[structable(optional, wide)]
    listener_id: Option<String>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional, status)]
    operating_status: Option<String>,

    /// The position of this policy on the listener. Positions start at 1.
    ///
    #[serde()]
    #[structable(optional, wide)]
    position: Option<i32>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional, wide)]
    provisioning_status: Option<String>,

    /// Requests matching this policy will be redirected to the specified URL
    /// or Prefix URL with the HTTP response code. Valid if `action` is
    /// `REDIRECT_TO_URL` or `REDIRECT_PREFIX`. Valid options are: 301, 302,
    /// 303, 307, or 308. Default is 302.
    ///
    /// **New in version 2.9**
    ///
    #[serde()]
    #[structable(optional, wide)]
    redirect_http_code: Option<i32>,

    /// Requests matching this policy will be redirected to the pool with this
    /// ID. Only valid if `action` is `REDIRECT_TO_POOL`. The pool has some
    /// restrictions, See
    /// [Protocol Combinations (Listener/Pool)](#valid-protocol).
    ///
    #[serde()]
    #[structable(optional, wide)]
    redirect_pool_id: Option<String>,

    /// Requests matching this policy will be redirected to this Prefix URL.
    /// Only valid if `action` is `REDIRECT_PREFIX`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    redirect_prefix: Option<String>,

    /// Requests matching this policy will be redirected to this URL. Only
    /// valid if `action` is `REDIRECT_TO_URL`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    redirect_url: Option<String>,

    /// List of associated L7 rule IDs.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    rules: Option<Value>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl L7PoliciesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List L7Policies");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.action {
            ep_builder.action(val);
        }
        if let Some(val) = &self.query.admin_state_up {
            ep_builder.admin_state_up(*val);
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.listener_id {
            ep_builder.listener_id(val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.operating_status {
            ep_builder.operating_status(val);
        }
        if let Some(val) = &self.query.page_reverse {
            ep_builder.page_reverse(*val);
        }
        if let Some(val) = &self.query.position {
            ep_builder.position(val);
        }
        if let Some(id) = &self.query.project.project_id {
            // project_id is passed. No need to lookup
            ep_builder.project_id(id);
        } else if let Some(name) = &self.query.project.project_name {
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
        if let Some(val) = &self.query.provisioning_status {
            ep_builder.provisioning_status(val);
        }
        if let Some(val) = &self.query.redirect_pool_id {
            ep_builder.redirect_pool_id(val);
        }
        if let Some(val) = &self.query.redirect_prefix {
            ep_builder.redirect_prefix(val);
        }
        if let Some(val) = &self.query.redirect_url {
            ep_builder.redirect_url(val);
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
