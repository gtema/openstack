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

//! Get Limit command
//!
//! Wraps invoking of the `v2.1/limits` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::limit::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows rate and absolute limits for the project.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "Show Rate And Absolute Limits")]
pub struct LimitCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    tenant_id: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Limit response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The number of allowed members for each server group.
    ///
    #[serde(rename = "maxServerGroupMembers")]
    #[structable(optional, title = "maxServerGroupMembers")]
    max_server_group_members: Option<i32>,

    /// The number of allowed server groups for each tenant.
    ///
    #[serde(rename = "maxServerGroups")]
    #[structable(optional, title = "maxServerGroups")]
    max_server_groups: Option<i32>,

    /// The number of allowed metadata items for each server.
    ///
    #[serde(rename = "maxServerMetamaxServerMeta")]
    #[structable(optional, title = "maxServerMetamaxServerMeta")]
    max_server_metamax_server_meta: Option<i32>,

    /// The number of allowed server cores for each tenant.
    ///
    #[serde(rename = "maxTotalCores")]
    #[structable(optional, title = "maxTotalCores")]
    max_total_cores: Option<i32>,

    /// The number of allowed servers for each tenant.
    ///
    #[serde(rename = "maxTotalInstances")]
    #[structable(optional, title = "maxTotalInstances")]
    max_total_instances: Option<i32>,

    /// The number of allowed key pairs for each user.
    ///
    #[serde(rename = "maxTotalKeypairs")]
    #[structable(optional, title = "maxTotalKeypairs")]
    max_total_keypairs: Option<i32>,

    /// The amount of allowed server RAM, in MiB, for each tenant.
    ///
    #[serde(rename = "maxTotalRAMSize")]
    #[structable(optional, title = "maxTotalRAMSize")]
    max_total_ramsize: Option<i32>,

    /// The number of used server cores in each tenant. If `reserved` query
    /// parameter is specified and it is not 0, the number of reserved server
    /// cores are also included.
    ///
    #[serde(rename = "totalCoresUsed")]
    #[structable(optional, title = "totalCoresUsed")]
    total_cores_used: Option<i32>,

    /// The number of servers in each tenant. If `reserved` query parameter is
    /// specified and it is not 0, the number of reserved servers are also
    /// included.
    ///
    #[serde(rename = "totalInstancesUsed")]
    #[structable(optional, title = "totalInstancesUsed")]
    total_instances_used: Option<i32>,

    /// The amount of used server RAM in each tenant. If `reserved` query
    /// parameter is specified and it is not 0, the amount of reserved server
    /// RAM is also included.
    ///
    #[serde(rename = "totalRAMUsed")]
    #[structable(optional, title = "totalRAMUsed")]
    total_ramused: Option<i32>,

    /// The number of used server groups in each tenant. If `reserved` query
    /// parameter is specified and it is not 0, the number of reserved server
    /// groups are also included.
    ///
    #[serde(rename = "totalServerGroupsUsed")]
    #[structable(optional, title = "totalServerGroupsUsed")]
    total_server_groups_used: Option<i32>,
}

impl LimitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Limit");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.tenant_id {
            ep_builder.tenant_id(val);
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
