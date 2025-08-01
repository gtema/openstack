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

//! Set MinimumBandwidthRule command
//!
//! Wraps invoking of the `v2.0/qos/policies/{policy_id}/minimum_bandwidth_rules/{id}` with `PUT` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::qos::policy::minimum_bandwidth_rule::set;
use openstack_types::network::v2::qos::policy::minimum_bandwidth_rule::response::set::MinimumBandwidthRuleResponse;

/// Updates a minimum bandwidth rule for a QoS policy.
///
/// Note that the rule cannot be updated, and the update is rejected with error
/// code 501, if there is any bound port referring to the rule via the qos
/// policy.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 404, 501
#[derive(Args)]
#[command(about = "Update minimum bandwidth rule")]
pub struct MinimumBandwidthRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `minimum_bandwidth_rule` object.
    #[command(flatten)]
    minimum_bandwidth_rule: MinimumBandwidthRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for
    /// /v2.0/qos/policies/{policy_id}/minimum_bandwidth_rules/{id} API
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,

    /// policy_id parameter for
    /// /v2.0/qos/policies/{policy_id}/minimum_bandwidth_rules/{id} API
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
    /// `ingress`.
    #[arg(help_heading = "Body parameters", long)]
    direction: Option<Direction>,

    /// The minimum KBPS (kilobits per second) value which should be available
    /// for port.
    #[arg(help_heading = "Body parameters", long)]
    min_kbps: Option<i32>,
}

impl MinimumBandwidthRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set MinimumBandwidthRule");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("network.qos/policy/minimum_bandwidth_rule"),
            Some("set"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        ep_builder.id(&self.path.id);
        ep_builder.policy_id(&self.path.policy_id);

        // Set body parameters
        // Set Request.minimum_bandwidth_rule data
        let args = &self.minimum_bandwidth_rule;
        let mut minimum_bandwidth_rule_builder = set::MinimumBandwidthRuleBuilder::default();
        if let Some(val) = &args.direction {
            let tmp = match val {
                Direction::Egress => set::Direction::Egress,
                Direction::Ingress => set::Direction::Ingress,
            };
            minimum_bandwidth_rule_builder.direction(tmp);
        }

        if let Some(val) = &args.min_kbps {
            minimum_bandwidth_rule_builder.min_kbps(*val);
        }

        ep_builder.minimum_bandwidth_rule(minimum_bandwidth_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<MinimumBandwidthRuleResponse>(data)?;
        Ok(())
    }
}
