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

//! List NetworkSegmentRanges command
//!
//! Wraps invoking of the `v2.0/network-segment-ranges` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::BoolString;
use openstack_sdk::api::network::v2::network_segment_range::list;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct NetworkSegmentRangesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    /// description query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    description: Option<String>,

    /// id query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// name query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    name: Option<String>,

    /// network_type query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["geneve","gre","vlan","vxlan"])]
    network_type: Option<String>,

    /// not-tags query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags: Option<Vec<String>>,

    /// not-tags-any query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    not_tags_any: Option<Vec<String>>,

    /// physical_network query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    physical_network: Option<String>,

    /// project_id query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    project_id: Option<String>,

    /// revision_number query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(help_heading = "Query parameters", long)]
    revision_number: Option<String>,

    /// tags query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags: Option<Vec<String>>,

    /// tags-any query parameter for /v2.0/network-segment-ranges API
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Query parameters", long)]
    tags_any: Option<Vec<String>>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// NetworkSegmentRanges response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, wide)]
    available: Option<String>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde(rename = "default")]
    #[structable(optional, title = "default", wide)]
    _default: Option<BoolString>,

    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    maximum: Option<f32>,

    #[serde()]
    #[structable(optional, wide)]
    minimum: Option<f32>,

    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    network_type: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    physical_network: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    project_id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    revision_number: Option<i32>,

    #[serde()]
    #[structable(optional, wide)]
    shared: Option<BoolString>,

    #[serde()]
    #[structable(optional, pretty, wide)]
    tags: Option<Value>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    used: Option<String>,
}

impl NetworkSegmentRangesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List NetworkSegmentRanges");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.query.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.query.network_type {
            ep_builder.network_type(val);
        }
        if let Some(val) = &self.query.physical_network {
            ep_builder.physical_network(val);
        }
        if let Some(val) = &self.query.revision_number {
            ep_builder.revision_number(val);
        }
        if let Some(val) = &self.query.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.query.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.query.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.query.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        if let Some(val) = &self.query.description {
            ep_builder.description(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}