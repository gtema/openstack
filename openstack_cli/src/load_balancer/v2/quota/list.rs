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

//! List Quotas command
//!
//! Wraps invoking of the `v2/lbaas/quotas` with `GET` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::load_balancer::v2::quota::list;
use openstack_types::load_balancer::v2::quota::response::list::QuotaResponse;

/// Lists all quotas for the project.
///
/// Use the `fields` query parameter to control which fields are returned in
/// the response body. Additionally, you can filter results by using query
/// string parameters. For information, see
/// [Filtering and column selection](#filtering).
///
/// Administrative users can specify a project ID that is different than their
/// own to list quotas for other projects.
///
/// If the quota is listed as `null` the quota is using the deployment default
/// quota settings.
///
/// A quota of `-1` means the quota is unlimited.
///
/// The list might be empty.
#[derive(Args)]
#[command(about = "List Quota")]
pub struct QuotasCommand {
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
struct PathParameters {}

impl QuotasCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Quotas");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<QuotaResponse>(data)?;
        Ok(())
    }
}
