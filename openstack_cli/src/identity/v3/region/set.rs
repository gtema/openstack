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

//! Set Region command
//!
//! Wraps invoking of the `v3/regions/{region_id}` with `PATCH` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::region::set;
use openstack_types::identity::v3::region::response::set::RegionResponse;

/// Updates a region.
///
/// You can update the description or parent region ID for a region. You cannot
/// update the region ID.
///
/// The following error might occur:
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/region`
#[derive(Args)]
#[command(about = "Update region")]
pub struct RegionCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `region` object
    #[command(flatten)]
    region: Region,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// region_id parameter for /v3/regions/{region_id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Region Body data
#[derive(Args, Clone)]
struct Region {
    /// The region description.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Set explicit NULL for the description
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "description")]
    no_description: bool,

    /// The region ID.
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// Set explicit NULL for the id
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "id")]
    no_id: bool,

    /// To make this region a child of another region, set this parameter to
    /// the ID of the parent region.
    #[arg(help_heading = "Body parameters", long)]
    parent_region_id: Option<String>,

    /// Set explicit NULL for the parent_region_id
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "parent_region_id")]
    no_parent_region_id: bool,
}

impl RegionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Region");

        let op = OutputProcessor::from_args(parsed_args, Some("identity.region"), Some("set"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        ep_builder.id(&self.path.id);

        // Set body parameters
        // Set Request.region data
        let args = &self.region;
        let mut region_builder = set::RegionBuilder::default();
        if let Some(val) = &args.description {
            region_builder.description(Some(val.into()));
        } else if args.no_description {
            region_builder.description(None);
        }

        if let Some(val) = &args.id {
            region_builder.id(Some(val.into()));
        } else if args.no_id {
            region_builder.id(None);
        }

        if let Some(val) = &args.parent_region_id {
            region_builder.parent_region_id(Some(val.into()));
        } else if args.no_parent_region_id {
            region_builder.parent_region_id(None);
        }

        ep_builder.region(region_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<RegionResponse>(data)?;
        Ok(())
    }
}
