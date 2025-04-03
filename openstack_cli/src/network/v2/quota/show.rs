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

//! Show Quota command
//!
//! Wraps invoking of the `v2.0/quotas/{id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::quota::get;
use structable_derive::StructTable;

/// Lists quotas for a project.
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
/// Error response codes: 401, 403
///
#[derive(Args)]
#[command(about = "List quotas for a project")]
pub struct QuotaCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/quotas/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Quota response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The number of floating IP addresses allowed for each project. A value
    /// of `-1` means no limit.
    ///
    #[serde()]
    #[structable(optional)]
    floatingip: Option<i32>,

    /// The number of networks allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[serde()]
    #[structable(optional)]
    network: Option<i32>,

    /// The number of ports allowed for each project. A value of `-1` means no
    /// limit.
    ///
    #[serde()]
    #[structable(optional)]
    port: Option<i32>,

    /// The ID of the project.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The number of role-based access control (RBAC) policies for each
    /// project. A value of `-1` means no limit.
    ///
    #[serde()]
    #[structable(optional)]
    rbac_policy: Option<i32>,

    /// The number of routers allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[serde()]
    #[structable(optional)]
    router: Option<i32>,

    /// The number of security groups allowed for each project. A value of `-1`
    /// means no limit.
    ///
    #[serde()]
    #[structable(optional)]
    security_group: Option<i32>,

    /// The number of security group rules allowed for each project. A value of
    /// `-1` means no limit.
    ///
    #[serde()]
    #[structable(optional)]
    security_group_rule: Option<i32>,

    /// The number of subnets allowed for each project. A value of `-1` means
    /// no limit.
    ///
    #[serde()]
    #[structable(optional)]
    subnet: Option<i32>,

    /// The number of subnet pools allowed for each project. A value of `-1`
    /// means no limit.
    ///
    #[serde()]
    #[structable(optional)]
    subnetpool: Option<i32>,
}

impl QuotaCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Quota");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
