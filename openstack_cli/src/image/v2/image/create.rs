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

//! Create Image command
//!
//! Wraps invoking of the `v2/images` with `POST` method

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
use clap::ValueEnum;
use http::Response;
use openstack_sdk::api::image::v2::image::create;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a catalog record for an operating system disk image. *(Since Image
/// API v2.0)*
///
/// The `Location` response header contains the URI for the image.
///
/// A multiple store backend support is introduced in the Rocky release as a
/// part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header
/// `OpenStack-image-store-ids` which contains the list of available stores
/// will be included in response. This header is only included if multiple
/// backend stores are supported.
///
/// The response body contains the new image entity.
///
/// Synchronous Postconditions
///
/// Normal response codes: 201
///
/// Error response codes: 400, 401, 403, 409, 413, 415
///
#[derive(Args)]
#[command(about = "Create image")]
pub struct ImageCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Format of the image container.
    ///
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the [Image Schema](#image-schema) response from
    /// the cloud itself for the valid values available.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `bare`, `ovf`, `ova`, or
    /// `docker`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    #[arg(help_heading = "Body parameters", long)]
    container_format: Option<ContainerFormat>,

    /// The format of the disk.
    ///
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the [Image Schema](#image-schema) response from
    /// the cloud itself for the valid values available.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `vhd`, `vhdx`, `vmdk`, `raw`,
    /// `qcow2`, `vdi`, `ploop` or `iso`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    /// **Newton changes**: The `vhdx` disk format is a supported value.
    ///
    /// **Ocata changes**: The `ploop` disk format is a supported value.
    ///
    #[arg(help_heading = "Body parameters", long)]
    disk_format: Option<DiskFormat>,

    /// A unique, user-defined image UUID, in the format:
    ///
    /// ```text
    /// nnnnnnnn-nnnn-nnnn-nnnn-nnnnnnnnnnnn
    ///
    /// ```
    ///
    /// Where **n** is a hexadecimal digit from 0 to f, or F.
    ///
    /// For example:
    ///
    /// ```text
    /// b2173dd3-7ad6-4362-baa6-a68bce3565cb
    ///
    /// ```
    ///
    /// If you omit this value, the API generates a UUID for the image. If you
    /// specify a value that has already been assigned, the request fails with
    /// a `409` response code.
    ///
    #[arg(help_heading = "Body parameters", long)]
    id: Option<String>,

    /// A set of URLs to access the image file kept in external store
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    locations: Option<Vec<Value>>,

    /// Amount of disk space in GB that is required to boot the image.
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_disk: Option<i32>,

    /// Amount of RAM in MB that is required to boot the image.
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_ram: Option<i32>,

    /// The name of the image.
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// If true, image will not appear in default image list response.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    os_hidden: Option<bool>,

    /// Owner of the image
    ///
    #[arg(help_heading = "Body parameters", long)]
    owner: Option<String>,

    /// Image protection for deletion. Valid value is `true` or `false`.
    /// Default is `false`.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    protected: Option<bool>,

    /// List of tags for this image. Each tag is a string of at most 255 chars.
    /// The maximum number of tags allowed on an image is set by the operator.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    /// Visibility for this image. Valid value is one of: `public`, `private`,
    /// `shared`, or `community`. At most sites, only an administrator can make
    /// an image `public`. Some sites may restrict what users can make an image
    /// `community`. Some sites may restrict what users can perform member
    /// operations on a `shared` image. *Since the Image API v2.5, the default
    /// value is `shared`.*
    ///
    #[arg(help_heading = "Body parameters", long)]
    visibility: Option<Visibility>,
    /// Additional properties to be sent with the request
    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, String>)]
    #[arg(help_heading = "Body parameters")]
    properties: Option<Vec<(String, String)>>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Visibility {
    Community,
    Private,
    Public,
    Shared,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum ContainerFormat {
    Aki,
    Ami,
    Ari,
    Bare,
    Compressed,
    Docker,
    Ova,
    Ovf,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum DiskFormat {
    Aki,
    Ami,
    Ari,
    Iso,
    Ploop,
    Qcow2,
    Raw,
    Vdi,
    Vhd,
    Vhdx,
    Vmdk,
}

/// Image response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Image");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.container_format data
        if let Some(args) = &self.container_format {
            let tmp = match args {
                ContainerFormat::Aki => create::ContainerFormat::Aki,
                ContainerFormat::Ami => create::ContainerFormat::Ami,
                ContainerFormat::Ari => create::ContainerFormat::Ari,
                ContainerFormat::Bare => create::ContainerFormat::Bare,
                ContainerFormat::Compressed => create::ContainerFormat::Compressed,
                ContainerFormat::Docker => create::ContainerFormat::Docker,
                ContainerFormat::Ova => create::ContainerFormat::Ova,
                ContainerFormat::Ovf => create::ContainerFormat::Ovf,
            };
            ep_builder.container_format(tmp);
        }

        // Set Request.disk_format data
        if let Some(args) = &self.disk_format {
            let tmp = match args {
                DiskFormat::Aki => create::DiskFormat::Aki,
                DiskFormat::Ami => create::DiskFormat::Ami,
                DiskFormat::Ari => create::DiskFormat::Ari,
                DiskFormat::Iso => create::DiskFormat::Iso,
                DiskFormat::Ploop => create::DiskFormat::Ploop,
                DiskFormat::Qcow2 => create::DiskFormat::Qcow2,
                DiskFormat::Raw => create::DiskFormat::Raw,
                DiskFormat::Vdi => create::DiskFormat::Vdi,
                DiskFormat::Vhd => create::DiskFormat::Vhd,
                DiskFormat::Vhdx => create::DiskFormat::Vhdx,
                DiskFormat::Vmdk => create::DiskFormat::Vmdk,
            };
            ep_builder.disk_format(tmp);
        }

        // Set Request.id data
        if let Some(args) = &self.id {
            ep_builder.id(args);
        }

        // Set Request.locations data
        if let Some(args) = &self.locations {
            let locations_builder: Vec<create::Locations> = args
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Locations>(v.to_owned()))
                .collect::<Vec<create::Locations>>();
            ep_builder.locations(locations_builder);
        }

        // Set Request.min_disk data
        if let Some(args) = &self.min_disk {
            ep_builder.min_disk(*args);
        }

        // Set Request.min_ram data
        if let Some(args) = &self.min_ram {
            ep_builder.min_ram(*args);
        }

        // Set Request.name data
        if let Some(args) = &self.name {
            ep_builder.name(Some(args.into()));
        }

        // Set Request.os_hidden data
        if let Some(args) = &self.os_hidden {
            ep_builder.os_hidden(*args);
        }

        // Set Request.owner data
        if let Some(args) = &self.owner {
            ep_builder.owner(Some(args.into()));
        }

        // Set Request.protected data
        if let Some(args) = &self.protected {
            ep_builder.protected(*args);
        }

        // Set Request.tags data
        if let Some(args) = &self.tags {
            ep_builder.tags(args.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        // Set Request.visibility data
        if let Some(args) = &self.visibility {
            let tmp = match args {
                Visibility::Community => create::Visibility::Community,
                Visibility::Private => create::Visibility::Private,
                Visibility::Public => create::Visibility::Public,
                Visibility::Shared => create::Visibility::Shared,
            };
            ep_builder.visibility(tmp);
        }

        if let Some(properties) = &self.properties {
            ep_builder.properties(properties.iter().cloned());
        }

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
