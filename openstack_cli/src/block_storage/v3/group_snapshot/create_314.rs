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

//! Create GroupSnapshot command [microversion = 3.14]
//!
//! Wraps invoking of the `v3/group_snapshots` with `POST` method

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
use openstack_sdk::api::block_storage::v3::group_snapshot::create_314;
use structable_derive::StructTable;

/// Create a new group_snapshot.
///
#[derive(Args)]
pub struct GroupSnapshotCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The group snapshot.
    ///
    #[command(flatten)]
    group_snapshot: GroupSnapshot,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// GroupSnapshot Body data
#[derive(Args, Clone)]
struct GroupSnapshot {
    /// The group snapshot description.
    ///
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    /// The ID of the group.
    ///
    #[arg(help_heading = "Body parameters", long)]
    group_id: String,

    /// The group snapshot name.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,
}

/// GroupSnapshot response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
    /// The date and time when the resource was created.
    ///
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The group snapshot description.
    ///
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// The ID of the group.
    ///
    #[serde()]
    #[structable(optional)]
    group_id: Option<String>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable(optional)]
    group_type: Option<String>,

    /// The group type ID.
    ///
    #[serde()]
    #[structable(optional)]
    group_type_id: Option<String>,

    /// The ID of the group snapshot.
    ///
    #[serde()]
    #[structable()]
    id: String,

    /// The group snapshot name.
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The UUID of the volume group project.
    ///
    #[serde()]
    #[structable(optional)]
    project_id: Option<String>,

    /// The status of the generic group snapshot.
    ///
    #[serde()]
    #[structable()]
    status: String,
}

impl GroupSnapshotCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create GroupSnapshot");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_314::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.14");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.group_snapshot data
        let args = &self.group_snapshot;
        let mut group_snapshot_builder = create_314::GroupSnapshotBuilder::default();

        group_snapshot_builder.group_id(&args.group_id);

        if let Some(val) = &args.name {
            group_snapshot_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            group_snapshot_builder.description(Some(val.into()));
        }

        ep_builder.group_snapshot(group_snapshot_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
