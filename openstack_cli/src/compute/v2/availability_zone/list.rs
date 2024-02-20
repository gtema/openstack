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

//! List AvailabilityZones command
//!
//! Wraps invoking of the `v2.1/os-availability-zone/detail` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::compute::v2::availability_zone::list_detailed;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Gets detailed availability zone information.
/// Policy defaults enable only users with the administrative role to perform
/// this operation.
/// Cloud providers can change these permissions through the `policy.json`
/// file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403)
#[derive(Args)]
#[command(about = "Get Detailed Availability Zone Information")]
pub struct AvailabilityZonesCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// AvailabilityZones response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The availability zone name.
    #[serde(rename = "zoneName")]
    #[structable(optional, title = "zoneName", wide)]
    zone_name: Option<String>,

    /// The current state of the availability zone.
    #[serde(rename = "zoneState")]
    #[structable(optional, title = "zoneState", wide)]
    zone_state: Option<ResponseZoneState>,

    /// An object containing a list of host information. The host information
    /// is comprised
    /// of host and service objects. The service object returns three
    /// parameters representing
    /// the states of the service: `active`, `available`, and `updated\_at`.
    #[serde()]
    #[structable(optional)]
    hosts: Option<HashMapStringValue>,
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseZoneState {
    available: Option<bool>,
}

impl fmt::Display for ResponseZoneState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "available={}",
            self.available
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
/// HashMap of Value response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

impl AvailabilityZonesCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List AvailabilityZones");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}