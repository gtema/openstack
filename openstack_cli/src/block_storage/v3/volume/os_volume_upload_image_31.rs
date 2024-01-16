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

use clap::ValueEnum;
use openstack_sdk::api::block_storage::v3::volume::find;
use openstack_sdk::api::block_storage::v3::volume::os_volume_upload_image_31;
use openstack_sdk::api::find;
use openstack_sdk::api::RawQueryAsync;
use serde_json::Value;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct VolumeArgs {
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
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v3/volumes/{id} API
    #[arg()]
    id: String,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum DiskFormat {
    Ploop,
    Qcow2,
    Raw,
    Vdi,
    Vhd,
    Vhdx,
    Vmdk,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Visibility {
    Community,
    Private,
    Public,
    Shared,
}

/// OsVolumeUploadImage Body data
#[derive(Args, Debug, Clone)]
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

/// Volume action command
pub struct VolumeCmd {
    pub args: VolumeArgs,
}
/// Volume response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {}

#[async_trait]
impl Command for VolumeCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Volume with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = os_volume_upload_image_31::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.1");

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_volume_upload_image data
        let args = &self.args.os_volume_upload_image;
        let mut os_volume_upload_image_builder =
            os_volume_upload_image_31::OsVolumeUploadImageBuilder::default();

        os_volume_upload_image_builder.image_name(&args.image_name);

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

        let rsp: Response<Bytes> = ep.raw_query_async(client).await?;
        let data = ResponseData {};
        // Maybe output some headers metadata
        op.output_human::<ResponseData>(&data)?;
        Ok(())
    }
}