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

//! Set Limit command
//!
//! Wraps invoking of the `v3/limits/{limit_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::limit::set;
use openstack_types::identity::v3::limit::response::set::LimitResponse;

/// Updates the specified limit. It only supports to update `resource_limit` or
/// `description` for the limit.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/limit`
#[derive(Args)]
#[command(about = "Update Limit")]
pub struct LimitCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `limit` object
    #[command(flatten)]
    limit: Limit,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// limit_id parameter for /v3/limits/{limit_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Limit Body data
#[derive(Args, Clone)]
struct Limit {
    /// The limit description.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The override limit.
    #[arg(help_heading = "Body parameters", long)]
    resource_limit: Option<i32>,
}

impl LimitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Limit");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.limit data
        let args = &self.limit;
        let mut limit_builder = set::LimitBuilder::default();
        if let Some(val) = &args.resource_limit {
            limit_builder.resource_limit(*val);
        }

        if let Some(val) = &args.description {
            limit_builder.description(Some(val.into()));
        }

        ep_builder.limit(limit_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<LimitResponse>(data)?;
        Ok(())
    }
}
