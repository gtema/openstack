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

//! List Flavors command
//!
//! Wraps invoking of the `v2.1/flavors/detail` with `GET` method

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
use openstack_sdk::api::compute::v2::flavor::list_detailed;
use openstack_sdk::api::{Pagination, paged};
use openstack_sdk::types::IntString;
use serde_json::Value;
use structable_derive::StructTable;

/// Lists flavors with details.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403)
///
#[derive(Args)]
#[command(about = "List Flavors With Details")]
pub struct FlavorsCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {
    #[arg(help_heading = "Query parameters", long)]
    is_public: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    limit: Option<i32>,

    #[arg(help_heading = "Query parameters", long)]
    marker: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    min_disk: Option<String>,

    #[arg(help_heading = "Query parameters", long)]
    min_ram: Option<String>,

    #[arg(help_heading = "Query parameters", long, value_parser = ["asc","desc"])]
    sort_dir: Option<String>,

    #[arg(help_heading = "Query parameters", long, value_parser = ["created_at","description","disabled","ephemeral_gb","flavorid","id","is_public","memory_mb","name","root_gb","rxtx_factor","swap","updated_at","vcpu_weight","vcpus"])]
    sort_key: Option<String>,
}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Flavors response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The description of the flavor.
    ///
    /// **New in version 2.55**
    ///
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The size of the root disk that will be created in GiB. If 0 the root
    /// disk will be set to exactly the size of the image used to deploy the
    /// instance. However, in this case the scheduler cannot select the compute
    /// host based on the virtual image size. Therefore, 0 should only be used
    /// for volume booted instances or for testing purposes. Volume-backed
    /// instances can be enforced for flavors with zero root disk via the
    /// `os_compute_api:servers:create:zero_disk_flavor` policy rule.
    ///
    #[serde()]
    #[structable(wide)]
    disk: i32,

    /// A dictionary of the flavor’s extra-specs key-and-value pairs. This will
    /// only be included if the user is allowed by policy to index flavor
    /// extra_specs.
    ///
    /// **New in version 2.61**
    ///
    #[serde()]
    #[structable(optional, pretty, wide)]
    extra_specs: Option<Value>,

    /// The ID of the flavor. While people often make this look like an int,
    /// this is really a string.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// The display name of a flavor.
    ///
    #[serde()]
    #[structable()]
    name: String,

    #[serde(rename = "os-flavor-access:is_public")]
    #[structable(pretty, title = "os-flavor-access:is_public", wide)]
    os_flavor_access_is_public: Value,

    /// Whether or not the flavor has been administratively disabled. This is
    /// an artifact of the legacy v2 API and will always be set to `false`.
    /// There is currently no way to disable a flavor and set this to `true`.
    ///
    #[serde(rename = "OS-FLV-DISABLED:disabled")]
    #[structable(title = "OS-FLV-DISABLED:disabled", wide)]
    os_flv_disabled_disabled: bool,

    /// The size of the ephemeral disk that will be created, in GiB. Ephemeral
    /// disks may be written over on server state changes. So should only be
    /// used as a scratch space for applications that are aware of its
    /// limitations. Defaults to 0.
    ///
    #[serde(rename = "OS-FLV-EXT-DATA:ephemeral")]
    #[structable(title = "OS-FLV-EXT-DATA:ephemeral", wide)]
    os_flv_ext_data_ephemeral: i32,

    /// The amount of RAM a flavor has, in MiB.
    ///
    #[serde()]
    #[structable(wide)]
    ram: i32,

    #[serde()]
    #[structable(pretty, wide)]
    rxtx_factor: Value,

    /// The size of a dedicated swap disk that will be allocated, in MiB. If 0
    /// (the default), no dedicated swap disk will be created. Currently, the
    /// empty string (‘’) is used to represent 0. As of microversion 2.75
    /// default return value of swap is 0 instead of empty string.
    ///
    #[serde()]
    #[structable(wide)]
    swap: IntString,

    /// The number of virtual CPUs that will be allocated to the server.
    ///
    #[serde()]
    #[structable(wide)]
    vcpus: i32,
}

impl FlavorsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Flavors");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.query.is_public {
            ep_builder.is_public(val);
        }
        if let Some(val) = &self.query.min_ram {
            ep_builder.min_ram(val);
        }
        if let Some(val) = &self.query.min_disk {
            ep_builder.min_disk(val);
        }
        if let Some(val) = &self.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
