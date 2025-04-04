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

//! Set Image command
//!
//! Wraps invoking of the `v2/images/{image_id}` with `PATCH` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_sdk::AsyncOpenStack;

use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;
use crate::output::OutputProcessor;

use crate::common::parse_json;
use crate::common::parse_key_val;
use clap::ValueEnum;
use json_patch::diff;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::image::v2::image::patch;
use serde_json::Value;
use serde_json::json;
use structable_derive::StructTable;

/// Updates an image. *(Since Image API v2.0)*
///
/// Conceptually, you update an image record by patching the JSON
/// representation of the image, passing a request body conforming to one of
/// the following media types:
///
/// Attempting to make a PATCH call using some other media type will provoke a
/// response code of 415 (Unsupported media type).
///
/// The `application/openstack-images-v2.1-json-patch` media type provides a
/// useful and compatible subset of the functionality defined in JavaScript
/// Object Notation (JSON) Patch [RFC6902](http://tools.ietf.org/html/rfc6902),
/// which defines the `application/json-patch+json` media type.
///
/// For information about the PATCH method and the available media types, see
/// [Image API v2 HTTP PATCH media types](http://specs.openstack.org/openstack/glance-specs/specs/api/v2/http-patch-image-api-v2.html).
///
/// Attempting to modify some image properties will cause the entire request to
/// fail with a 403 (Forbidden) response code:
///
/// Attempting to add a location path to an image that is not in `queued` or
/// `active` state will result in a 409 (Conflict) response code *(since Image
/// API v2.4)*.
///
/// Normal response codes: 200
///
/// Error response codes: 400, 401, 403, 404, 409, 413, 415
///
#[derive(Args)]
#[command(about = "Update image")]
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
    /// the cloud itself for the valid values available. See
    /// [Container Format](https://docs.openstack.org/glance/latest/user/formats.html#container-format)
    /// in the Glance documentation for more information.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `bare`, `ovf`, `ova`,
    /// `docker`, or `compressed`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    /// **Train changes**: The `compressed` container format is a supported
    /// value.
    ///
    #[arg(help_heading = "Body parameters", long)]
    container_format: Option<ContainerFormat>,

    /// The format of the disk.
    ///
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the [Image Schema](#image-schema) response from
    /// the cloud itself for the valid values available. See
    /// [Disk Format](https://docs.openstack.org/glance/latest/user/formats.html#disk-format)
    /// in the Glance documentation for more information.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `vhd`, `vhdx`, `vmdk`, `raw`,
    /// `qcow2`, `vdi`, `ploop` or `iso`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    /// **Newton changes**: The `vhdx` disk format is a supported value.\
    /// **Ocata changes**: The `ploop` disk format is a supported value.
    ///
    #[arg(help_heading = "Body parameters", long)]
    disk_format: Option<DiskFormat>,

    /// A list of objects, each of which describes an image location. Each
    /// object contains a `url` key, whose value is a URL specifying a
    /// location, and a `metadata` key, whose value is a dict of key:value
    /// pairs containing information appropriate to the use of whatever
    /// external store is indicated by the URL. *This list appears only if the*
    /// `show_multiple_locations` *option is set to* `true` *in the Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this option is disabled by default.**
    ///
    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long, value_name="JSON", value_parser=parse_json)]
    locations: Option<Vec<Value>>,

    /// Amount of disk space in GB that is required to boot the image. The
    /// value might be `null` (JSON null data type).
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_disk: Option<i32>,

    /// Amount of RAM in MB that is required to boot the image. The value might
    /// be `null` (JSON null data type).
    ///
    #[arg(help_heading = "Body parameters", long)]
    min_ram: Option<i32>,

    /// The name of the image. Value might be `null` (JSON null data type).
    ///
    #[arg(help_heading = "Body parameters", long)]
    name: Option<String>,

    /// This field controls whether an image is displayed in the default
    /// image-list response. A “hidden” image is out of date somehow (for
    /// example, it may not have the latest updates applied) and hence should
    /// not be a user’s first choice, but it’s not deleted because it may be
    /// needed for server rebuilds. By hiding it from the default image list,
    /// it’s easier for end users to find and use a more up-to-date version of
    /// this image. *(Since Image API v2.7)*
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    os_hidden: Option<bool>,

    /// An identifier for the owner of the image, usually the project (also
    /// called the “tenant”) ID. The value might be `null` (JSON null data
    /// type).
    ///
    #[arg(help_heading = "Body parameters", long)]
    owner: Option<String>,

    /// A boolean value that must be `false` or the image cannot be deleted.
    ///
    #[arg(action=clap::ArgAction::Set, help_heading = "Body parameters", long)]
    protected: Option<bool>,

    /// List of tags for this image, possibly an empty list.
    ///
    /// Parameter is an array, may be provided multiple times.
    ///
    #[arg(action=clap::ArgAction::Append, help_heading = "Body parameters", long)]
    tags: Option<Vec<String>>,

    /// Image visibility, that is, the access permission for the image.
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
struct PathParameters {
    /// image_id parameter for /v2/images/{image_id} API
    ///
    #[arg(
        help_heading = "Path parameters",
        id = "path_param_id",
        value_name = "ID"
    )]
    id: String,
}

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
struct ResponseData {
    /// An MD5 hash over the image data. The value might be `null` (JSON null
    /// data type), as this field is no longer populated by the Image Service
    /// beginning with the Victoria release. It remains present for backward
    /// compatibility with legacy images. To validate image data, instead use
    /// the secure multihash fields `os_hash_algo` and `os_hash_value`.
    ///
    #[serde()]
    #[structable(optional)]
    checksum: Option<String>,

    /// Format of the image container.
    ///
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the [Image Schema](#image-schema) response from
    /// the cloud itself for the valid values available. See
    /// [Container Format](https://docs.openstack.org/glance/latest/user/formats.html#container-format)
    /// in the Glance documentation for more information.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `bare`, `ovf`, `ova`,
    /// `docker`, or `compressed`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    /// **Train changes**: The `compressed` container format is a supported
    /// value.
    ///
    #[serde()]
    #[structable(optional)]
    container_format: Option<String>,

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
    #[structable(optional)]
    created_at: Option<String>,

    /// The URL to access the image file kept in external store. *It is present
    /// only if the* `show_image_direct_url` *option is* `true` *in the Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this option is disabled by default.**
    ///
    #[serde()]
    #[structable(optional)]
    direct_url: Option<String>,

    /// The format of the disk.
    ///
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the [Image Schema](#image-schema) response from
    /// the cloud itself for the valid values available. See
    /// [Disk Format](https://docs.openstack.org/glance/latest/user/formats.html#disk-format)
    /// in the Glance documentation for more information.
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `vhd`, `vhdx`, `vmdk`, `raw`,
    /// `qcow2`, `vdi`, `ploop` or `iso`.
    ///
    /// The value might be `null` (JSON null data type).
    ///
    /// **Newton changes**: The `vhdx` disk format is a supported value.\
    /// **Ocata changes**: The `ploop` disk format is a supported value.
    ///
    #[serde()]
    #[structable(optional)]
    disk_format: Option<String>,

    /// The URL for the virtual machine image file.
    ///
    #[serde()]
    #[structable(optional)]
    file: Option<String>,

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
    /// If you omit this value, the API generates a UUID for the image.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// A list of objects, each of which describes an image location. Each
    /// object contains a `url` key, whose value is a URL specifying a
    /// location, and a `metadata` key, whose value is a dict of key:value
    /// pairs containing information appropriate to the use of whatever
    /// external store is indicated by the URL. *This list appears only if the*
    /// `show_multiple_locations` *option is set to* `true` *in the Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this option is disabled by default.**
    ///
    #[serde()]
    #[structable(optional, pretty)]
    locations: Option<Value>,

    /// Amount of disk space in GB that is required to boot the image. The
    /// value might be `null` (JSON null data type).
    ///
    #[serde()]
    #[structable(optional)]
    min_disk: Option<i32>,

    /// Amount of RAM in MB that is required to boot the image. The value might
    /// be `null` (JSON null data type).
    ///
    #[serde()]
    #[structable(optional)]
    min_ram: Option<i32>,

    /// The name of the image. Value might be `null` (JSON null data type).
    ///
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The algorithm used to compute a secure hash of the image data for this
    /// image. The result of the computation is displayed as the value of the
    /// `os_hash_value` property. The value might be `null` (JSON null data
    /// type). The algorithm used is chosen by the cloud operator; it may not
    /// be configured by end users. *(Since Image API v2.7)*
    ///
    #[serde()]
    #[structable(optional)]
    os_hash_algo: Option<String>,

    /// The hexdigest of the secure hash of the image data computed using the
    /// algorithm whose name is the value of the `os_hash_algo` property. The
    /// value might be `null` (JSON null data type) if data has not yet been
    /// associated with this image, or if the image was created using a version
    /// of the Image Service API prior to version 2.7. *(Since Image API v2.7)*
    ///
    #[serde()]
    #[structable(optional)]
    os_hash_value: Option<String>,

    /// This field controls whether an image is displayed in the default
    /// image-list response. A “hidden” image is out of date somehow (for
    /// example, it may not have the latest updates applied) and hence should
    /// not be a user’s first choice, but it’s not deleted because it may be
    /// needed for server rebuilds. By hiding it from the default image list,
    /// it’s easier for end users to find and use a more up-to-date version of
    /// this image. *(Since Image API v2.7)*
    ///
    #[serde()]
    #[structable(optional)]
    os_hidden: Option<bool>,

    /// An identifier for the owner of the image, usually the project (also
    /// called the “tenant”) ID. The value might be `null` (JSON null data
    /// type).
    ///
    #[serde()]
    #[structable(optional)]
    owner: Option<String>,

    /// A boolean value that must be `false` or the image cannot be deleted.
    ///
    #[serde()]
    #[structable(optional)]
    protected: Option<bool>,

    /// The URL for the schema describing a virtual machine image.
    ///
    #[serde()]
    #[structable(optional)]
    schema: Option<String>,

    /// The URL for the virtual machine image.
    ///
    #[serde(rename = "self")]
    #[structable(optional, title = "self")]
    _self: Option<String>,

    /// The size of the image data, in bytes. The value might be `null` (JSON
    /// null data type).
    ///
    #[serde()]
    #[structable(optional)]
    size: Option<i32>,

    /// The image status.
    ///
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// Store in which image data resides. Only present when the operator has
    /// enabled multiple stores. May be a comma-separated list of store
    /// identifiers.
    ///
    #[serde()]
    #[structable(optional)]
    stores: Option<String>,

    /// List of tags for this image, possibly an empty list.
    ///
    #[serde()]
    #[structable(optional, pretty)]
    tags: Option<Value>,

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

    /// Virtual size of image in bytes
    ///
    #[serde()]
    #[structable(optional)]
    virtual_size: Option<i32>,

    /// Image visibility, that is, the access permission for the image.
    ///
    #[serde()]
    #[structable(optional)]
    visibility: Option<String>,
}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Image");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        // Patching resource requires fetching and calculating diff
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();

        let data: ResponseData = serde_json::from_value(find_data)?;
        let mut new = data.clone();
        if let Some(val) = &self.name {
            new.name = Some(val.into());
        }
        if let Some(val) = &self.visibility {
            // StringEnum
            let tmp = match val {
                Visibility::Community => "community",
                Visibility::Private => "private",
                Visibility::Public => "public",
                Visibility::Shared => "shared",
            };
            new.visibility = Some(tmp.to_string());
        }
        if let Some(val) = &self.protected {
            new.protected = Some(*val);
        }
        if let Some(val) = &self.os_hidden {
            new.os_hidden = Some(*val);
        }
        if let Some(val) = &self.owner {
            new.owner = Some(val.into());
        }
        if let Some(val) = &self.container_format {
            // StringEnum
            let tmp = match val {
                ContainerFormat::Aki => "aki",
                ContainerFormat::Ami => "ami",
                ContainerFormat::Ari => "ari",
                ContainerFormat::Bare => "bare",
                ContainerFormat::Compressed => "compressed",
                ContainerFormat::Docker => "docker",
                ContainerFormat::Ova => "ova",
                ContainerFormat::Ovf => "ovf",
            };
            new.container_format = Some(tmp.to_string());
        }
        if let Some(val) = &self.disk_format {
            // StringEnum
            let tmp = match val {
                DiskFormat::Aki => "aki",
                DiskFormat::Ami => "ami",
                DiskFormat::Ari => "ari",
                DiskFormat::Iso => "iso",
                DiskFormat::Ploop => "ploop",
                DiskFormat::Qcow2 => "qcow2",
                DiskFormat::Raw => "raw",
                DiskFormat::Vdi => "vdi",
                DiskFormat::Vhd => "vhd",
                DiskFormat::Vhdx => "vhdx",
                DiskFormat::Vmdk => "vmdk",
            };
            new.disk_format = Some(tmp.to_string());
        }
        if let Some(val) = &self.tags {
            new.tags = Some(serde_json::from_value(val.to_owned().into())?);
        }
        if let Some(val) = &self.min_ram {
            new.min_ram = Some(*val);
        }
        if let Some(val) = &self.min_disk {
            new.min_disk = Some(*val);
        }
        if let Some(val) = &self.locations {
            new.locations = Some(serde_json::from_value(val.to_owned().into())?);
        }

        let curr_json = serde_json::to_value(&data).unwrap();
        let mut new_json = serde_json::to_value(&new).unwrap();
        if let Some(properties) = &self.properties {
            for (key, val) in properties {
                new_json[key] = json!(val);
            }
        }

        let patch = diff(&curr_json, &new_json);

        let mut patch_ep_builder = patch::Request::builder();
        patch_ep_builder.id(&resource_id);
        patch_ep_builder.patch(patch);

        let patch_ep = patch_ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let new_data = patch_ep.query_async(client).await?;
        op.output_single::<ResponseData>(new_data)?;
        Ok(())
    }
}
