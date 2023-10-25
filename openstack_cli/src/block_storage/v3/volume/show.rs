//! Get single Volume
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{error::OpenStackCliError, Command};
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::parse_key_val;
use crate::common::HashMapStringString;
use crate::common::VecValue;
use openstack_sdk::api::block_storage::v3::volume::find;
use openstack_sdk::api::block_storage::v3::volume::get;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::RestClient;
use serde_json::Value;

/// Get single Volume
#[derive(Args, Clone, Debug)]
pub struct VolumeArgs {
    /// The UUID of the project in a multi-tenancy cloud.
    #[arg(long)]
    project_id: Option<String>,

    /// Volume ID
    #[arg()]
    id: String,
}

pub struct VolumeCmd {
    pub args: VolumeArgs,
}

/// Volume
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Volume {
    /// Instance attachment information. If this volume is attached to a server
    /// instance, the attachments list includes the UUID of the attached
    /// server, an attachment UUID, the name of the attached host, if any, the
    /// volume UUID, the device, and the device UUID. Otherwise, this list is
    /// empty.
    #[structable(optional)]
    attachments: Option<VecValue>,

    /// The name of the availability zone.
    #[structable(optional)]
    availabilitiy_zone: Option<String>,

    /// Current back-end of the volume. Host format is host@backend#pool.
    #[serde(rename = "os-vol-host-attr:host")]
    #[structable(optional)]
    host: Option<String>,

    /// If true, this volume is encrypted.
    #[serde(rename = "encrypted")]
    #[structable(optional)]
    is_encrypted: Option<bool>,

    /// The UUID of the encryption key. Only included for encrypted volumes.
    /// New in version 3.64
    #[structable(optional)]
    encryption_key_id: Option<String>,

    /// The date and time when the resource was updated.
    /// The date and time stamp format is ISO 8601.
    #[structable(optional)]
    updated_at: Option<String>,

    /// The volume replication status.
    #[structable(optional)]
    replication_status: Option<String>,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    #[structable(optional)]
    snapshot_id: Option<String>,

    /// The UUID of the volume.
    #[structable(optional)]
    id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[structable(optional)]
    size: Option<u64>,

    /// The UUID of the user.
    #[structable(optional)]
    user_id: Option<String>,

    /// The status of this volume migration (None means that a migration is not
    /// currently in progress).
    #[serde(rename = "os-vol-mig-status-attr:migstat")]
    #[structable(optional)]
    migration_status: Option<String>,

    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    #[structable(optional)]
    metadata: Option<HashMapStringString>,

    /// The volume description.
    #[structable(optional)]
    status: Option<String>,

    /// List of image metadata entries. Only included for volumes that were
    /// created from an image, or from a snapshot of a volume originally
    /// created from an image.
    #[structable(optional)]
    volume_image_metadata: Option<HashMapStringString>,

    /// The volume description.
    #[structable(optional)]
    description: Option<String>,

    /// If true, this volume can attach to more than one instance.
    #[serde(rename = "multiattach")]
    #[structable(optional)]
    is_multiattach: Option<bool>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    #[structable(optional)]
    source_volid: Option<String>,

    /// The UUID of the consistency group.
    #[structable(optional)]
    consistencygroup_id: Option<String>,

    /// The volume ID that this volume name on the back- end is based on.
    #[serde(rename = "os-vol-mig-status-attr:name_id")]
    #[structable(optional)]
    migration_id: Option<String>,

    /// The volume name.
    #[structable(optional)]
    name: Option<String>,

    /// Enables or disables the bootable attribute. You can boot an instance
    /// from a bootable volume.
    #[structable(optional)]
    bootable: Option<String>,

    /// The date and time when the resource was created.
    /// The date and time stamp format is ISO 8601.
    #[structable(optional)]
    created_at: Option<String>,

    /// The associated volume type name for the volume.
    #[structable(optional)]
    volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    /// New in version 3.63
    #[structable(optional)]
    volume_type_id: Option<String>,

    /// The ID of the group.
    /// New in version 3.13
    #[structable(optional)]
    group_id: Option<String>,

    /// The volume links.
    #[structable(optional)]
    volumes_links: Option<VecValue>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or null if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    /// New in version 3.21
    #[structable(optional)]
    provider_id: Option<String>,

    /// A unique identifier that’s used to indicate what node the volume-
    /// service for a particular volume is being serviced by.
    /// New in version 3.48
    #[structable(optional)]
    service_uuid: Option<String>,

    /// An indicator whether the back-end hosting the volume utilizes
    /// shared_targets or not. Default=True.
    /// New in version 3.48
    /// Available until version 3.68
    #[structable(optional)]
    shared_targets: Option<bool>,

    /// The cluster name of volume backend.
    /// New in version 3.61
    #[structable(optional)]
    cluster_name: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    /// New in version 3.65
    #[structable(optional)]
    consumes_quota: Option<bool>,
}

#[async_trait]
impl Command for VolumeCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Volume with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = find::Volume::builder();
        // Set path parameters
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        } else {
            ep_builder.project_id(
                client
                    .get_current_project()
                    .expect("Project ID must be known")
                    .id,
            );
        }
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::BlockStorage)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<Volume>(data)?;
        Ok(())
    }
}
