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
//! Response type for the get images operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use structable::{StructTable, StructTableOptions};

/// Image response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ImageResponse {
    /// md5 hash of image contents.
    ///
    #[structable(optional, serialize, wide)]
    pub checksum: Option<String>,

    /// Format of the container
    ///
    #[structable(optional, serialize, wide)]
    pub container_format: Option<ContainerFormat>,

    /// Date and time of image registration
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// URL to access the image file kept in external store
    ///
    #[structable(optional, wide)]
    pub direct_url: Option<String>,

    /// Format of the disk
    ///
    #[structable(optional, serialize, wide)]
    pub disk_format: Option<DiskFormat>,

    /// An image file url
    ///
    #[structable(optional, wide)]
    pub file: Option<String>,

    /// An identifier for the image
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// A set of URLs to access the image file kept in external store
    ///
    #[structable(optional, serialize, wide)]
    pub locations: Option<Vec<Locations>>,

    /// Amount of disk space (in GB) required to boot image.
    ///
    #[structable(optional, wide)]
    pub min_disk: Option<i32>,

    /// Amount of ram (in MB) required to boot image.
    ///
    #[structable(optional, wide)]
    pub min_ram: Option<i32>,

    /// Descriptive name for the image
    ///
    #[structable(optional, serialize)]
    pub name: Option<String>,

    /// Algorithm to calculate the os_hash_value
    ///
    #[structable(optional, serialize, wide)]
    pub os_hash_algo: Option<String>,

    /// Hexdigest of the image contents using the algorithm specified by the
    /// os_hash_algo
    ///
    #[structable(optional, serialize, wide)]
    pub os_hash_value: Option<String>,

    /// If true, image will not appear in default image list response.
    ///
    #[structable(optional, wide)]
    pub os_hidden: Option<bool>,

    /// Owner of the image
    ///
    #[structable(optional, serialize, wide)]
    pub owner: Option<String>,

    /// If true, image will not be deletable.
    ///
    #[structable(optional, wide)]
    pub protected: Option<bool>,

    /// An image schema url
    ///
    #[structable(optional, wide)]
    pub schema: Option<String>,

    /// An image self url
    ///
    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    pub _self: Option<String>,

    /// Size of image file in bytes
    ///
    #[structable(optional, serialize, wide)]
    pub size: Option<i64>,

    /// Status of the image
    ///
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// Store in which image data resides. Only present when the operator has
    /// enabled multiple stores. May be a comma-separated list of store
    /// identifiers.
    ///
    #[structable(optional, wide)]
    pub stores: Option<String>,

    /// List of strings related to the image
    ///
    #[structable(optional, serialize, wide)]
    pub tags: Option<Vec<String>>,

    /// Date and time of the last image modification
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// Virtual size of image in bytes
    ///
    #[structable(optional, serialize, wide)]
    pub virtual_size: Option<i64>,

    /// Scope of image accessibility
    ///
    #[structable(optional, serialize, wide)]
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
