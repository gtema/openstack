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
//! Wraps invoking of the `v2/lbaas/pools/{pool_id}/members` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use openstack_sdk::api::find;
use openstack_sdk::api::load_balancer::v2::pool::member::find;
use openstack_sdk::api::load_balancer::v2::pool::member::replace;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Set the state of members for a pool in one API call. This may include
/// creating new members, deleting old members, and updating existing members.
/// Existing members are matched based on address/port combination.
///
/// For example, assume a pool currently has two members. These members have
/// the following address/port combinations: ‘192.0.2.15:80’ and
/// ‘192.0.2.16:80’. Now assume a PUT request is made that includes members
/// with address/port combinations: ‘192.0.2.16:80’ and ‘192.0.2.17:80’.
///
/// The member ‘192.0.2.15:80’ will be deleted, because it was not in the
/// request.
///
/// The member ‘192.0.2.16:80’ will be updated to match the request data for
/// that member, because it was matched.
///
/// The member ‘192.0.2.17:80’ will be created, because no such member existed.
///
/// The optional parameter `additive_only` when defined as `true` will skip
/// deletions for members missing from the provided list. If this were set in
/// the above example, the member ‘192.0.2.15:80’ would have remained in the
/// pool.
///
/// If the request is valid, the service returns the `Accepted (202)` response
/// code. To confirm the updates, check that the member provisioning statuses
/// are `ACTIVE` for new or updated members, and that any unspecified members
/// were correctly deleted. If the statuses are `PENDING_UPDATE` or
/// `PENDING_DELETE`, use GET to poll the member objects for changes.
///
#[derive(Args)]
#[command(about = "Batch Update Members")]
pub struct MemberCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    members: Vec<Value>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_pool_id",
        value_name = "POOL_ID"
    )]
    pool_id: String,
}
/// Member response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The IP address of the backend member server.
    ///
    #[serde()]
    #[structable(optional)]
    address: Option<String>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<bool>,

    /// Is the member a backup? Backup members only receive traffic when all
    /// non-backup members are down.
    ///
    /// **New in version 2.1**
    ///
    #[serde()]
    #[structable(optional)]
    backup: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The ID of the member.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// An alternate IP address used for health monitoring a backend member.
    /// Default is `null` which monitors the member `address`.
    ///
    #[serde()]
    #[structable(optional)]
    monitor_address: Option<String>,

    /// An alternate protocol port used for health monitoring a backend member.
    /// Default is `null` which monitors the member `protocol_port`.
    ///
    #[serde()]
    #[structable(optional)]
    monitor_port: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    #[serde()]
    #[structable(optional)]
    operating_status: Option<String>,

    /// The ID of the project owning this resource.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The protocol port number the backend member server is listening on.
    ///
    #[serde()]
    #[structable(optional)]
    protocol_port: Option<i32>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    #[serde()]
    #[structable(optional)]
    provisioning_status: Option<String>,

    /// The subnet ID the member service is accessible from.
    ///
    #[serde()]
    #[structable(optional)]
    subnet_id: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The member vNIC type used for the member port. One of `normal` or
    /// `direct`.
    ///
    /// **New in version 2.29**
    ///
    #[serde()]
    #[structable(optional)]
    vnic_type: Option<String>,

    /// The weight of a member determines the portion of requests or
    /// connections it services compared to the other members of the pool. For
    /// example, a member with a weight of 10 receives five times as many
    /// requests as a member with a weight of 2. A value of 0 means the member
    /// does not receive new connections but continues to service existing
    /// connections. A valid value is from `0` to `256`. Default is `1`.
    ///
    #[serde()]
    #[structable(optional)]
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

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.pool_id(&self.path.pool_id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = replace::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.pool_id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.members data

        let members_builder: Vec<replace::Members> = self
            .members
            .iter()
            .flat_map(|v| serde_json::from_value::<replace::Members>(v.to_owned()))
            .collect::<Vec<replace::Members>>();
        ep_builder.members(members_builder);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
