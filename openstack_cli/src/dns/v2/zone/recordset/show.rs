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

//! Show Recordset command
//!
//! Wraps invoking of the `v2/zones/{zone_id}/recordsets/{recordset_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use openstack_sdk::api::dns::v2::zone::find as find_zone;
use openstack_sdk::api::dns::v2::zone::recordset::find;
use openstack_sdk::api::find;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Show an single recordset
///
#[derive(Args)]
#[command(about = "Show a Recordset")]
pub struct RecordsetCommand {
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
    /// Zone resource for which the operation should be performed.
    #[command(flatten)]
    zone: ZoneInput,

    /// recordset_id parameter for
    /// /v2/zones/{zone_id}/recordsets/{recordset_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

/// Zone input select group
#[derive(Args)]
#[group(required = true, multiple = false)]
struct ZoneInput {
    /// Zone Name.
    #[arg(long, help_heading = "Path parameters", value_name = "ZONE_NAME")]
    zone_name: Option<String>,
    /// Zone ID.
    #[arg(long, help_heading = "Path parameters", value_name = "ZONE_ID")]
    zone_id: Option<String>,
}
/// Recordset response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// current action in progress on the resource
    ///
    #[serde()]
    #[structable(optional)]
    action: Option<String>,

    /// Date / Time when resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Description for this recordset
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// ID for the resource
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Links to the resource, and other related resources. When a response has
    /// been broken into pages, we will include a `next` link that should be
    /// followed to retrieve all results
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// DNS Name for the recordset
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// ID for the project that owns the resource
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// A list of data for this recordset. Each item will be a separate record
    /// in Designate These items should conform to the DNS spec for the record
    /// type - e.g. A records must be IPv4 addresses, CNAME records must be a
    /// hostname.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    records: Option<Value>,

    /// The status of the resource.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// TTL (Time to Live) for the recordset.
    ///
    #[serde()]
    #[structable(optional)]
    ttl: Option<i32>,

    /// They RRTYPE of the recordset.
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

    /// ID for the zone that contains this recordset
    ///
    #[serde()]
    #[structable(optional)]
    zone_id: Option<String>,

    /// The name of the zone that contains this recordset
    ///
    #[serde()]
    #[structable(optional)]
    zone_name: Option<String>,
}

impl RecordsetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Recordset");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        // Process path parameter `zone_id`
        if let Some(id) = &self.path.zone.zone_id {
            // zone_id is passed. No need to lookup
            find_builder.zone_id(id);
        } else if let Some(name) = &self.path.zone.zone_name {
            // zone_name is passed. Need to lookup resource
            let mut sub_find_builder = find_zone::Request::builder();
            warn!("Querying zone by name (because of `--zone-name` parameter passed) may not be definite. This may fail in which case parameter `--zone-id` should be used instead.");

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        find_builder.zone_id(id_str.to_owned());
                    }
                    None => {
                        return Err(OpenStackCliError::ResourceAttributeNotString(
                            serde_json::to_string(&val)?,
                        ))
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ))
                }
            };
        }
        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
