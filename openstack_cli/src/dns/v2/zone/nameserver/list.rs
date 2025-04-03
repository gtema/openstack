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

//! List Nameservers command
//!
//! Wraps invoking of the `v2/zones/{zone_id}/nameservers` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::dns::v2::zone::find as find_zone;
use openstack_sdk::api::dns::v2::zone::nameserver::list;
use openstack_sdk::api::find_by_name;
use structable_derive::StructTable;
use tracing::warn;

/// Show the nameservers for a zone
///
#[derive(Args)]
#[command(about = "Get the Name Servers for a Zone")]
pub struct NameserversCommand {
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
/// Nameservers response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The hostname of the nameserver that the zone should be delegated to
    ///
    #[serde()]
    #[structable(optional)]
    hostname: Option<String>,

    /// The priority of the nameserver. This is used to determine the order of
    /// the the nameserver listings, and which server is used in the SOA record
    /// for the zone.
    ///
    #[serde()]
    #[structable(optional)]
    priority: Option<i32>,
}

impl NameserversCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Nameservers");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list::Request::builder();

        // Set path parameters

        // Process path parameter `zone_id`
        if let Some(id) = &self.path.zone.zone_id {
            // zone_id is passed. No need to lookup
            ep_builder.zone_id(id);
        } else if let Some(name) = &self.path.zone.zone_name {
            // zone_name is passed. Need to lookup resource
            let mut sub_find_builder = find_zone::Request::builder();
            warn!(
                "Querying zone by name (because of `--zone-name` parameter passed) may not be definite. This may fail in which case parameter `--zone-id` should be used instead."
            );

            sub_find_builder.id(name);
            let find_ep = sub_find_builder
                .build()
                .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
            let find_data: serde_json::Value = find_by_name(find_ep).query_async(client).await?;
            // Try to extract resource id
            match find_data.get("id") {
                Some(val) => match val.as_str() {
                    Some(id_str) => {
                        ep_builder.zone_id(id_str.to_owned());
                    }
                    None => {
                        return Err(OpenStackCliError::ResourceAttributeNotString(
                            serde_json::to_string(&val)?,
                        ));
                    }
                },
                None => {
                    return Err(OpenStackCliError::ResourceAttributeMissing(
                        "id".to_string(),
                    ));
                }
            };
        }
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
