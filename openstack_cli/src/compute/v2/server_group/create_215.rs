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

//! Create ServerGroup command [microversion = 2.15]
//!
//! Wraps invoking of the `v2.1/os-server-groups` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server_group::create_215;
use openstack_types::compute::v2::server_group::response::create::ServerGroupResponse;

/// Creates a server group.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// conflict(409)
#[derive(Args)]
#[command(about = "Create Server Group (microversion = 2.15)")]
pub struct ServerGroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The server group object.
    #[command(flatten)]
    server_group: ServerGroup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ServerGroup Body data
#[derive(Args, Clone)]
struct ServerGroup {
    /// The name of the server group.
    #[arg(help_heading = "Body parameters", long)]
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
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    policies: Vec<String>,
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

        let mut ep_builder = create_215::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.15");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.server_group data
        let args = &self.server_group;
        let mut server_group_builder = create_215::ServerGroupBuilder::default();

        server_group_builder.name(&args.name);

        server_group_builder.policies(args.policies.iter().map(Into::into).collect::<Vec<_>>());

        ep_builder.server_group(server_group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ServerGroupResponse>(data)?;
        Ok(())
    }
}
