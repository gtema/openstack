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

//! Create MeteringLabel command
//!
//! Wraps invoking of the `v2.0/metering/metering-labels` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::network::v2::metering::metering_label::create;
use openstack_types::network::v2::metering::metering_label::response::create::MeteringLabelResponse;

/// Creates an L3 metering label.
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403
#[derive(Args)]
#[command(about = "Create metering label")]
pub struct MeteringLabelCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `metering_label` object.
    #[command(flatten)]
    metering_label: MeteringLabel,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// MeteringLabel Body data
#[derive(Args, Clone)]
struct MeteringLabel {
    /// A human-readable description for the resource. Default is an empty
    /// string.
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Human-readable name of the resource. Default is an empty string.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Indicates whether this metering label is shared across all projects.
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    shared: Option<bool>,

    /// The ID of the project that owns the resource. Only administrative and
    /// users with advsvc role can specify a project ID other than their own.
    /// You cannot change this value through authorization policies.
    #[arg(help_heading = "Body parameters", long)]
    tenant_id: Option<String>,
}

impl MeteringLabelCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create MeteringLabel");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.metering_label data
        let args = &self.metering_label;
        let mut metering_label_builder = create::MeteringLabelBuilder::default();
        if let Some(val) = &args.name {
            metering_label_builder.name(val);
        }

        if let Some(val) = &args.description {
            metering_label_builder.description(val);
        }

        if let Some(val) = &args.tenant_id {
            metering_label_builder.tenant_id(val);
        }

        if let Some(val) = &args.shared {
            metering_label_builder.shared(*val);
        }

        ep_builder.metering_label(metering_label_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<MeteringLabelResponse>(data)?;
        Ok(())
    }
}
