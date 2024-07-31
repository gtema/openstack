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

//! Create VolumeManage command [microversion = 3.16]
//!
//! Wraps invoking of the `v3/os-volume-manage` with `POST` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

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
use openstack_sdk::api::block_storage::v3::volume_manage::create_316;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Instruct Cinder to manage a storage object.
///
/// Manages an existing backend storage object (e.g. a Linux logical volume or
/// a SAN disk) by creating the Cinder objects required to manage it, and
/// possibly renaming the backend storage object (driver dependent)
///
/// From an API perspective, this operation behaves very much like a volume
/// creation operation, except that properties such as image, snapshot and
/// volume references don't make sense, because we are taking an existing
/// storage object into Cinder management.
///
/// Required HTTP Body:
///
/// ```text
///
/// {
///   "volume": {
///     "host": "<Cinder host on which the existing storage resides>",
///     "cluster": "<Cinder cluster on which the storage resides>",
///     "ref": "<Driver-specific reference to existing storage object>"
///   }
/// }
///
/// ```
///
/// See the appropriate Cinder drivers' implementations of the manage_volume
/// method to find out the accepted format of 'ref'.
///
/// This API call will return with an error if any of the above elements are
/// missing from the request, or if the 'host' element refers to a cinder host
/// that is not registered.
///
/// The volume will later enter the error state if it is discovered that 'ref'
/// is bad.
///
/// Optional elements to 'volume' are:
///
/// ```text
///
/// name               A name for the new volume.
/// description        A description for the new volume.
/// volume_type        ID or name of a volume type to associate with
///                    the new Cinder volume. Does not necessarily
///                    guarantee that the managed volume will have the
///                    properties described in the volume_type. The
///                    driver may choose to fail if it identifies that
///                    the specified volume_type is not compatible with
///                    the backend storage object.
/// metadata           Key/value pairs to be associated with the new
///                    volume.
/// availability_zone  The availability zone to associate with the new
///                    volume.
/// bootable           If set to True, marks the volume as bootable.
///
/// ```
///
#[derive(Args)]
pub struct VolumeManageCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    volume: Volume,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}
/// Volume Body data
#[derive(Args, Clone)]
struct Volume {
    #[arg(help_heading = "Body parameters", long)]
    availability_zone: Option<String>,

    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    bootable: Option<bool>,

    #[arg(help_heading = "Body parameters", long)]
    cluster: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    description: Option<String>,

    #[arg(help_heading = "Body parameters", long)]
    host: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    #[arg(help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    _ref: Value,

    #[arg(help_heading = "Body parameters", long)]
    volume_type: Option<String>,
}

/// VolumeManage response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl VolumeManageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create VolumeManage");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_316::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.16");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.volume data
        let args = &self.volume;
        let mut volume_builder = create_316::VolumeBuilder::default();
        if let Some(val) = &args.description {
            volume_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.availability_zone {
            volume_builder.availability_zone(Some(val.into()));
        }

        if let Some(val) = &args.bootable {
            volume_builder.bootable(*val);
        }

        if let Some(val) = &args.volume_type {
            volume_builder.volume_type(Some(val.into()));
        }

        if let Some(val) = &args.name {
            volume_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.host {
            volume_builder.host(Some(val.into()));
        }

        volume_builder._ref(args._ref.clone());

        if let Some(val) = &args.metadata {
            volume_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.cluster {
            volume_builder.cluster(Some(val.into()));
        }

        ep_builder.volume(volume_builder.build().unwrap());

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
