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

//! List Subnetpools command
//!
//! Wraps invoking of the `v2.0/subnetpools` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use crate::common::IntString;
use openstack_sdk::api::network::v2::subnetpool::list;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use structable_derive::StructTable;

/// Lists subnet pools that the project has access to.
///
/// Default policy settings return only the subnet pools owned by the project
/// of the user submitting the request, unless the user has administrative
/// role.
///
/// Standard query parameters are supported on the URI. For more information,
/// see [Filtering and Column Selection](#filtering).
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. For more information, see [Fields](#fields).
///
/// Pagination query parameters are supported if Neutron configuration supports
/// it by overriding `allow_pagination=false`. For more information, see
/// [Pagination](#pagination).
///
/// Sorting query parameters are supported if Neutron configuration supports it
/// with `allow_sorting=true`. For more information, see [Sorting](#sorting).
///
/// Normal response codes: 200
///
/// Error response codes: 401
///
#[derive(Args)]
#[command(about = "List subnet pools")]
pub struct SubnetpoolsCommand {
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
    /// address_scope_id query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    address_scope_id: Option<String>,

    /// default_prefixlen query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    default_prefixlen: Option<i32>,

    /// default_quota query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    default_quota: Option<i32>,

    /// description query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// id query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// ip_version query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    ip_version: Option<String>,

    /// is_default query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    is_default: Option<bool>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    ///
    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    /// max_prefixlen query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    max_prefixlen: Option<i32>,

    /// min_prefixlen query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    min_prefixlen: Option<i32>,

    /// name query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// not-tags query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags_any: Option<Vec<String>>,

    /// Reverse the page direction
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    page_reverse: Option<bool>,

    /// revision_number query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// shared query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Query parameters", long)]
    shared: Option<bool>,

    /// Sort direction. This is an optional feature and may be silently ignored
    /// by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_dir: Option<Vec<String>>,

    /// Sort results by the attribute. This is an optional feature and may be
    /// silently ignored by the server.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    sort_key: Option<Vec<String>>,

    /// tags query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/subnetpools API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags_any: Option<Vec<String>>,

    /// tenant_id query parameter for /v2.0/subnetpools API
    ///
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Subnetpools response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// An address scope to assign to the subnet pool.
    ///
    #[serde()]
    #[structable(optional, wide)]
    address_scope_id: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The size of the prefix to allocate when the `cidr` or `prefixlen`
    /// attributes are omitted when you create the subnet. Default is
    /// `min_prefixlen`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    default_prefixlen: Option<IntString>,

    /// A per-project quota on the prefix space that can be allocated from the
    /// subnet pool for project subnets. Default is no quota is enforced on
    /// allocations from the subnet pool. For IPv4 subnet pools,
    /// `default_quota` is measured in units of /32. For IPv6 subnet pools,
    /// `default_quota` is measured units of /64. All projects that use the
    /// subnet pool have the same prefix quota applied.
    ///
    #[serde()]
    #[structable(optional, wide)]
    default_quota: Option<IntString>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The ID of the subnet pool.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    ip_version: Option<i32>,

    /// The subnetpool is default pool or not.
    ///
    #[serde()]
    #[structable(optional, wide)]
    is_default: Option<BoolString>,

    /// The maximum prefix size that can be allocated from the subnet pool. For
    /// IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is
    /// `128`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    max_prefixlen: Option<IntString>,

    /// The smallest prefix that can be allocated from a subnet pool. For IPv4
    /// subnet pools, default is `8`. For IPv6 subnet pools, default is `64`.
    ///
    #[serde()]
    #[structable(optional, wide)]
    min_prefixlen: Option<IntString>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A list of subnet prefixes to assign to the subnet pool. The API merges
    /// adjacent prefixes and treats them as a single prefix. Each subnet
    /// prefix must be unique among all subnet prefixes in all subnet pools
    /// that are associated with the address scope.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    prefixes: Option<Value>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    ///
    #[serde()]
    #[structable(optional, wide)]
    shared: Option<BoolString>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional, wide)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl SubnetpoolsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Subnetpools");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
        }
        if let Some(val) = &self.query.default_quota {
            ep_builder.default_quota(*val);
        }
        if let Some(val) = &self.query.ip_version {
            ep_builder.ip_version(val);
        }
        if let Some(val) = &self.query.default_prefixlen {
            ep_builder.default_prefixlen(*val);
        }
        if let Some(val) = &self.query.min_prefixlen {
            ep_builder.min_prefixlen(*val);
        }
        if let Some(val) = &self.query.max_prefixlen {
            ep_builder.max_prefixlen(*val);
        }
        if let Some(val) = &self.query.is_default {
            ep_builder.is_default(*val);
        }
        if let Some(val) = &self.query.shared {
            ep_builder.shared(*val);
        }
        if let Some(val) = &self.query.address_scope_id {
            ep_builder.address_scope_id(val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val.iter());
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val.iter());
        }
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.page_reverse {
            ep_builder.page_reverse(*val);
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
