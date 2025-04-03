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

//! Set AliasMinimumBandwidthRule command
//!
//! Wraps invoking of the `v2.0/qos/alias-minimum-bandwidth-rules/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::qos::alias_minimum_bandwidth_rule::set;
use openstack_sdk::types::IntString;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct AliasMinimumBandwidthRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    alias_minimum_bandwidth_rule: AliasMinimumBandwidthRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/qos/alias-minimum-bandwidth-rules/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Direction {
    Egress,
    Ingress,
}

/// AliasMinimumBandwidthRule Body data
#[derive(Args, Clone)]
struct AliasMinimumBandwidthRule {
    #[arg(help_heading = "Body parameters", long)]
    direction: Option<Direction>,

    #[arg(help_heading = "Body parameters", long)]
    min_kbps: Option<i32>,
}

/// AliasMinimumBandwidthRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    direction: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    min_kbps: Option<IntString>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,
}

impl AliasMinimumBandwidthRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set AliasMinimumBandwidthRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.alias_minimum_bandwidth_rule data
        let args = &self.alias_minimum_bandwidth_rule;
        let mut alias_minimum_bandwidth_rule_builder =
            set::AliasMinimumBandwidthRuleBuilder::default();
        if let Some(val) = &args.min_kbps {
            alias_minimum_bandwidth_rule_builder.min_kbps(*val);
        }

        if let Some(val) = &args.direction {
            let tmp = match val {
                Direction::Egress => set::Direction::Egress,
                Direction::Ingress => set::Direction::Ingress,
            };
            alias_minimum_bandwidth_rule_builder.direction(tmp);
        }

        ep_builder
            .alias_minimum_bandwidth_rule(alias_minimum_bandwidth_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
