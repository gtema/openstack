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

//! Create ServerGroup command [microversion = 2.64]
//!
//! Wraps invoking of the `v2.1/os-server-groups` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server_group::create_264;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a server group.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
///
#[derive(Args)]
#[command(about = "Create Server Group (microversion = 2.64)")]
pub struct ServerGroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The server group object.
    ///
    #[command(flatten)]
    server_group: ServerGroup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Policy {
    Affinity,
    AntiAffinity,
    SoftAffinity,
    SoftAntiAffinity,
}

/// Rules Body data
#[derive(Args, Clone)]
#[group(required = false, multiple = true)]
struct Rules {
    #[arg(help_heading = "Body parameters", long)]
    max_server_per_host: Option<i32>,
}

/// ServerGroup Body data
#[derive(Args, Clone)]
struct ServerGroup {
    /// The name of the server group.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: String,

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
    #[arg(help_heading = "Body parameters", long)]
    policy: Policy,

    /// The `rules` field, which is a dict, can be applied to the policy.
    /// Currently, only the `max_server_per_host` rule is supported for the
    /// `anti-affinity` policy. The `max_server_per_host` rule allows
    /// specifying how many members of the anti-affinity group can reside on
    /// the same compute host. If not specified, only one member from the same
    /// anti-affinity group can reside on a given host. Requesting policy rules
    /// with any other policy than `anti-affinity` will be 400.
    ///
    /// **New in version 2.64**
    ///
    #[command(flatten)]
    rules: Option<Rules>,
}

/// ServerGroup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The UUID of the server group.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// A list of members in the server group.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    members: Option<Value>,

    /// Metadata key and value pairs. The maximum size for each metadata key
    /// and value pair is 255 bytes. It’s always empty and only used for
    /// keeping compatibility.
    ///
    /// **Available until version 2.63**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The name of the server group.
    ///
    #[serde()]
    #[structable()]
    name: String,

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
    #[structable(optional, pretty)]
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
    #[structable()]
    policy: String,

    /// The project ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    #[serde()]
    #[structable()]
    project_id: String,

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
    #[structable(optional, pretty)]
    rules: Option<Value>,

    /// The user ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    #[serde()]
    #[structable()]
    user_id: String,
}

impl ServerGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ServerGroup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_264::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.64");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.server_group data
        let args = &self.server_group;
        let mut server_group_builder = create_264::ServerGroupBuilder::default();

        server_group_builder.name(&args.name);

        let tmp = match &args.policy {
            Policy::Affinity => create_264::Policy::Affinity,
            Policy::AntiAffinity => create_264::Policy::AntiAffinity,
            Policy::SoftAffinity => create_264::Policy::SoftAffinity,
            Policy::SoftAntiAffinity => create_264::Policy::SoftAntiAffinity,
        };
        server_group_builder.policy(tmp);

        if let Some(val) = &args.rules {
            let mut rules_builder = create_264::RulesBuilder::default();
            if let Some(val) = &val.max_server_per_host {
                rules_builder.max_server_per_host(*val);
            }
            server_group_builder.rules(rules_builder.build().expect("A valid object"));
        }

        ep_builder.server_group(server_group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
