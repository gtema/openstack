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
//! Response type for the patch images/{image_id} operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// Image response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ImageResponse {
    /// An MD5 hash over the image data. The value might be `null` (JSON null
    /// data type), as this field is no longer populated by the Image Service
    /// beginning with the Victoria release. It remains present for backward
    /// compatibility with legacy images. To validate image data, instead use
    /// the secure multihash fields `os_hash_algo` and `os_hash_value`.
    ///
    #[structable(optional, serialize)]
    pub checksum: Option<String>,

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
    #[structable(optional, serialize)]
    pub container_format: Option<ContainerFormat>,

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
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The URL to access the image file kept in external store. *It is present
    /// only if the* `show_image_direct_url` *option is* `true` *in the Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this option is disabled by default.**
    ///
    #[structable(optional)]
    pub direct_url: Option<String>,

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
    #[structable(optional, serialize)]
    pub disk_format: Option<DiskFormat>,

    /// The URL for the virtual machine image file.
    ///
    #[structable(optional)]
    pub file: Option<String>,

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
    #[structable(optional)]
    pub id: Option<String>,

    /// A list of objects, each of which describes an image location. Each
    /// object contains a `url` key, whose value is a URL specifying a
    /// location, and a `metadata` key, whose value is a dict of key:value
    /// pairs containing information appropriate to the use of whatever
    /// external store is indicated by the URL. *This list appears only if the*
    /// `show_multiple_locations` *option is set to* `true` *in the Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this option is disabled by default.**
    ///
    #[structable(optional, serialize)]
    pub locations: Option<Vec<Locations>>,

    /// Amount of disk space in GB that is required to boot the image. The
    /// value might be `null` (JSON null data type).
    ///
    #[structable(optional)]
    pub min_disk: Option<i32>,

    /// Amount of RAM in MB that is required to boot the image. The value might
    /// be `null` (JSON null data type).
    ///
    #[structable(optional)]
    pub min_ram: Option<i32>,

    /// The name of the image. Value might be `null` (JSON null data type).
    ///
    #[structable(optional, serialize)]
    pub name: Option<String>,

    /// The algorithm used to compute a secure hash of the image data for this
    /// image. The result of the computation is displayed as the value of the
    /// `os_hash_value` property. The value might be `null` (JSON null data
    /// type). The algorithm used is chosen by the cloud operator; it may not
    /// be configured by end users. *(Since Image API v2.7)*
    ///
    #[structable(optional, serialize)]
    pub os_hash_algo: Option<String>,

    /// The hexdigest of the secure hash of the image data computed using the
    /// algorithm whose name is the value of the `os_hash_algo` property. The
    /// value might be `null` (JSON null data type) if data has not yet been
    /// associated with this image, or if the image was created using a version
    /// of the Image Service API prior to version 2.7. *(Since Image API v2.7)*
    ///
    #[structable(optional, serialize)]
    pub os_hash_value: Option<String>,

    /// This field controls whether an image is displayed in the default
    /// image-list response. A “hidden” image is out of date somehow (for
    /// example, it may not have the latest updates applied) and hence should
    /// not be a user’s first choice, but it’s not deleted because it may be
    /// needed for server rebuilds. By hiding it from the default image list,
    /// it’s easier for end users to find and use a more up-to-date version of
    /// this image. *(Since Image API v2.7)*
    ///
    #[structable(optional)]
    pub os_hidden: Option<bool>,

    /// An identifier for the owner of the image, usually the project (also
    /// called the “tenant”) ID. The value might be `null` (JSON null data
    /// type).
    ///
    #[structable(optional, serialize)]
    pub owner: Option<String>,

    /// A boolean value that must be `false` or the image cannot be deleted.
    ///
    #[structable(optional)]
    pub protected: Option<bool>,

    /// The URL for the schema describing a virtual machine image.
    ///
    #[structable(optional)]
    pub schema: Option<String>,

    /// The URL for the virtual machine image.
    ///
    #[serde(rename = "self")]
    #[structable(optional, title = "self")]
    pub _self: Option<String>,

    /// The size of the image data, in bytes. The value might be `null` (JSON
    /// null data type).
    ///
    #[structable(optional, serialize)]
    pub size: Option<i32>,

    /// The image status.
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// Store in which image data resides. Only present when the operator has
    /// enabled multiple stores. May be a comma-separated list of store
    /// identifiers.
    ///
    #[structable(optional)]
    pub stores: Option<String>,

    /// List of tags for this image, possibly an empty list.
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

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
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// Virtual size of image in bytes
    ///
    #[structable(optional, serialize)]
    pub virtual_size: Option<i32>,

    /// Image visibility, that is, the access permission for the image.
    ///
    #[structable(optional, serialize)]
    pub visibility: Option<Visibility>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Active
    #[serde(rename = "active")]
    Active,

    // Deactivated
    #[serde(rename = "deactivated")]
    Deactivated,

    // Deleted
    #[serde(rename = "deleted")]
    Deleted,

    // Importing
    #[serde(rename = "importing")]
    Importing,

    // Killed
    #[serde(rename = "killed")]
    Killed,

    // PendingDelete
    #[serde(rename = "pending_delete")]
    PendingDelete,

    // Queued
    #[serde(rename = "queued")]
    Queued,

    // Saving
    #[serde(rename = "saving")]
    Saving,

    // Uploading
    #[serde(rename = "uploading")]
    Uploading,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Visibility {
    // Community
    #[serde(rename = "community")]
    Community,

    // Private
    #[serde(rename = "private")]
    Private,

    // Public
    #[serde(rename = "public")]
    Public,

    // Shared
    #[serde(rename = "shared")]
    Shared,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum ContainerFormat {
    // Aki
    #[serde(rename = "aki")]
    Aki,

    // Ami
    #[serde(rename = "ami")]
    Ami,

    // Ari
    #[serde(rename = "ari")]
    Ari,

    // Bare
    #[serde(rename = "bare")]
    Bare,

    // Compressed
    #[serde(rename = "compressed")]
    Compressed,

    // Docker
    #[serde(rename = "docker")]
    Docker,

    // Ova
    #[serde(rename = "ova")]
    Ova,

    // Ovf
    #[serde(rename = "ovf")]
    Ovf,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum DiskFormat {
    // Aki
    #[serde(rename = "aki")]
    Aki,

    // Ami
    #[serde(rename = "ami")]
    Ami,

    // Ari
    #[serde(rename = "ari")]
    Ari,

    // Iso
    #[serde(rename = "iso")]
    Iso,

    // Ploop
    #[serde(rename = "ploop")]
    Ploop,

    // Qcow2
    #[serde(rename = "qcow2")]
    Qcow2,

    // Raw
    #[serde(rename = "raw")]
    Raw,

    // Vdi
    #[serde(rename = "vdi")]
    Vdi,

    // Vhd
    #[serde(rename = "vhd")]
    Vhd,

    // Vhdx
    #[serde(rename = "vhdx")]
    Vhdx,

    // Vmdk
    #[serde(rename = "vmdk")]
    Vmdk,
}

/// Values to be used to populate the corresponding image properties. If the
/// image status is not 'queued', values must exactly match those already
/// contained in the image properties.
///
/// `ValidationData` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidationData {
    pub checksum: Option<String>,
    pub os_hash_algo: String,
    pub os_hash_value: String,
}

/// `Locations` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Locations {
    pub metadata: HashMap<String, Value>,
    pub url: String,
    pub validation_data: Option<ValidationData>,
}
