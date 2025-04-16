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

//! Create PacketRateLimitRule command
//!
//! Wraps invoking of the `v2.0/policies/{policy_id}/packet_rate_limit_rules` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use clap::ValueEnum;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::policy::packet_rate_limit_rule::create;
use openstack_types::network::v2::policy::packet_rate_limit_rule::response::create::PacketRateLimitRuleResponse;

/// Command without description in OpenAPI
#[derive(Args)]
pub struct PacketRateLimitRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    packet_rate_limit_rule: PacketRateLimitRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// policy_id parameter for
    /// /v2.0/policies/{policy_id}/packet_rate_limit_rules/{id} API
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

/// PacketRateLimitRule Body data
#[derive(Args, Clone)]
struct PacketRateLimitRule {
    #[arg(help_heading = "Body parameters", long)]
    direction: Option<Direction>,

    #[arg(help_heading = "Body parameters", long)]
    max_burst_kpps: Option<i32>,

    #[arg(help_heading = "Body parameters", long)]
    max_kpps: Option<i32>,
}

impl PacketRateLimitRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create PacketRateLimitRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        ep_builder.policy_id(&self.path.policy_id);
        // Set query parameters
        // Set body parameters
        // Set Request.packet_rate_limit_rule data
        let args = &self.packet_rate_limit_rule;
        let mut packet_rate_limit_rule_builder = create::PacketRateLimitRuleBuilder::default();
        if let Some(val) = &args.max_kpps {
            packet_rate_limit_rule_builder.max_kpps(*val);
        }

        if let Some(val) = &args.max_burst_kpps {
            packet_rate_limit_rule_builder.max_burst_kpps(*val);
        }

        if let Some(val) = &args.direction {
            let tmp = match val {
                Direction::Egress => create::Direction::Egress,
                Direction::Ingress => create::Direction::Ingress,
            };
            packet_rate_limit_rule_builder.direction(tmp);
        }

        ep_builder.packet_rate_limit_rule(packet_rate_limit_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<PacketRateLimitRuleResponse>(data)?;
        Ok(())
    }
}
