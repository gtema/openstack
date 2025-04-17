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

//! Create SecurityGroup command
//!
//! Wraps invoking of the `v2.0/security-groups` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::security_group::create;
use openstack_types::network::v2::security_group::response::create::SecurityGroupResponse;

/// Creates an OpenStack Networking security group.
///
/// This operation creates a security group with default security group rules
/// for the IPv4 and IPv6 ether types.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 409
#[derive(Args)]
#[command(about = "Create security group")]
pub struct SecurityGroupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `security_group` object.
    #[command(flatten)]
    security_group: SecurityGroup,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// SecurityGroup Body data
#[derive(Args, Clone)]
struct SecurityGroup {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Human-readable name of the resource.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Indicates if the security group is stateful or stateless.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    stateful: Option<bool>,

    /// The ID of the project.
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

impl SecurityGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create SecurityGroup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.security_group data
        let args = &self.security_group;
        let mut security_group_builder = create::SecurityGroupBuilder::default();
        if let Some(val) = &args.name {
            security_group_builder.name(val);
        }

        if let Some(val) = &args.tenant_id {
            security_group_builder.tenant_id(val);
        }

        if let Some(val) = &args.description {
            security_group_builder.description(val);
        }

        if let Some(val) = &args.stateful {
            security_group_builder.stateful(*val);
        }

        ep_builder.security_group(security_group_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<SecurityGroupResponse>(data)?;
        Ok(())
    }
}
