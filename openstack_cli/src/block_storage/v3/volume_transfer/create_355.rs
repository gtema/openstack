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

//! Create VolumeTransfer command [microversion = 3.55]
//!
//! Wraps invoking of the `v3/volume-transfers` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use bytes::Bytes;
use http::Response;
use openstack_sdk::api::block_storage::v3::volume_transfer::create_355;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Create a new volume transfer.
///
#[derive(Args)]
pub struct VolumeTransferCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// The volume transfer object.
    ///
    #[command(flatten)]
    transfer: Transfer,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Transfer Body data
#[derive(Args, Clone)]
struct Transfer {
    /// The name of the object.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// Transfer volume without snapshots. Defaults to False if not specified.
    ///
    /// **New in version 3.55**
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    no_snapshots: Option<bool>,

    /// The UUID of the volume.
    ///
    #[arg(help_heading = "Body parameters", long)]
    volume_id: String,
}

/// VolumeTransfer response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl VolumeTransferCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create VolumeTransfer");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_355::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.55");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.transfer data
        let args = &self.transfer;
        let mut transfer_builder = create_355::TransferBuilder::default();

        transfer_builder.volume_id(&args.volume_id);

        if let Some(val) = &args.name {
            transfer_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.no_snapshots {
            transfer_builder.no_snapshots(*val);
        }

        ep_builder.transfer(transfer_builder.build().unwrap());

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
