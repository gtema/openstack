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

//! Create QosSpec command
//!
//! Wraps invoking of the `v3/qos-specs` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::block_storage::v3::qos_spec::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct QosSpecCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `qos_specs` object.
    ///
    #[command(flatten)]
    qos_specs: QosSpecs,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// QosSpecs Body data
#[derive(Args, Clone)]
struct QosSpecs {
    /// The name of the QoS specification.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: String,
}

/// QosSpec response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The consumer type.
    ///
    #[serde()]
    #[structable(optional)]
    consumer: Option<String>,

    /// The generated ID for the QoS specification.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the QoS specification.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A `specs` object.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    specs: Option<Value>,
}

impl QosSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create QosSpec");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.qos_specs data
        let args = &self.qos_specs;
        let mut qos_specs_builder = create::QosSpecsBuilder::default();

        qos_specs_builder.name(&args.name);

        ep_builder.qos_specs(qos_specs_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
