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

//! Create SnapshotManage command
//!
//! Wraps invoking of the `v3/os-snapshot-manage` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use crate::common::parse_json;
use crate::common::parse_key_val;
use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::snapshot_manage::create;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Instruct Cinder to manage a storage snapshot object.
///
/// Manages an existing backend storage snapshot object (e.g. a Linux logical
/// volume or a SAN disk) by creating the Cinder objects required to manage it,
/// and possibly renaming the backend storage snapshot object (driver
/// dependent).
///
/// From an API perspective, this operation behaves very much like a snapshot
/// creation operation.
///
/// Required HTTP Body:
///
/// ```text
///
/// {
///   "snapshot":
///   {
///     "volume_id": "<Cinder volume already exists in volume backend>",
///     "ref":
///        "<Driver-specific reference to the existing storage object>"
///   }
/// }
///
/// ```
///
/// See the appropriate Cinder drivers' implementations of the manage_snapshot
/// method to find out the accepted format of 'ref'. For example,in LVM driver,
/// it will be the logic volume name of snapshot which you want to manage.
///
/// This API call will return with an error if any of the above elements are
/// missing from the request, or if the 'volume_id' element refers to a cinder
/// volume that could not be found.
///
/// The snapshot will later enter the error state if it is discovered that
/// 'ref' is bad.
///
/// Optional elements to 'snapshot' are:
///
/// ```text
///
/// name           A name for the new snapshot.
/// description    A description for the new snapshot.
/// metadata       Key/value pairs to be associated with the new snapshot.
///
/// ```
///
#[derive(Args)]
pub struct SnapshotManageCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

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
    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    _ref: Option<Value>,

    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

/// SnapshotManage response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl SnapshotManageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create SnapshotManage");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.snapshot data
        let args = &self.snapshot;
        let mut snapshot_builder = create::SnapshotBuilder::default();
        if let Some(val) = &args.description {
            snapshot_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            snapshot_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.name {
            snapshot_builder.name(Some(val.into()));
        }

        snapshot_builder.volume_id(&args.volume_id);

        if let Some(val) = &args._ref {
            snapshot_builder._ref(val.clone());
        }

        ep_builder.snapshot(snapshot_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let _rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}
