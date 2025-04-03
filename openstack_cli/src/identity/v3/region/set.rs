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
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::identity::v3::region::set;
use structable_derive::StructTable;

/// Updates a region.
///
/// You can update the description or parent region ID for a region. You cannot
/// update the region ID.
///
/// The following error might occur:
///
/// Relationship:
/// `https://docs.openstack.org/api/openstack-identity/3/rel/region`
///
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
    ///
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
    ///
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
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// To make this region a child of another region, set this parameter to
    /// the ID of the parent region.
    ///
    #[arg(help_heading = "Body parameters", long)]
    parent_id: Option<String>,
}

/// Region response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The region description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID for the region.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// To make this region a child of another region, set this parameter to
    /// the ID of the parent region.
    ///
    #[serde()]
    #[structable(optional)]
    parent_id: Option<String>,
}

impl RegionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Region");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.region data
        let args = &self.region;
        let mut region_builder = set::RegionBuilder::default();
        if let Some(val) = &args.description {
            region_builder.description(val);
        }

        if let Some(val) = &args.parent_id {
            region_builder.parent_id(val);
        }

        ep_builder.region(region_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
