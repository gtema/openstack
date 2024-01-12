//! Shows details for an image.
//! *(Since Image API v2.0)*
//!
//! The response body contains a single image entity.
//!
//! Preconditions
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404
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

use openstack_sdk::api::find;
use openstack_sdk::api::image::v2::image::find;
use openstack_sdk::api::image::v2::image::get;
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

/// Image show command
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
    size: Option<i64>,

    /// The virtual size of the image. The value might
    /// be `null` (JSON null data type).
    #[serde()]
    #[structable(optional, wide)]
    virtual_size: Option<i64>,

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
        return write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
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
        return write!(f, "{}", data.join(";"));
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
        return write!(f, "{}", data.join(";"));
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseLocations(Vec<ResponseLocations>);
impl fmt::Display for VecResponseLocations {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "[{}]",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }
}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;

        op.output_single::<ResponseData>(find_data)?;
        Ok(())
    }
}
