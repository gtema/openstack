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

//! Show Member command
//!
//! Wraps invoking of the `v2/lbaas/pools/{pool_id}/members/{member_id}` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find;
use openstack_sdk::api::load_balancer::v2::pool::member::find;
use openstack_types::load_balancer::v2::pool::member::response::get::MemberResponse;

/// Shows the details of a pool member.
///
/// If you are not an administrative user and the parent load balancer does not
/// belong to your project, the service returns the HTTP `Forbidden (403)`
/// response code.
///
/// This operation does not require a request body.
#[derive(Args)]
#[command(about = "Show Member details")]
pub struct MemberCommand {
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

impl MemberCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Member");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("load-balancer.pool/member"),
            Some("show"),
        );
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        find_builder.pool_id(&self.path.pool_id);

        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<MemberResponse>(find_data)?;
        Ok(())
    }
}
