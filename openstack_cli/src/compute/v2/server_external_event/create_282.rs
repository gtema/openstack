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

//! Create ServerExternalEvent command [microversion = 2.82]
//!
//! Wraps invoking of the `v2.1/os-server-external-events` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::compute::v2::server_external_event::create_282;
use openstack_types::compute::v2::server_external_event::response::create::ServerExternalEventResponse;
use serde_json::Value;

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
#[derive(Args)]
#[command(about = "Run Events (microversion = 2.82)")]
pub struct ServerExternalEventCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// List of external events to process.
    ///
    /// Parameter is an array, may be provided multiple times.
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=crate::common::parse_json)]
    events: Vec<Value>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

impl ServerExternalEventCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create ServerExternalEvent");

        let op = OutputProcessor::from_args(
            parsed_args,
            Some("compute.server_external_event"),
            Some("create"),
        );
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_282::Request::builder();
        ep_builder.header(
            http::header::HeaderName::from_static("openstack-api-version"),
            http::header::HeaderValue::from_static("compute 2.82"),
        );

        // Set body parameters
        // Set Request.events data

        let events_builder: Vec<create_282::Events> = self
            .events
            .iter()
            .flat_map(|v| serde_json::from_value::<create_282::Events>(v.to_owned()))
            .collect::<Vec<create_282::Events>>();
        ep_builder.events(events_builder);

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ServerExternalEventResponse>(data)?;
        Ok(())
    }
}
