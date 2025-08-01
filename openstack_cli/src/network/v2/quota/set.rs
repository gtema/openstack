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

//! Set Quota command
//!
//! Wraps invoking of the `v2.0/quotas/{id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::quota::set;
use openstack_types::network::v2::quota::response::set::QuotaResponse;

/// Updates quotas for a project. Use when non-default quotas are desired.
///
/// Normal response codes: 200
///
/// Error response codes: 401, 403
#[derive(Args)]
#[command(about = "Update quota for a project")]
pub struct QuotaCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `quota` object.
    #[command(flatten)]
    quota: Quota,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/quotas/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Quota Body data
#[derive(Args, Clone)]
struct Quota {
    /// The number of floating IP addresses allowed for each project. A value
    /// of `-1` means no limit.
    #[arg(help_heading = "Body parameters", long)]
    floatingip: Option<i32>,

    /// The number of networks allowed for each project. A value of `-1` means
    /// no limit.
    #[arg(help_heading = "Body parameters", long)]
    network: Option<i32>,

    /// The number of ports allowed for each project. A value of `-1` means no
    /// limit.
    #[arg(help_heading = "Body parameters", long)]
    port: Option<i32>,

    /// The ID of the project.
    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    /// The number of role-based access control (RBAC) policies for each
    /// project. A value of `-1` means no limit.
    #[arg(help_heading = "Body parameters", long)]
    rbac_policy: Option<i32>,

    /// The number of routers allowed for each project. A value of `-1` means
    /// no limit.
    #[arg(help_heading = "Body parameters", long)]
    router: Option<i32>,

    /// The number of security groups allowed for each project. A value of `-1`
    /// means no limit.
    #[arg(help_heading = "Body parameters", long)]
    security_group: Option<i32>,

    /// The number of security group rules allowed for each project. A value of
    /// `-1` means no limit.
    #[arg(help_heading = "Body parameters", long)]
    security_group_rule: Option<i32>,

    /// The number of subnets allowed for each project. A value of `-1` means
    /// no limit.
    #[arg(help_heading = "Body parameters", long)]
    subnet: Option<i32>,

    /// The number of subnet pools allowed for each project. A value of `-1`
    /// means no limit.
    #[arg(help_heading = "Body parameters", long)]
    subnetpool: Option<i32>,
}

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Quota");

        let op = OutputProcessor::from_args(parsed_args, Some("network.quota"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.quota data
        let args = &self.quota;
        let mut quota_builder = set::QuotaBuilder::default();
        if let Some(val) = &args.floatingip {
            quota_builder.floatingip(*val);
        }

        if let Some(val) = &args.network {
            quota_builder.network(*val);
        }

        if let Some(val) = &args.port {
            quota_builder.port(*val);
        }

        if let Some(val) = &args.project_id {
            quota_builder.project_id(val);
        }

        if let Some(val) = &args.rbac_policy {
            quota_builder.rbac_policy(*val);
        }

        if let Some(val) = &args.router {
            quota_builder.router(*val);
        }

        if let Some(val) = &args.security_group {
            quota_builder.security_group(*val);
        }

        if let Some(val) = &args.security_group_rule {
            quota_builder.security_group_rule(*val);
        }

        if let Some(val) = &args.subnet {
            quota_builder.subnet(*val);
        }

        if let Some(val) = &args.subnetpool {
            quota_builder.subnetpool(*val);
        }

        ep_builder.quota(quota_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<QuotaResponse>(data)?;
        Ok(())
    }
}
