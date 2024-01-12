//! Creates a catalog record for an operating system disk image.
//! *(Since Image API v2.0)*
//!
//! The `Location` response header contains the URI for the image.
//!
//! A multiple store backend support is introduced in the Rocky release
//! as a part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a
//! new header `OpenStack-image-store-ids` which contains the list of
//! available stores will be included in response. This header is only
//! included if multiple backend stores are supported.
//!
//! The response body contains the new image entity.
//!
//! Synchronous Postconditions
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 401, 403, 409, 413, 415
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
use openstack_sdk::api::image::v2::image::create;
use openstack_sdk::api::QueryAsync;
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
    id: Option<String>,
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
pub struct PathParameters {}

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

/// Image create command
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

    /// The virtual size of the image. The value might
    /// be `null` (JSON null data type).
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
        info!("Create Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create::Request::builder();

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.id data
        if let Some(args) = &self.args.id {
            ep_builder.id(args);
        }

        // Set Request.name data
        if let Some(args) = &self.args.name {
            ep_builder.name(Some(args.into()));
        }

        // Set Request.visibility data
        if let Some(args) = &self.args.visibility {
            let tmp = match args {
                Visibility::Community => create::Visibility::Community,
                Visibility::Private => create::Visibility::Private,
                Visibility::Public => create::Visibility::Public,
                Visibility::Shared => create::Visibility::Shared,
            };
            ep_builder.visibility(tmp);
        }

        // Set Request.protected data
        if let Some(args) = &self.args.protected {
            ep_builder.protected(*args);
        }

        // Set Request.os_hidden data
        if let Some(args) = &self.args.os_hidden {
            ep_builder.os_hidden(*args);
        }

        // Set Request.owner data
        if let Some(args) = &self.args.owner {
            ep_builder.owner(Some(args.into()));
        }

        // Set Request.container_format data
        if let Some(args) = &self.args.container_format {
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
        if let Some(args) = &self.args.disk_format {
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

        // Set Request.tags data
        if let Some(args) = &self.args.tags {
            ep_builder.tags(args.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        // Set Request.min_ram data
        if let Some(args) = &self.args.min_ram {
            ep_builder.min_ram(*args);
        }

        // Set Request.min_disk data
        if let Some(args) = &self.args.min_disk {
            ep_builder.min_disk(*args);
        }

        // Set Request.locations data
        if let Some(args) = &self.args.locations {
            let sub: Vec<create::Locations> = args
                .iter()
                .flat_map(|v| serde_json::from_value::<create::Locations>(v.clone()))
                .collect::<Vec<create::Locations>>();
            ep_builder.locations(sub);
        }

        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
