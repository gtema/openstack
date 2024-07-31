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

//! List Services command
//!
//! Wraps invoking of the `v2.1/os-services` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::IntString;
use openstack_sdk::api::compute::v2::service::list;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Lists all running Compute services.
///
/// Provides details why any services were disabled.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Compute Services")]
pub struct ServicesCommand {
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
    #[arg(help_heading = "Query parameters", long)]
    binary: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    host: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Services response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The binary name of the service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    binary: Option<String>,

    /// The reason for disabling a service.
    ///
    #[serde()]
    #[structable(optional, wide)]
    disabled_reason: Option<String>,

    /// Whether or not this service was forced down manually by an
    /// administrator after the service was fenced. This value is useful to
    /// know that some 3rd party has verified the service should be marked
    /// down.
    ///
    #[serde()]
    #[structable(optional, wide)]
    forced_down: Option<bool>,

    /// The name of the host.
    ///
    #[serde()]
    #[structable(optional, wide)]
    host: Option<String>,

    /// The id of the service as a uuid.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<IntString>,

    /// Service name
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The state of the service. One of `up` or `down`.
    ///
    #[serde()]
    #[structable(optional)]
    state: Option<String>,

    /// The status of the service. One of `enabled` or `disabled`.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The availability zone name.
    ///
    #[serde()]
    #[structable(optional, wide)]
    zone: Option<String>,
}

impl ServicesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Services");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.host {
            ep_builder.host(val);
        }
        if let Some(val) = &self.query.binary {
            ep_builder.binary(val);
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
