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

//! Set Member command
//!
//! Wraps invoking of the `v2/lbaas/pools/{pool_id}/members/{member_id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find;
use openstack_sdk::api::load_balancer::v2::pool::member::find;
use openstack_sdk::api::load_balancer::v2::pool::member::set;
use openstack_types::load_balancer::v2::pool::member::response::set::MemberResponse;

/// Update an existing member.
///
/// If the request is valid, the service returns the `Accepted (202)` response
/// code. To confirm the update, check that the member provisioning status is
/// `ACTIVE`. If the status is `PENDING_UPDATE`, use a GET operation to poll
/// the member object for changes.
///
/// Setting the member weight to `0` means that the member will not receive new
/// requests but will finish any existing connections. This “drains” the
/// backend member of active connections.
///
/// This operation returns the updated member object with the `ACTIVE`,
/// `PENDING_UPDATE`, or `ERROR` provisioning status.
#[derive(Args)]
#[command(about = "Update a Member")]
pub struct MemberCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Defines attributes that are acceptable of a PUT request.
    #[command(flatten)]
    member: Member,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// member_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id}
    /// API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,

    /// pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_pool_id",
        value_name = "POOL_ID"
    )]
    pool_id: String,
}
/// Member Body data
#[derive(Args, Clone)]
struct Member {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    admin_state_up: Option<bool>,

    /// Is the member a backup? Backup members only receive traffic when all
    /// non-backup members are down.
    ///
    /// **New in version 2.1**
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    backup: Option<bool>,

    /// An alternate IP address used for health monitoring a backend member.
    /// Default is `null` which monitors the member `address`.
    #[arg(help_heading = "Body parameters", long)]
    monitor_address: Option<String>,

    /// An alternate protocol port used for health monitoring a backend member.
    /// Default is `null` which monitors the member `protocol_port`.
    #[arg(help_heading = "Body parameters", long)]
    monitor_port: Option<i32>,

    /// Human-readable name of the resource.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    /// The weight of a member determines the portion of requests or
    /// connections it services compared to the other members of the pool. For
    /// example, a member with a weight of 10 receives five times as many
    /// requests as a member with a weight of 2. A value of 0 means the member
    /// does not receive new connections but continues to service existing
    /// connections. A valid value is from `0` to `256`. Default is `1`.
    #[arg(help_heading = "Body parameters", long)]
    weight: Option<i32>,
}

impl MemberCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Member");

        let op =
            OutputProcessor::from_args(parsed_args, Some("load-balancer.pool/member"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        find_builder.pool_id(&self.path.pool_id);

        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set::Request::builder();

        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.pool_id(resource_id.clone());

        // Set body parameters
        // Set Request.member data
        let args = &self.member;
        let mut member_builder = set::MemberBuilder::default();
        if let Some(val) = &args.admin_state_up {
            member_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.backup {
            member_builder.backup(*val);
        }

        if let Some(val) = &args.monitor_address {
            member_builder.monitor_address(val);
        }

        if let Some(val) = &args.monitor_port {
            member_builder.monitor_port(*val);
        }

        if let Some(val) = &args.name {
            member_builder.name(val);
        }

        if let Some(val) = &args.tags {
            member_builder.tags(val.iter().map(Into::into).collect::<Vec<_>>());
        }

        if let Some(val) = &args.weight {
            member_builder.weight(*val);
        }

        ep_builder.member(member_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<MemberResponse>(data)?;
        Ok(())
    }
}
