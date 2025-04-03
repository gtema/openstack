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

//! Set SecurityGroupRule command
//!
//! Wraps invoking of the `v2.0/security-group-rules/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::network::v2::security_group_rule::set;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::types::BoolString;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct SecurityGroupRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    security_group_rule: SecurityGroupRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/security-group-rules/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// SecurityGroupRule Body data
#[derive(Args, Clone)]
struct SecurityGroupRule {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,
}

/// SecurityGroupRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    belongs_to_default_sg: Option<BoolString>,

    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    #[serde()]
    #[structable(optional)]
    direction: Option<String>,

    #[serde()]
    #[structable(optional)]
    ethertype: Option<String>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    normalized_cidr: Option<String>,

    #[serde()]
    #[structable(optional)]
    port_range_max: Option<i32>,

    #[serde()]
    #[structable(optional)]
    port_range_min: Option<i32>,

    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    #[serde()]
    #[structable(optional)]
    remote_address_group_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    remote_group_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    remote_ip_prefix: Option<String>,

    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    #[serde()]
    #[structable(optional)]
    security_group_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl SecurityGroupRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set SecurityGroupRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.security_group_rule data
        let args = &self.security_group_rule;
        let mut security_group_rule_builder = set::SecurityGroupRuleBuilder::default();
        if let Some(val) = &args.description {
            security_group_rule_builder.description(val);
        }

        ep_builder.security_group_rule(security_group_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
