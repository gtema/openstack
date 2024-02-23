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

//! Action Volume command [microversion = 3.1]
//!
//! Wraps invoking of the `v3/volumes/{id}/action` with `POST` method

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

use bytes::Bytes;
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::block_storage::v3::volume::os_volume_upload_image_31;
use openstack_sdk::api::RawQueryAsync;
use structable_derive::StructTable;

/// Command without description in OpenAPI
///
#[derive(Args)]
pub struct VolumeCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_volume_upload_image: OsVolumeUploadImage,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    ///
    #[arg(id = "path_param_id", value_name = "ID")]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum DiskFormat {
    Ploop,
    Qcow2,
    Raw,
    Vdi,
    Vhd,
    Vhdx,
    Vmdk,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Visibility {
    Community,
    Private,
    Public,
    Shared,
}

/// OsVolumeUploadImage Body data
#[derive(Args)]
struct OsVolumeUploadImage {
    #[arg(long)]
    image_name: String,

    #[arg(action=clap::ArgAction::Set, long)]
    force: Option<bool>,

    #[arg(long)]
    disk_format: Option<DiskFormat>,

    #[arg(long)]
    container_format: Option<String>,

    #[arg(long)]
    visibility: Option<Visibility>,

    #[arg(action=clap::ArgAction::Set, long)]
    protected: Option<bool>,
}

/// Volume response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl VolumeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = os_volume_upload_image_31::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.1");

        // Set path parameters
        ep_builder.id(&self.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_volume_upload_image data
        let args = &self.os_volume_upload_image;
        let mut os_volume_upload_image_builder =
            os_volume_upload_image_31::OsVolumeUploadImageBuilder::default();

        os_volume_upload_image_builder.image_name(args.image_name.clone());

        if let Some(val) = &args.force {
            os_volume_upload_image_builder.force(*val);
        }

        if let Some(val) = &args.disk_format {
            let tmp = match val {
                DiskFormat::Ploop => os_volume_upload_image_31::DiskFormat::Ploop,
                DiskFormat::Qcow2 => os_volume_upload_image_31::DiskFormat::Qcow2,
                DiskFormat::Raw => os_volume_upload_image_31::DiskFormat::Raw,
                DiskFormat::Vdi => os_volume_upload_image_31::DiskFormat::Vdi,
                DiskFormat::Vhd => os_volume_upload_image_31::DiskFormat::Vhd,
                DiskFormat::Vhdx => os_volume_upload_image_31::DiskFormat::Vhdx,
                DiskFormat::Vmdk => os_volume_upload_image_31::DiskFormat::Vmdk,
            };
            os_volume_upload_image_builder.disk_format(tmp);
        }

        if let Some(val) = &args.container_format {
            os_volume_upload_image_builder.container_format(Some(val.into()));
        }

        if let Some(val) = &args.visibility {
            let tmp = match val {
                Visibility::Community => os_volume_upload_image_31::Visibility::Community,
                Visibility::Private => os_volume_upload_image_31::Visibility::Private,
                Visibility::Public => os_volume_upload_image_31::Visibility::Public,
                Visibility::Shared => os_volume_upload_image_31::Visibility::Shared,
            };
            os_volume_upload_image_builder.visibility(tmp);
        }

        if let Some(val) = &args.protected {
            os_volume_upload_image_builder.protected(*val);
        }

        ep_builder.os_volume_upload_image(os_volume_upload_image_builder.build().unwrap());

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
