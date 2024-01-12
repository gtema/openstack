//! Updates an image.
//! *(Since Image API v2.0)*
//!
//! Conceptually, you update an image record by patching the JSON
//! representation of
//! the image, passing a request body conforming to one of the following media
//! types:
//!
//! Attempting to make a PATCH call using some other media type will provoke a
//! response code of 415 (Unsupported media type).
//!
//! The `application/openstack-images-v2.1-json-patch` media type provides a
//! useful and compatible subset of the functionality defined in JavaScript
//! Object
//! Notation (JSON) Patch [RFC6902](http://tools.ietf.org/html/rfc6902), which
//! defines the `application/json-patch+json` media type.
//!
//! For information about the PATCH method and the available media types, see
//! [Image API v2 HTTP PATCH media
//! types](http://specs.openstack.org/openstack/glance-specs/specs/api/v2/http-
//! patch-image-api-v2.html).
//!
//! Attempting to modify some image properties will cause the entire request to
//! fail with a 403 (Forbidden) response code:
//!
//! Attempting to add a location path to an image that is not in `queued` or
//! `active` state will result in a 409 (Conflict) response code
//! *(since Image API v2.4)*.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404, 409, 413, 415
//!
use async_trait::async_trait;
use bytes::Bytes;
use clap::Args;
use http::Response;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::parse_key_val;
use clap::ValueEnum;
use json_patch::{diff, Patch};
use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::image::v2::image::patch;
use openstack_sdk::api::QueryAsync;
use serde_json::json;
use serde_json::to_value;
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long)]
    name: Option<String>,
    #[arg(long)]
    visibility: Option<Visibility>,
    #[arg(action=clap::ArgAction::Set, long)]
    protected: Option<bool>,
    #[arg(action=clap::ArgAction::Set, long)]
    os_hidden: Option<bool>,
    #[arg(long)]
    owner: Option<String>,
    #[arg(long)]
    container_format: Option<ContainerFormat>,
    #[arg(long)]
    disk_format: Option<DiskFormat>,
    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,
    #[arg(long)]
    min_ram: Option<i32>,
    #[arg(long)]
    min_disk: Option<i32>,
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    locations: Option<Vec<Value>>,
    /// Additional properties to be sent with the request
    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, String>)]
    properties: Option<Vec<(String, String)>>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    id: String,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Visibility {
    Community,
    Private,
    Public,
    Shared,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
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

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
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

/// ValidationData Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ValidationData {
    #[arg(long)]
    checksum: Option<String>,

    #[arg(long, required = false)]
    os_hash_algo: String,

    #[arg(long, required = false)]
    os_hash_value: String,
}

/// Image set command
pub struct ImageCmd {
    pub args: ImageArgs,
}
/// Image response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// A unique, user-defined image UUID, in the format:
    ///
    ///
    ///
    /// ```text
    /// nnnnnnnn-nnnn-nnnn-nnnn-nnnnnnnnnnnn
    ///
    /// ```
    ///
    ///
    /// Where **n** is a hexadecimal digit from 0 to f, or F.
    ///
    ///
    /// For example:
    ///
    ///
    ///
    /// ```text
    /// b2173dd3-7ad6-4362-baa6-a68bce3565cb
    ///
    /// ```
    ///
    ///
    /// If you omit this value, the API generates a UUID for the image.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The name of the image. Value might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The image status.
    #[serde()]
    #[structable(optional, wide)]
    status: Option<String>,

    /// Image visibility, that is, the access permission for the image.
    #[serde()]
    #[structable(optional, wide)]
    visibility: Option<String>,

    /// A boolean value that must be `false` or the image cannot be deleted.
    #[serde()]
    #[structable(optional, wide)]
    protected: Option<bool>,

    /// This field controls whether an image is displayed in the default
    /// image-list response. A “hidden” image is out of date somehow (for
    /// example, it may not have the latest updates applied) and hence should
    /// not be a user’s first choice, but it’s not deleted because it may be
    /// needed for server rebuilds. By hiding it from the default image list,
    /// it’s easier for end users to find and use a more up-to-date version of
    /// this image. *(Since Image API v2.7)*
    #[serde()]
    #[structable(optional, wide)]
    os_hidden: Option<bool>,

    /// An MD5 hash over the image data. The value might be `null` (JSON null
    /// data type),
    /// as this field is no longer populated by the Image Service beginning
    /// with the
    /// Victoria release. It remains present for backward compatibility with
    /// legacy
    /// images. To validate image data, instead use the secure multihash fields
    /// `os\_hash\_algo` and `os\_hash\_value`.
    #[serde()]
    #[structable(optional, wide)]
    checksum: Option<String>,

    /// The algorithm used to compute a secure hash of the image data for this
    /// image. The result of the computation is displayed as the value of the
    /// `os\_hash\_value` property. The value might be `null` (JSON null
    /// data type). The algorithm used is chosen by the cloud operator; it
    /// may not be configured by end users. *(Since Image API v2.7)*
    #[serde()]
    #[structable(optional, wide)]
    os_hash_algo: Option<String>,

    /// The hexdigest of the secure hash of the image data computed using the
    /// algorithm whose name is the value of the `os\_hash\_algo` property.
    /// The value might be `null` (JSON null data type) if data has not
    /// yet been associated with this image, or if the image was created using
    /// a version of the Image Service API prior to version 2.7.
    /// *(Since Image API v2.7)*
    #[serde()]
    #[structable(optional, wide)]
    os_hash_value: Option<String>,

    /// An identifier for the owner of the image, usually the project (also
    /// called the “tenant”) ID.
    /// The value might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    owner: Option<String>,

    /// The size of the image data, in bytes. The value
    /// might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    size: Option<i32>,

    /// Virtual size of image in bytes
    #[serde()]
    #[structable(optional, wide)]
    virtual_size: Option<i32>,

    /// Format of the image container.
    ///
    ///
    /// Values may vary based on the configuration available in a
    /// particular OpenStack cloud. See the [Image Schema](#image-schema)
    /// response from the cloud itself for the valid values available.
    ///
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `bare`,
    /// `ovf`, `ova`, or `docker`.
    ///
    ///
    /// The value might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    container_format: Option<String>,

    /// The format of the disk.
    ///
    ///
    /// Values may vary based on the configuration available in a
    /// particular OpenStack cloud. See the [Image Schema](#image-schema)
    /// response from the cloud itself for the valid values available.
    ///
    ///
    /// Example formats are: `ami`, `ari`, `aki`, `vhd`,
    /// `vhdx`, `vmdk`, `raw`, `qcow2`, `vdi`, `ploop` or
    /// `iso`.
    ///
    ///
    /// The value might be `null` (JSON null data type).
    ///
    ///
    /// **Newton changes**: The `vhdx` disk format is a supported
    /// value.
    ///
    /// **Ocata changes**: The `ploop` disk format is a supported
    /// value.
    #[serde()]
    #[structable(optional, wide)]
    disk_format: Option<String>,

    /// The date and time when the resource was created.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC. In the previous example, the offset value is `-05:00`.
    ///
    ///
    /// If the `updated\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// List of tags for this image, possibly an empty list.
    #[serde()]
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The URL to access the image file kept in external store. *It is present
    /// only if the* `show\_image\_direct\_url` *option is* `true` *in the
    /// Image
    /// service’s configuration file.* **Because it presents a security risk,
    /// this
    /// option is disabled by default.**
    #[serde()]
    #[structable(optional, wide)]
    direct_url: Option<String>,

    /// Amount of RAM in MB that is required to boot the image.
    /// The value might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    min_ram: Option<i32>,

    /// Amount of disk space in GB that is required to boot the image.
    /// The value might be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    min_disk: Option<i32>,

    /// The URL for the virtual machine image.
    #[serde(rename = "self")]
    #[structable(optional, title = "self", wide)]
    _self: Option<String>,

    /// The URL for the virtual machine image file.
    #[serde()]
    #[structable(optional, wide)]
    file: Option<String>,

    /// Store in which image data resides.  Only present when the operator has
    /// enabled multiple stores.  May be a comma-separated list of store
    /// identifiers.
    #[serde()]
    #[structable(optional, wide)]
    stores: Option<String>,

    /// The URL for the schema describing a virtual machine image.
    #[serde()]
    #[structable(optional, wide)]
    schema: Option<String>,

    /// A list of objects, each of which describes an image location. Each
    /// object
    /// contains a `url` key, whose value is a URL specifying a location, and a
    /// `metadata` key, whose value is a dict of key:value pairs containing
    /// information appropriate to the use of whatever external store is
    /// indicated
    /// by the URL. *This list appears only if the* `show\_multiple\_locations`
    /// *option is set to* `true` *in the Image service’s configuration file.*
    /// **Because it presents a security risk, this option is disabled by
    /// default.**
    #[serde()]
    #[structable(optional, wide)]
    locations: Option<VecResponseLocations>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringValue(HashMap<String, Value>);
impl fmt::Display for HashMapStringValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.0
                .iter()
                .map(|v| format!("{}={}", v.0, v.1))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseValidationData {
    checksum: Option<String>,
    os_hash_algo: String,
    os_hash_value: String,
}

impl fmt::Display for ResponseValidationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "checksum={}",
                self.checksum
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!("os_hash_algo={}", self.os_hash_algo),
            format!("os_hash_value={}", self.os_hash_value),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseLocations {
    url: String,
    metadata: HashMapStringValue,
    validation_data: Option<ResponseValidationData>,
}

impl fmt::Display for ResponseLocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("url={}", self.url),
            format!("metadata={}", self.metadata),
            format!(
                "validation_data={}",
                self.validation_data
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseLocations(Vec<ResponseLocations>);
impl fmt::Display for VecResponseLocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
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
        if let Some(val) = &self.args.name {
            new.name = Some(val.into());
        }
        if let Some(val) = &self.args.visibility {
            // StringEnum
            let tmp = match val {
                Visibility::Community => "community",
                Visibility::Private => "private",
                Visibility::Public => "public",
                Visibility::Shared => "shared",
            };
            new.visibility = Some(tmp.to_string());
        }
        if let Some(val) = &self.args.protected {
            new.protected = Some(*val);
        }
        if let Some(val) = &self.args.os_hidden {
            new.os_hidden = Some(*val);
        }
        if let Some(val) = &self.args.owner {
            new.owner = Some(val.into());
        }
        if let Some(val) = &self.args.container_format {
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
        if let Some(val) = &self.args.disk_format {
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
        if let Some(val) = &self.args.tags {
            new.tags = Some(VecString(val.clone()));
        }
        if let Some(val) = &self.args.min_ram {
            new.min_ram = Some(*val);
        }
        if let Some(val) = &self.args.min_disk {
            new.min_disk = Some(*val);
        }
        if let Some(val) = &self.args.locations {
            new.locations = Some(serde_json::from_value(serde_json::Value::from(
                val.clone(),
            ))?);
        }

        let curr_json = serde_json::to_value(&data).unwrap();
        let mut new_json = serde_json::to_value(&new).unwrap();
        if let Some(properties) = &self.args.properties {
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