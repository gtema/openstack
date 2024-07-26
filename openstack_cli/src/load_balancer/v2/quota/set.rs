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
//! Wraps invoking of the `v2/lbaas/quotas/{project_id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::load_balancer::v2::quota::set;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Updates a quota for a project.
///
/// If the request is valid, the service returns the `Accepted (202)` response
/// code.
///
/// This operation returns the updated quota object.
///
/// If the quota is specified as `null` the quota will use the deployment
/// default quota settings.
///
/// Specifying a quota of `-1` means the quota is unlimited.
///
/// Specifying a quota of `0` means the project cannot create any of the
/// resource.
///
#[derive(Args)]
#[command(about = "Update a Quota")]
pub struct QuotaCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Individual quota definitions.
    ///
    #[command(flatten)]
    quota: Quota,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// project_id parameter for /v2/lbaas/quotas/{project_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_project_id",
        value_name = "PROJECT_ID"
    )]
    project_id: String,
}
/// Quota Body data
#[derive(Args, Clone)]
struct Quota {
    #[arg(help_heading = "Body parameters", long)]
    health_monitor: Option<i32>,

    /// The configured health monitor quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    healthmonitor: Option<i32>,

    /// The configured l7policy quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    l7policy: Option<i32>,

    /// The configured l7rule quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    l7rule: Option<i32>,

    /// The configured listener quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    listener: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    load_balancer: Option<i32>,

    /// The configured load balancer quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    loadbalancer: Option<i32>,

    /// The configured member quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    member: Option<i32>,

    /// The configured pool quota limit. A setting of `null` means it is using
    /// the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[arg(help_heading = "Body parameters", long)]
    pool: Option<i32>,
}

/// Quota response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    health_monitor: Option<i32>,

    /// The configured health monitor quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    healthmonitor: Option<i32>,

    /// The configured l7policy quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    l7policy: Option<i32>,

    /// The configured l7rule quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    l7rule: Option<i32>,

    /// The configured listener quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    listener: Option<i32>,

    #[serde()]
    #[structable(optional)]
    load_balancer: Option<i32>,

    /// The configured load balancer quota limit. A setting of `null` means it
    /// is using the deployment default quota. A setting of `-1` means
    /// unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    loadbalancer: Option<i32>,

    /// The configured member quota limit. A setting of `null` means it is
    /// using the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    member: Option<i32>,

    /// The configured pool quota limit. A setting of `null` means it is using
    /// the deployment default quota. A setting of `-1` means unlimited.
    ///
    #[serde()]
    #[structable(optional)]
    pool: Option<i32>,
}

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Quota");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.project_id(&self.path.project_id);
        // Set query parameters
        // Set body parameters
        // Set Request.quota data
        let args = &self.quota;
        let mut quota_builder = set::QuotaBuilder::default();
        if let Some(val) = &args.loadbalancer {
            quota_builder.loadbalancer(*val);
        }

        if let Some(val) = &args.load_balancer {
            quota_builder.load_balancer(*val);
        }

        if let Some(val) = &args.listener {
            quota_builder.listener(*val);
        }

        if let Some(val) = &args.member {
            quota_builder.member(*val);
        }

        if let Some(val) = &args.pool {
            quota_builder.pool(*val);
        }

        if let Some(val) = &args.healthmonitor {
            quota_builder.healthmonitor(*val);
        }

        if let Some(val) = &args.health_monitor {
            quota_builder.health_monitor(*val);
        }

        if let Some(val) = &args.l7policy {
            quota_builder.l7policy(*val);
        }

        if let Some(val) = &args.l7rule {
            quota_builder.l7rule(*val);
        }

        ep_builder.quota(quota_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
