//! Lists all Block Storage volumes, with details, that the project can access,
//! since v3.31 if non-admin users specify invalid filters in the url, API will
//! return bad request.
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
use openstack_sdk::api::block_storage::v3::volumes::detail::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// Lists all Block Storage volumes, with details, that the project can access,
/// since v3.31 if non-admin users specify invalid filters in the url, API will
/// return bad request.
#[derive(Args, Clone, Debug)]
pub struct VolumesArgs {
    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct VolumesCmd {
    pub args: VolumesArgs,
}

/// Volumes
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Volumes {
    /// Instance attachment information. If this volume is attached to a server
    /// instance, the attachments list includes the UUID of the attached
    /// server, an attachment UUID, the name of the attached host, if any, the
    /// volume UUID, the device, and the device UUID. Otherwise, this list is
    /// empty.
    #[structable(optional, wide)]
    attachments: Option<VecValue>,

    /// The name of the availability zone.
    #[structable(optional, wide)]
    availabilitiy_zone: Option<String>,

    /// Current back-end of the volume. Host format is host@backend#pool.
    #[serde(rename = "os-vol-host-attr:host")]
    #[structable(optional, wide)]
    host: Option<String>,

    /// If true, this volume is encrypted.
    #[serde(rename = "encrypted")]
    #[structable(optional, wide)]
    is_encrypted: Option<bool>,

    /// The UUID of the encryption key. Only included for encrypted volumes.
    /// New in version 3.64
    #[structable(optional, wide)]
    encryption_key_id: Option<String>,

    /// The date and time when the resource was updated.
    /// The date and time stamp format is ISO 8601.
    #[structable(optional)]
    updated_at: Option<String>,

    /// The volume replication status.
    #[structable(optional, wide)]
    replication_status: Option<String>,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    #[structable(optional, wide)]
    snapshot_id: Option<String>,

    /// The UUID of the volume.
    #[structable(optional)]
    id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[structable(optional, wide)]
    size: Option<u64>,

    /// The UUID of the user.
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// The status of this volume migration (None means that a migration is not
    /// currently in progress).
    #[serde(rename = "os-vol-mig-status-attr:migstat")]
    #[structable(optional, wide)]
    migration_status: Option<String>,

    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the volume.
    #[structable(optional, wide)]
    metadata: Option<HashMapStringString>,

    /// The volume description.
    #[structable(optional, wide)]
    status: Option<String>,

    /// List of image metadata entries. Only included for volumes that were
    /// created from an image, or from a snapshot of a volume originally
    /// created from an image.
    #[structable(optional, wide)]
    volume_image_metadata: Option<HashMapStringString>,

    /// The volume description.
    #[structable(optional, wide)]
    description: Option<String>,

    /// If true, this volume can attach to more than one instance.
    #[serde(rename = "multiattach")]
    #[structable(optional, wide)]
    is_multiattach: Option<bool>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    #[structable(optional, wide)]
    source_volid: Option<String>,

    /// The UUID of the consistency group.
    #[structable(optional, wide)]
    consistencygroup_id: Option<String>,

    /// The volume ID that this volume name on the back- end is based on.
    #[serde(rename = "os-vol-mig-status-attr:name_id")]
    #[structable(optional, wide)]
    migration_id: Option<String>,

    /// The volume name.
    #[structable(optional)]
    name: Option<String>,

    /// Enables or disables the bootable attribute. You can boot an instance
    /// from a bootable volume.
    #[structable(optional, wide)]
    bootable: Option<String>,

    /// The date and time when the resource was created.
    /// The date and time stamp format is ISO 8601.
    #[structable(optional)]
    created_at: Option<String>,

    /// The associated volume type name for the volume.
    #[structable(optional, wide)]
    volume_type: Option<String>,

    /// The associated volume type ID for the volume.
    /// New in version 3.63
    #[structable(optional, wide)]
    volume_type_id: Option<String>,

    /// The ID of the group.
    /// New in version 3.13
    #[structable(optional, wide)]
    group_id: Option<String>,

    /// The volume links.
    #[structable(optional, wide)]
    volumes_links: Option<VecValue>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or null if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    /// New in version 3.21
    #[structable(optional, wide)]
    provider_id: Option<String>,

    /// A unique identifier that’s used to indicate what node the volume-
    /// service for a particular volume is being serviced by.
    /// New in version 3.48
    #[structable(optional, wide)]
    service_uuid: Option<String>,

    /// An indicator whether the back-end hosting the volume utilizes
    /// shared_targets or not. Default=True.
    /// New in version 3.48
    /// Available until version 3.68
    #[structable(optional, wide)]
    shared_targets: Option<bool>,

    /// The cluster name of volume backend.
    /// New in version 3.61
    #[structable(optional, wide)]
    cluster_name: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    /// New in version 3.65
    #[structable(optional, wide)]
    consumes_quota: Option<bool>,
}

#[async_trait]
impl Command for VolumesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Volumes with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Volumes::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::BlockStorage)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Volumes>(data)?;
        Ok(())
    }
}
