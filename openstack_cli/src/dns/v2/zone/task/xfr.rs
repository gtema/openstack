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

//! Action Task command
//!
//! Wraps invoking of the `v2/zones/{zone_id}/tasks/xfr` with `POST` method

use clap::Args;
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use eyre::OptionExt;
use eyre::eyre;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::dns::v2::zone::find as find_zone;
use openstack_sdk::api::dns::v2::zone::task::xfr;
use openstack_sdk::api::find_by_name;
use openstack_types::dns::v2::zone::task::response::xfr::TaskResponse;
use serde_json::Value;
use tracing::warn;

/// Request of the zones/zone_id/tasks/xfr:post operation
#[derive(Args)]
#[command(about = "Manually Trigger an Update of a Secondary Zone")]
pub struct TaskCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, Value)>>,
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

impl TaskCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Task");

        let op = OutputProcessor::from_args(parsed_args, Some("dns.zone/task"), Some("xfr"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = xfr::Request::builder();

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

        // Set body parameters
        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<TaskResponse>(data)?;
        Ok(())
    }
}
