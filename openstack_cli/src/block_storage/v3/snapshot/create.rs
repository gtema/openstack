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
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::output::OutputProcessor;

use crate::common::parse_key_val;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::block_storage::v3::snapshot::create;
use openstack_types::block_storage::v3::snapshot::response::create::SnapshotResponse;

/// Creates a new snapshot.
#[derive(Args)]
pub struct SnapshotCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// A `snapshot` object.
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
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// Set explicit NULL for the description
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "description")]
    no_description: bool,

    /// The name of the snapshot.
    #[arg(help_heading = "Body parameters", long)]
    display_name: Option<String>,

    /// Set explicit NULL for the display_name
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "display_name")]
    no_display_name: bool,

    /// Indicates whether to snapshot, even if the volume is attached. Default
    /// is `false`. See [valid boolean values](#valid-boolean-values)
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    force: Option<bool>,

    /// One or more metadata key and value pairs for the snapshot.
    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// The name of the snapshot.
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Set explicit NULL for the name
    #[arg(help_heading = "Body parameters", long, action = clap::ArgAction::SetTrue, conflicts_with = "name")]
    no_name: bool,

    /// The UUID of the volume.
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

impl SnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Snapshot");

        let op =
            OutputProcessor::from_args(parsed_args, Some("block-storage.snapshot"), Some("create"));
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set body parameters
        // Set Request.snapshot data
        let args = &self.snapshot;
        let mut snapshot_builder = create::SnapshotBuilder::default();
        if let Some(val) = &args.description {
            snapshot_builder.description(Some(val.into()));
        } else if args.no_description {
            snapshot_builder.description(None);
        }

        if let Some(val) = &args.display_name {
            snapshot_builder.display_name(Some(val.into()));
        } else if args.no_display_name {
            snapshot_builder.display_name(None);
        }

        if let Some(val) = &args.force {
            snapshot_builder.force(*val);
        }

        if let Some(val) = &args.metadata {
            snapshot_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.name {
            snapshot_builder.name(Some(val.into()));
        } else if args.no_name {
            snapshot_builder.name(None);
        }

        snapshot_builder.volume_id(args.volume_id.clone());

        ep_builder.snapshot(snapshot_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<SnapshotResponse>(data)?;
        Ok(())
    }
}
