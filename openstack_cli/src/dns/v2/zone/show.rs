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

//! Show Zone command
//!
//! Wraps invoking of the `v2/zones/{zone_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::dns::v2::zone::find;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Show a zone
///
#[derive(Args)]
#[command(about = "Show a Zone")]
pub struct ZoneCommand {
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
struct PathParameters {
    /// zone_id parameter for /v2/zones/{zone_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Zone response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// current action in progress on the resource
    ///
    #[serde()]
    #[structable(optional)]
    action: Option<String>,

    /// Key:Value pairs of information about this zone, and the pool the user
    /// would like to place the zone in. This information can be used by the
    /// scheduler to place zones on the correct pool.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    attributes: Option<Value>,

    /// Date / Time when resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Description for this zone
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// e-mail for the zone. Used in SOA records for the zone
    ///
    #[serde()]
    #[structable(optional)]
    email: Option<String>,

    /// ID for the resource
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Mandatory for secondary zones. The servers to slave from to get DNS
    /// information
    ///
    #[serde()]
    #[structable(optional, pretty)]
    masters: Option<Value>,

    /// DNS Name for the zone
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// ID for the pool hosting this zone
    ///
    #[serde()]
    #[structable(optional)]
    pool_id: Option<String>,

    /// ID for the project that owns the resource
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// current serial number for the zone
    ///
    #[serde()]
    #[structable(optional)]
    serial: Option<i32>,

    /// True if the zone is shared with another project.
    ///
    /// **New in version 2.1**
    ///
    #[serde()]
    #[structable(optional)]
    shared: Option<bool>,

    /// The status of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// For secondary zones. The last time an update was retrieved from the
    /// master servers
    ///
    #[serde()]
    #[structable(optional)]
    transferred_at: Option<String>,

    /// TTL (Time to Live) for the zone.
    ///
    #[serde()]
    #[structable(optional)]
    ttl: Option<i32>,

    /// Type of zone. PRIMARY is controlled by Designate, SECONDARY zones are
    /// slaved from another DNS Server. Defaults to PRIMARY
    ///
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// Date / Time when resource last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// Version of the resource
    ///
    #[serde()]
    #[structable(optional)]
    version: Option<i32>,
}

impl ZoneCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Zone");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}