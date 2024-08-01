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

//! Create ServerExternalEvent command [microversion = 2.76]
//!
//! Wraps invoking of the `v2.1/os-server-external-events` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use openstack_sdk::api::compute::v2::server_external_event::create_276;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates one or more external events, which the API dispatches to the host a
/// server is assigned to. If the server is not currently assigned to a host
/// the event will not be delivered.
///
/// You will receive back the list of events that you submitted, with an
/// updated `code` and `status` indicating their level of success.
///
/// Normal response codes: 200, 207
///
/// A 200 will be returned if all events succeeded, 207 will be returned if any
/// events could not be processed. The `code` attribute for the event will
/// explain further what went wrong.
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "Run Events (microversion = 2.76)")]
pub struct ServerExternalEventCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// List of external events to process.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    events: Vec<Value>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// ServerExternalEvent response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// List of external events to process.
    ///
    #[serde()]
    #[structable(pretty)]
    events: Value,
}

impl ServerExternalEventCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ServerExternalEvent");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_276::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.76");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.events data

        let events_builder: Vec<create_276::Events> = self
            .events
            .iter()
            .flat_map(|v| serde_json::from_value::<create_276::Events>(v.to_owned()))
            .collect::<Vec<create_276::Events>>();
        ep_builder.events(events_builder);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}