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

//! Create MinimumBandwidthRule command
//!
//! Wraps invoking of the `v2.0/qos/policies/{policy_id}/minimum_bandwidth_rules` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::network::v2::qos::policy::minimum_bandwidth_rule::create;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Creates a minimum bandwidth rule for a QoS policy.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 404, 409
///
#[derive(Args)]
#[command(about = "Create minimum bandwidth rule")]
pub struct MinimumBandwidthRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `minimum_bandwidth_rule` object.
    ///
    #[command(flatten)]
    minimum_bandwidth_rule: MinimumBandwidthRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// policy_id parameter for
    /// /v2.0/qos/policies/{policy_id}/minimum_bandwidth_rules/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_policy_id",
        value_name = "POLICY_ID"
    )]
    policy_id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Direction {
    Egress,
    Ingress,
}

/// MinimumBandwidthRule Body data
#[derive(Args, Clone)]
struct MinimumBandwidthRule {
    /// The direction of the traffic to which the QoS rule is applied, as seen
    /// from the point of view of the `port`. Valid values are `egress` and
    /// `ingress`. Default value is `egress`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    direction: Option<Direction>,

    /// The minimum KBPS (kilobits per second) value which should be available
    /// for port.
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_kbps: Option<f32>,
}

/// MinimumBandwidthRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The direction of the traffic to which the QoS rule is applied, as seen
    /// from the point of view of the `port`. Valid values are `egress` and
    /// `ingress`. Default value is `egress`.
    ///
    #[serde()]
    #[structable(optional)]
    direction: Option<String>,

    /// The ID of the QoS minimum bandwidth rule.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The minimum KBPS (kilobits per second) value which should be available
    /// for port.
    ///
    #[serde()]
    #[structable(optional)]
    min_kbps: Option<f32>,
}

impl MinimumBandwidthRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create MinimumBandwidthRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.policy_id(&self.path.policy_id);
        // Set query parameters
        // Set body parameters
        // Set Request.minimum_bandwidth_rule data
        let args = &self.minimum_bandwidth_rule;
        let mut minimum_bandwidth_rule_builder = create::MinimumBandwidthRuleBuilder::default();
        if let Some(val) = &args.min_kbps {
            minimum_bandwidth_rule_builder.min_kbps(*val);
        }

        if let Some(val) = &args.direction {
            let tmp = match val {
                Direction::Egress => create::Direction::Egress,
                Direction::Ingress => create::Direction::Ingress,
            };
            minimum_bandwidth_rule_builder.direction(tmp);
        }

        ep_builder.minimum_bandwidth_rule(minimum_bandwidth_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
