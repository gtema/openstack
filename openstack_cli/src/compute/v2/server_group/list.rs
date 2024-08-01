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

//! List ServerGroups command
//!
//! Wraps invoking of the `v2.1/os-server-groups` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::server_group::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Lists all server groups for the tenant.
///
/// Administrative users can use the `all_projects` query parameter to list all
/// server groups for all projects.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Server Groups")]
pub struct ServerGroupsCommand {
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
    all_projects: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    #[arg(help_heading = "Query parameters", long)]
    offset: Option<i32>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ServerGroups response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The UUID of the server group.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// A list of members in the server group.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    members: Option<Value>,

    /// Metadata key and value pairs. The maximum size for each metadata key
    /// and value pair is 255 bytes. It’s always empty and only used for
    /// keeping compatibility.
    ///
    /// **Available until version 2.63**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    metadata: Option<Value>,

    /// The name of the server group.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of exactly one policy name to associate with the server group.
    /// The current valid policy names are:
    ///
    /// - `anti-affinity` - servers in this group must be scheduled to
    ///   different hosts.
    /// - `affinity` - servers in this group must be scheduled to the same
    ///   host.
    /// - `soft-anti-affinity` - servers in this group should be scheduled to
    ///   different hosts if possible, but if not possible then they should
    ///   still be scheduled instead of resulting in a build failure. This
    ///   policy was added in microversion 2.15.
    /// - `soft-affinity` - servers in this group should be scheduled to the
    ///   same host if possible, but if not possible then they should still be
    ///   scheduled instead of resulting in a build failure. This policy was
    ///   added in microversion 2.15.
    ///
    /// **Available until version 2.63**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    policies: Option<Value>,

    /// The `policy` field represents the name of the policy. The current valid
    /// policy names are:
    ///
    /// - `anti-affinity` - servers in this group must be scheduled to
    ///   different hosts.
    /// - `affinity` - servers in this group must be scheduled to the same
    ///   host.
    /// - `soft-anti-affinity` - servers in this group should be scheduled to
    ///   different hosts if possible, but if not possible then they should
    ///   still be scheduled instead of resulting in a build failure.
    /// - `soft-affinity` - servers in this group should be scheduled to the
    ///   same host if possible, but if not possible then they should still be
    ///   scheduled instead of resulting in a build failure.
    ///
    /// **New in version 2.64**
    ///
    #[serde()]
    #[structable(optional, wide)]
    policy: Option<String>,

    /// The project ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// The `rules` field, which is a dict, can be applied to the policy.
    /// Currently, only the `max_server_per_host` rule is supported for the
    /// `anti-affinity` policy. The `max_server_per_host` rule allows
    /// specifying how many members of the anti-affinity group can reside on
    /// the same compute host. If not specified, only one member from the same
    /// anti-affinity group can reside on a given host.
    ///
    /// **New in version 2.64**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    rules: Option<Value>,

    /// The user ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,
}

impl ServerGroupsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List ServerGroups");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.all_projects {
            ep_builder.all_projects(val);
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.offset {
            ep_builder.offset(*val);
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