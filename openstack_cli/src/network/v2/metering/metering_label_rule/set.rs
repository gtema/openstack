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

//! Set MeteringLabelRule command
//!
//! Wraps invoking of the `v2.0/metering/metering-label-rules/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::network::v2::metering::metering_label_rule::set;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::types::BoolString;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct MeteringLabelRuleCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    metering_label_rule: MeteringLabelRule,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v2.0/metering/metering-label-rules/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// MeteringLabelRule Body data
#[derive(Args, Clone)]
struct MeteringLabelRule {
    #[arg(help_heading = "Body parameters", long)]
    destination_ip_prefix: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    source_ip_prefix: Option<String>,
}

/// MeteringLabelRule response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    #[serde()]
    #[structable(optional)]
    destination_ip_prefix: Option<String>,

    #[serde()]
    #[structable(optional)]
    direction: Option<String>,

    #[serde()]
    #[structable(optional)]
    excluded: Option<BoolString>,

    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    #[serde()]
    #[structable(optional)]
    metering_label_id: Option<String>,

    #[serde()]
    #[structable(optional)]
    remote_ip_prefix: Option<String>,

    #[serde()]
    #[structable(optional)]
    source_ip_prefix: Option<String>,

    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,
}

impl MeteringLabelRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set MeteringLabelRule");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.metering_label_rule data
        let args = &self.metering_label_rule;
        let mut metering_label_rule_builder = set::MeteringLabelRuleBuilder::default();
        if let Some(val) = &args.source_ip_prefix {
            metering_label_rule_builder.source_ip_prefix(val);
        }

        if let Some(val) = &args.destination_ip_prefix {
            metering_label_rule_builder.destination_ip_prefix(val);
        }

        ep_builder.metering_label_rule(metering_label_rule_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
