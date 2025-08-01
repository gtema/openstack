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

//! Create NetworkSegmentRange command
//!
//! Wraps invoking of the `v2.0/network-segment-ranges` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::network_segment_range::create;
use openstack_types::network::v2::network_segment_range::response::create::NetworkSegmentRangeResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct NetworkSegmentRangeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    network_segment_range: NetworkSegmentRange,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum NetworkType {
    Geneve,
    Gre,
    Vlan,
    Vxlan,
}

/// NetworkSegmentRange Body data
#[derive(Args, Clone)]
struct NetworkSegmentRange {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    maximum: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    minimum: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    network_type: Option<NetworkType>,

    #[arg(help_heading = "Body parameters", long)]
    physical_network: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    project_id: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    shared: Option<bool>,
}

impl NetworkSegmentRangeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create NetworkSegmentRange");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("network.network_segment_range"),
            Some("create"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set body parameters
        // Set Request.network_segment_range data
        let args = &self.network_segment_range;
        let mut network_segment_range_builder = create::NetworkSegmentRangeBuilder::default();
        if let Some(val) = &args.description {
            network_segment_range_builder.description(val);
        }

        if let Some(val) = &args.maximum {
            network_segment_range_builder.maximum(*val);
        }

        if let Some(val) = &args.minimum {
            network_segment_range_builder.minimum(*val);
        }

        if let Some(val) = &args.name {
            network_segment_range_builder.name(val);
        }

        if let Some(val) = &args.network_type {
            let tmp = match val {
                NetworkType::Geneve => create::NetworkType::Geneve,
                NetworkType::Gre => create::NetworkType::Gre,
                NetworkType::Vlan => create::NetworkType::Vlan,
                NetworkType::Vxlan => create::NetworkType::Vxlan,
            };
            network_segment_range_builder.network_type(tmp);
        }

        if let Some(val) = &args.physical_network {
            network_segment_range_builder.physical_network(val);
        }

        if let Some(val) = &args.project_id {
            network_segment_range_builder.project_id(val);
        }

        if let Some(val) = &args.shared {
            network_segment_range_builder.shared(*val);
        }

        ep_builder.network_segment_range(network_segment_range_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<NetworkSegmentRangeResponse>(data)?;
        Ok(())
    }
}
