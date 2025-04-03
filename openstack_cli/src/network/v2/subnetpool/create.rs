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

//! Create Subnetpool command
//!
//! Wraps invoking of the `v2.0/subnetpools` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::network::v2::subnetpool::create;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::types::BoolString;
use openstack_sdk::types::IntString;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a subnet pool.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 404
///
#[derive(Args)]
#[command(about = "Create subnet pool")]
pub struct SubnetpoolCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `subnetpool` object.
    ///
    #[command(flatten)]
    subnetpool: Subnetpool,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Subnetpool Body data
#[derive(Args, Clone)]
struct Subnetpool {
    /// An address scope to assign to the subnet pool.
    ///
    #[arg(help_heading = "Body parameters", long)]
    address_scope_id: Option<String>,

    /// The size of the prefix to allocate when the `cidr` or `prefixlen`
    /// attributes are omitted when you create the subnet. Default is
    /// `min_prefixlen`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    default_prefixlen: Option<i32>,

    /// A per-project quota on the prefix space that can be allocated from the
    /// subnet pool for project subnets. Default is no quota is enforced on
    /// allocations from the subnet pool. For IPv4 subnet pools,
    /// `default_quota` is measured in units of /32. For IPv6 subnet pools,
    /// `default_quota` is measured units of /64. All projects that use the
    /// subnet pool have the same prefix quota applied.
    ///
    #[arg(help_heading = "Body parameters", long)]
    default_quota: Option<i32>,

    /// A human-readable description for the resource. Default is an empty
    /// string.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The subnetpool is default pool or not.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    is_default: Option<bool>,

    /// The maximum prefix size that can be allocated from the subnet pool. For
    /// IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is
    /// `128`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    max_prefixlen: Option<i32>,

    /// The smallest prefix that can be allocated from a subnet pool. For IPv4
    /// subnet pools, default is `8`. For IPv6 subnet pools, default is `64`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_prefixlen: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// A list of subnet prefixes to assign to the subnet pool. The API merges
    /// adjacent prefixes and treats them as a single prefix. Each subnet
    /// prefix must be unique among all subnet prefixes in all subnet pools
    /// that are associated with the address scope.
    ///
    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    prefixes: Option<Vec<String>>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    shared: Option<bool>,

    /// The ID of the project.
    ///
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

/// Subnetpool response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// An address scope to assign to the subnet pool.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional)]
    default_prefixlen: Option<IntString>,

    /// A per-project quota on the prefix space that can be allocated from the
    /// subnet pool for project subnets. Default is no quota is enforced on
    /// allocations from the subnet pool. For IPv4 subnet pools,
    /// `default_quota` is measured in units of /32. For IPv6 subnet pools,
    /// `default_quota` is measured units of /64. All projects that use the
    /// subnet pool have the same prefix quota applied.
    ///
    #[serde()]
    #[structable(optional)]
    default_quota: Option<IntString>,

    /// A human-readable description for the resource.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the subnet pool.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    #[serde()]
    #[structable(optional)]
    ip_version: Option<i32>,

    /// The subnetpool is default pool or not.
    ///
    #[serde()]
    #[structable(optional)]
    is_default: Option<BoolString>,

    /// The maximum prefix size that can be allocated from the subnet pool. For
    /// IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is
    /// `128`.
    ///
    #[serde()]
    #[structable(optional)]
    max_prefixlen: Option<IntString>,

    /// The smallest prefix that can be allocated from a subnet pool. For IPv4
    /// subnet pools, default is `8`. For IPv6 subnet pools, default is `64`.
    ///
    #[serde()]
    #[structable(optional)]
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
    #[structable(optional, pretty)]
    prefixes: Option<Value>,

    /// The revision number of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    ///
    #[serde()]
    #[structable(optional)]
    shared: Option<BoolString>,

    /// The list of tags on the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl SubnetpoolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Subnetpool");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.subnetpool data
        let args = &self.subnetpool;
        let mut subnetpool_builder = create::SubnetpoolBuilder::default();
        if let Some(val) = &args.name {
            subnetpool_builder.name(val);
        }

        if let Some(val) = &args.tenant_id {
            subnetpool_builder.tenant_id(val);
        }

        if let Some(val) = &args.prefixes {
            subnetpool_builder.prefixes(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.default_quota {
            subnetpool_builder.default_quota(*val);
        }

        if let Some(val) = &args.default_prefixlen {
            subnetpool_builder.default_prefixlen(*val);
        }

        if let Some(val) = &args.min_prefixlen {
            subnetpool_builder.min_prefixlen(*val);
        }

        if let Some(val) = &args.max_prefixlen {
            subnetpool_builder.max_prefixlen(*val);
        }

        if let Some(val) = &args.is_default {
            subnetpool_builder.is_default(*val);
        }

        if let Some(val) = &args.shared {
            subnetpool_builder.shared(*val);
        }

        if let Some(val) = &args.address_scope_id {
            subnetpool_builder.address_scope_id(Some(val.into()));
        }

        if let Some(val) = &args.description {
            subnetpool_builder.description(val);
        }

        ep_builder.subnetpool(subnetpool_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
