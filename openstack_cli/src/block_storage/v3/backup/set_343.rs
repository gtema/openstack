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

//! Set Backup command [microversion = 3.43]
//!
//! Wraps invoking of the `v3/backups/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_key_val;
use openstack_sdk::api::block_storage::v3::backup::find;
use openstack_sdk::api::block_storage::v3::backup::set_343;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Update a backup.
///
#[derive(Args)]
pub struct BackupCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    backup: Option<Backup>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/backups/{id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}
/// Backup Body data
#[derive(Args, Clone)]
struct Backup {
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// Backup response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The name of the availability zone.
    ///
    #[serde()]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The container name or null.
    ///
    #[serde()]
    #[structable(optional)]
    container: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is ISO 8601
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The time when the data on the volume was first saved. If it is a backup
    /// from volume, it will be the same as created_at for a backup. If it is a
    /// backup from a snapshot, it will be the same as created_at for the
    /// snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    data_timestamp: Option<String>,

    /// The backup description or null.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// If the backup failed, the reason for the failure. Otherwise, null.
    ///
    #[serde()]
    #[structable(optional)]
    fail_reason: Option<String>,

    /// If this value is true, there are other backups depending on this
    /// backup.
    ///
    #[serde()]
    #[structable(optional)]
    has_dependent_backups: Option<bool>,

    /// The UUID of the backup.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Indicates whether the backup mode is incremental. If this value is
    /// true, the backup mode is incremental. If this value is false, the
    /// backup mode is full.
    ///
    #[serde()]
    #[structable(optional)]
    is_incremental: Option<bool>,

    /// Links to the resources in question. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    links: Option<Value>,

    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the resource.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The backup name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The number of objects in the backup.
    ///
    #[serde()]
    #[structable(optional)]
    object_count: Option<i32>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable(optional)]
    size: Option<i64>,

    /// The UUID of the source volume snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    snapshot_id: Option<String>,

    /// The backup status. Refer to Backup statuses table for the possible
    /// status value.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is ISO 8601
    ///
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[structable(optional)]
    volume_id: Option<String>,
}

impl BackupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Backup");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        find_builder.header("OpenStack-API-Version", "volume 3.43");
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        let mut ep_builder = set_343::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.43");

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.backup data
        if let Some(lbackup) = &self.backup {
            let mut backup_builder = set_343::BackupBuilder::default();
            if let Some(val) = &lbackup.name {
                backup_builder.name(Some(val.into()));
            }
            if let Some(val) = &lbackup.description {
                backup_builder.description(Some(val.into()));
            }
            if let Some(val) = &lbackup.metadata {
                backup_builder.metadata(val.iter().cloned());
            }
            ep_builder.backup(backup_builder.build().expect("A valid object"));
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
