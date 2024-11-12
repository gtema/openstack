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

//! Show Inventory command
//!
//! Wraps invoking of the `resource_providers/{uuid}/inventories/{resource_class}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::placement::v1::resource_provider::inventory::get;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Normal Response Codes: 200
///
/// Error response codes: itemNotFound(404)
///
#[derive(Args)]
#[command(about = "Show resource provider inventory")]
pub struct InventoryCommand {
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
    /// uuid parameter for
    /// /resource_providers/{uuid}/inventories/{resource_class} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_uuid",
        value_name = "UUID"
    )]
    uuid: String,

    /// resource_class parameter for
    /// /resource_providers/{uuid}/inventories/{resource_class} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_resource_class",
        value_name = "RESOURCE_CLASS"
    )]
    resource_class: String,
}
/// Inventory response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// It is used in determining whether consumption of the resource of the
    /// provider can exceed physical constraints.
    ///
    /// For example, for a vCPU resource with:
    ///
    /// ```text
    /// allocation_ratio = 16.0
    /// total = 8
    ///
    /// ```
    ///
    /// Overall capacity is equal to 128 vCPUs.
    ///
    #[serde()]
    #[structable(optional)]
    allocation_ratio: Option<f32>,

    /// A maximum amount any single allocation against an inventory can have.
    ///
    #[serde()]
    #[structable(optional)]
    max_unit: Option<i32>,

    /// A minimum amount any single allocation against an inventory can have.
    ///
    #[serde()]
    #[structable(optional)]
    min_unit: Option<i32>,

    /// The amount of the resource a provider has reserved for its own use.
    ///
    #[serde()]
    #[structable(optional)]
    reserved: Option<i32>,

    /// A consistent view marker that assists with the management of concurrent
    /// resource provider updates.
    ///
    #[serde()]
    #[structable()]
    resource_provider_generation: i32,

    /// A representation of the divisible amount of the resource that may be
    /// requested. For example, step_size = 5 means that only values divisible
    /// by 5 (5, 10, 15, etc.) can be requested.
    ///
    #[serde()]
    #[structable(optional)]
    step_size: Option<i32>,

    /// The actual amount of the resource that the provider can accommodate.
    ///
    #[serde()]
    #[structable()]
    total: i32,
}

impl InventoryCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Inventory");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.uuid(&self.path.uuid);
        ep_builder.resource_class(&self.path.resource_class);
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