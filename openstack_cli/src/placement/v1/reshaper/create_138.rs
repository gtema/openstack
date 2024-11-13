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

//! Create Reshaper command [microversion = 1.38]
//!
//! Wraps invoking of the `reshaper` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::placement::v1::reshaper::create_138;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Atomically migrate resource provider inventories and associated
/// allocations. This is used when some of the inventory needs to move from one
/// resource provider to another, such as when a class of inventory moves from
/// a parent provider to a new child provider.
///
/// Normal Response Codes: 204
///
/// Error Response Codes: badRequest(400), conflict(409)
///
#[derive(Args)]
#[command(about = "Reshaper (microversion = 1.38)")]
pub struct ReshaperCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A dictionary of multiple allocations, keyed by consumer uuid. Each
    /// collection of allocations describes the full set of allocations for
    /// each consumer. Each consumer allocations dict is itself a dictionary of
    /// resource allocations keyed by resource provider uuid. An empty
    /// dictionary indicates no change in existing allocations, whereas an
    /// empty `allocations` dictionary **within** a consumer dictionary
    /// indicates that all allocations for that consumer should be deleted.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    allocations: Vec<(String, Value)>,

    /// A dictionary of multiple inventories, keyed by resource provider uuid.
    /// Each inventory describes the desired full inventory for each resource
    /// provider. An empty dictionary causes the inventory for that provider to
    /// be deleted.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    inventories: Vec<(String, Value)>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Reshaper response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ReshaperCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Reshaper");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_138::Request::builder();
        ep_builder.header("OpenStack-API-Version", "placement 1.38");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.allocations data

        ep_builder.allocations(
            self.allocations
                .iter()
                .map(|(k, v)| {
                    serde_json::from_value(v.to_owned())
                        .map(|v: create_138::AllocationsItemStruct| (k, v))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        );

        // Set Request.inventories data

        ep_builder.inventories(
            self.inventories
                .iter()
                .map(|(k, v)| {
                    serde_json::from_value(v.to_owned())
                        .map(|v: create_138::InventoriesItemStruct| (k, v))
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_iter(),
        );

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
