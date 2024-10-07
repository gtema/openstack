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

//! Show Share command
//!
//! Wraps invoking of the `v2/zones/{zone_id}/shares/{zone_share_id}` with `GET` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use eyre::OptionExt;
use openstack_sdk::api::dns::v2::zone::find as find_zone;
use openstack_sdk::api::dns::v2::zone::share::get;
use openstack_sdk::api::find_by_name;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;
use tracing::warn;

/// Show a single zone share.
///
/// **New in version 2.1**
///
#[derive(Args)]
#[command(about = "Show a Zone Share")]
pub struct ShareCommand {
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

    /// zone_share_id parameter for /v2/zones/{zone_id}/shares/{zone_share_id}
    /// API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_zone_share_id",
        value_name = "ZONE_SHARE_ID"
    )]
    zone_share_id: String,
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
/// Share response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Date / Time when resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

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

    /// ID for the project that owns the resource
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The project ID the zone will be shared with.
    ///
    /// **New in version 2.1**
    ///
    #[serde()]
    #[structable(optional)]
    target_project_id: Option<String>,

    /// Date / Time when resource last updated.
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,
}

impl ShareCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Share");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = get::Request::builder();

        // Set path parameters

        // Process path parameter `zone_id`
        if let Some(id) = &self.path.zone.zone_id {
            // zone_id is passed. No need to lookup
            ep_builder.zone_id(id);
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
                        ep_builder.zone_id(id_str.to_owned());
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
        ep_builder.zone_share_id(&self.path.zone_share_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
