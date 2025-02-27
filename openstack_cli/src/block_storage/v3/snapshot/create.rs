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

//! Create Snapshot command
//!
//! Wraps invoking of the `v3/snapshots` with `POST` method

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
use openstack_sdk::api::block_storage::v3::snapshot::create;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a new snapshot.
///
#[derive(Args)]
pub struct SnapshotCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `snapshot` object.
    ///
    #[command(flatten)]
    snapshot: Snapshot,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Snapshot Body data
#[derive(Args, Clone)]
struct Snapshot {
    /// A description for the snapshot. Default is `None`.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The name of the snapshot.
    ///
    #[arg(help_heading = "Body parameters", long)]
    display_name: Option<String>,

    /// Indicates whether to snapshot, even if the volume is attached. Default
    /// is `false`. See [valid boolean values](#valid-boolean-values)
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    force: Option<bool>,

    /// One or more metadata key and value pairs for the snapshot.
    ///
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The name of the snapshot.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// The UUID of the volume.
    ///
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

/// Snapshot response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    ///
    /// **New in version 3.65**
    ///
    #[serde()]
    #[structable(optional)]
    consumes_quota: Option<bool>,

    /// The total count of requested resource before pagination is applied.
    ///
    #[serde()]
    #[structable(optional)]
    count: Option<i32>,

    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    ///
    #[serde()]
    #[structable()]
    created_at: String,

    /// A description for the snapshot.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the group snapshot.
    ///
    /// **New in version 3.14**
    ///
    #[serde()]
    #[structable(optional)]
    group_snapshot_id: Option<String>,

    /// The UUID of the object.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// One or more metadata key and value pairs for the snapshot.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    metadata: Option<Value>,

    /// The name of the snapshot. Default is `None`.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// A percentage value for the build progress.
    ///
    #[serde(rename = "os-extended-snapshot-attributes:progress")]
    #[structable(optional, title = "os-extended-snapshot-attributes:progress")]
    os_extended_snapshot_attributes_progress: Option<String>,

    /// The UUID of the owning project.
    ///
    #[serde(rename = "os-extended-snapshot-attributes:project_id")]
    #[structable(optional, title = "os-extended-snapshot-attributes:project_id")]
    os_extended_snapshot_attributes_project_id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    ///
    #[serde()]
    #[structable()]
    size: i64,

    /// The status for the snapshot.
    ///
    #[serde()]
    #[structable()]
    status: String,

    /// The date and time when the resource was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
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

impl SnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Snapshot");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.snapshot data
        let args = &self.snapshot;
        let mut snapshot_builder = create::SnapshotBuilder::default();
        if let Some(val) = &args.name {
            snapshot_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.display_name {
            snapshot_builder.display_name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            snapshot_builder.description(Some(val.into()));
        }

        snapshot_builder.volume_id(args.volume_id.clone());

        if let Some(val) = &args.force {
            snapshot_builder.force(*val);
        }

        if let Some(val) = &args.metadata {
            snapshot_builder.metadata(val.iter().cloned());
        }

        ep_builder.snapshot(snapshot_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
