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

//! Set RegisteredLimit command
//!
//! Wraps invoking of the `v3/registered_limits/{registered_limit_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::registered_limit::set;
use openstack_types::identity::v3::registered_limit::response::set::RegisteredLimitResponse;

/// Updates the specified registered limit.
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/registered_limit`
#[derive(Args)]
#[command(about = "Update Registered Limit")]
pub struct RegisteredLimitCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `registered_limit` objects
    #[command(flatten)]
    registered_limit: RegisteredLimit,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// registered_limit_id parameter for
    /// /v3/registered_limits/{registered_limit_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// RegisteredLimit Body data
#[derive(Args, Clone)]
struct RegisteredLimit {
    /// The default limit for the registered limit.
    #[arg(help_heading = "Body parameters", long)]
    default_limit: Option<i32>,

    /// The registered limit description.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Set explicit NULL for the description
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "description")]
    no_description: bool,

    /// The ID of the region that contains the service endpoint. Either
    /// service_id, resource_name, or region_id must be different than existing
    /// value otherwise it will raise 409.
    #[arg(help_heading = "Body parameters", long)]
    region_id: Option<String>,

    /// Set explicit NULL for the region_id
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "region_id")]
    no_region_id: bool,

    /// The resource name. Either service_id, resource_name or region_id must
    /// be different than existing value otherwise it will raise 409.
    #[arg(help_heading = "Body parameters", long)]
    resource_name: Option<String>,

    /// The UUID of the service to update to which the registered limit
    /// belongs. Either service_id, resource_name, or region_id must be
    /// different than existing value otherwise it will raise 409.
    #[arg(help_heading = "Body parameters", long)]
    service_id: Option<String>,
}

impl RegisteredLimitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set RegisteredLimit");

        let op =
            OutputProcessor::from_args(parsed_args, Some("identity.registered_limit"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.registered_limit data
        let args = &self.registered_limit;
        let mut registered_limit_builder = set::RegisteredLimitBuilder::default();
        if let Some(val) = &args.default_limit {
            registered_limit_builder.default_limit(*val);
        }

        if let Some(val) = &args.description {
            registered_limit_builder.description(Some(val.into()));
        } else if args.no_description {
            registered_limit_builder.description(None);
        }

        if let Some(val) = &args.region_id {
            registered_limit_builder.region_id(Some(val.into()));
        } else if args.no_region_id {
            registered_limit_builder.region_id(None);
        }

        if let Some(val) = &args.resource_name {
            registered_limit_builder.resource_name(val);
        }

        if let Some(val) = &args.service_id {
            registered_limit_builder.service_id(val);
        }

        ep_builder.registered_limit(registered_limit_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<RegisteredLimitResponse>(data)?;
        Ok(())
    }
}
