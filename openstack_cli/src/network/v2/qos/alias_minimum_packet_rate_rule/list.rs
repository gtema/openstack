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

//! List AliasMinimumPacketRateRules command
//!
//! Wraps invoking of the `v2.0/qos/alias-minimum-packet-rate-rules` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::network::v2::qos::alias_minimum_packet_rate_rule::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct AliasMinimumPacketRateRulesCommand {
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
    /// direction query parameter for /v2.0/qos/alias-minimum-packet-rate-rules
    /// API
    ///
    #[arg(help_heading = "Query parameters", long, value_parser = ["any","egress","ingress"])]
    direction: Option<String>,

    /// id query parameter for /v2.0/qos/alias-minimum-packet-rate-rules API
    ///
    #[arg(help_heading = "Query parameters", long)]
    id: Option<String>,

    /// min_kpps query parameter for /v2.0/qos/alias-minimum-packet-rate-rules
    /// API
    ///
    #[arg(help_heading = "Query parameters", long)]
    min_kpps: Option<f32>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// AliasMinimumPacketRateRules response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional, wide)]
    direction: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional, wide)]
    min_kpps: Option<f32>,
}

impl AliasMinimumPacketRateRulesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AliasMinimumPacketRateRules");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.query.min_kpps {
            ep_builder.min_kpps(*val);
        }
        if let Some(val) = &self.query.direction {
            ep_builder.direction(val);
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
