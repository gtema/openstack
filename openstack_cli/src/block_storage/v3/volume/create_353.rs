//! Creates a new volume.
//!
//! :param req: the request
//! :param body: the request body
//! :returns: dict -- the new volume dictionary
//! :raises HTTPNotFound, HTTPBadRequest:
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
use openstack_sdk::api::block_storage::v3::volume::create_353;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::BTreeMap;
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
    volume: Volume,
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    os_sch_hnt_scheduler_hints: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}
/// Volume Body data
#[derive(Args, Debug, Clone)]
struct Volume {
    #[arg(long)]
    name: Option<String>,

    #[arg(long)]
    description: Option<String>,

    #[arg(long)]
    display_name: Option<String>,

    #[arg(long)]
    display_description: Option<String>,

    #[arg(long)]
    volume_type: Option<String>,

    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    #[arg(long)]
    snapshot_id: Option<String>,

    #[arg(long)]
    source_volid: Option<String>,

    #[arg(long)]
    consistencygroup_id: Option<String>,

    #[arg(long)]
    size: Option<Option<i32>>,

    #[arg(long)]
    availability_zone: Option<String>,

    #[arg(action=clap::ArgAction::Set, long)]
    multiattach: Option<Option<bool>>,

    #[arg(long)]
    image_id: Option<String>,

    #[arg(long)]
    image_ref: Option<String>,

    #[arg(long)]
    group_id: Option<String>,

    #[arg(long)]
    backup_id: Option<String>,
}

/// Volume create command
pub struct VolumeCmd {
    pub args: VolumeArgs,
}
/// Volume response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The volume name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The volume description.
    #[serde()]
    #[structable(optional, wide)]
    description: Option<String>,

    /// The associated volume type name for the volume.
    #[serde()]
    #[structable(optional, wide)]
    volume_type: Option<String>,

    /// A metadata object. Contains one or more metadata key and value pairs
    /// that are associated with the resource.
    #[serde()]
    #[structable(optional, wide)]
    metadata: Option<HashMapStringString>,

    /// To create a volume from an existing snapshot, specify the UUID of the
    /// volume snapshot. The volume is created in same availability zone and
    /// with same size as the snapshot.
    #[serde()]
    #[structable(optional, wide)]
    snapshot_id: Option<String>,

    /// The UUID of the source volume. The API creates a new volume with the
    /// same size as the source volume unless a larger size is requested.
    #[serde()]
    #[structable(optional, wide)]
    source_volid: Option<String>,

    /// The UUID of the consistency group.
    #[serde()]
    #[structable(optional, wide)]
    consistencygroup_id: Option<String>,

    /// The size of the volume, in gibibytes (GiB).
    #[serde()]
    #[structable(optional, wide)]
    size: Option<i64>,

    /// The name of the availability zone.
    #[serde()]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// If true, this volume can attach to more than one instance.
    #[serde()]
    #[structable(optional, wide)]
    multiattach: Option<bool>,

    /// The volume migration status. Admin only.
    #[serde()]
    #[structable(optional, wide)]
    migration_status: Option<String>,

    /// Instance attachment information. If this volume is attached to a server
    /// instance, the attachments list includes the UUID of the attached
    /// server, an attachment UUID, the name of the attached host, if any, the
    /// volume UUID, the device, and the device UUID. Otherwise, this list is
    /// empty.
    #[serde()]
    #[structable(optional, wide)]
    attachments: Option<VecResponseAttachments>,

    /// Links to the resources in question. See [API Guide / Links and
    /// References](https://docs.openstack.org/api-
    /// guide/compute/links_and_references.html) for more info.
    #[serde()]
    #[structable(optional, wide)]
    links: Option<Value>,

    /// If true, this volume is encrypted.
    #[serde()]
    #[structable(optional, wide)]
    encrypted: Option<bool>,

    /// The date and time when the resource was created.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The volume replication status.
    #[serde()]
    #[structable(optional, wide)]
    replication_status: Option<String>,

    /// The UUID of the volume.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID of the user.
    #[serde()]
    #[structable(optional, wide)]
    user_id: Option<String>,

    /// The associated volume type ID for the volume.
    #[serde()]
    #[structable(optional, wide)]
    volume_type_id: Option<String>,

    /// The ID of the group.
    #[serde()]
    #[structable(optional, wide)]
    group_id: Option<String>,

    /// The provider ID for the volume. The value is either a string set by the
    /// driver or null if the driver doesn’t use the field or if it hasn’t
    /// created it yet. Only returned for administrators.
    #[serde()]
    #[structable(optional, wide)]
    provider_id: Option<String>,

    /// A unique identifier that’s used to indicate what node the volume-
    /// service for a particular volume is being serviced by.
    #[serde()]
    #[structable(optional, wide)]
    service_uuid: Option<String>,

    /// An indicator whether the host connecting the volume should lock for the
    /// whole attach/detach process or not. true means only is iSCSI initiator
    /// running on host doesn’t support manual scans, false means never use
    /// locks, and null means to always use locks. Look at os-brick’s
    /// guard_connection context manager. Default=True.
    #[serde()]
    #[structable(optional, wide)]
    shared_targets: Option<bool>,

    /// The cluster name of volume backend.
    #[serde()]
    #[structable(optional, wide)]
    cluster_name: Option<String>,

    /// Whether this resource consumes quota or not. Resources that not counted
    /// for quota usage are usually temporary internal resources created to
    /// perform an operation.
    #[serde()]
    #[structable(optional, wide)]
    consumes_quota: Option<bool>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct HashMapStringString(HashMap<String, String>);
impl fmt::Display for HashMapStringString {
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
struct ResponseAttachments {
    server_id: Option<String>,
    attachment_id: Option<String>,
    attached_at: Option<String>,
    host_name: Option<String>,
    volume_id: Option<String>,
    device: Option<String>,
    id: Option<String>,
}

impl fmt::Display for ResponseAttachments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "server_id={}",
                self.server_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "attachment_id={}",
                self.attachment_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "attached_at={}",
                self.attached_at
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "host_name={}",
                self.host_name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "volume_id={}",
                self.volume_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "device={}",
                self.device
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseAttachments(Vec<ResponseAttachments>);
impl fmt::Display for VecResponseAttachments {
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseLinks {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for ResponseLinks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "href={}",
                self.href
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "rel={}",
                self.rel
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}

#[async_trait]
impl Command for VolumeCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Volume with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_353::Request::builder();
        ep_builder.header("OpenStack-API-Version", "volume 3.53");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.volume data
        let args = &self.args.volume;
        let mut volume_builder = create_353::VolumeBuilder::default();
        if let Some(val) = &args.name {
            volume_builder.name(Some(val.into()));
        }

        if let Some(val) = &args.description {
            volume_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.display_name {
            volume_builder.display_name(Some(val.into()));
        }

        if let Some(val) = &args.display_description {
            volume_builder.display_description(Some(val.into()));
        }

        if let Some(val) = &args.volume_type {
            volume_builder.volume_type(Some(val.into()));
        }

        if let Some(val) = &args.metadata {
            volume_builder.metadata(val.iter().cloned());
        }

        if let Some(val) = &args.snapshot_id {
            volume_builder.snapshot_id(Some(val.into()));
        }

        if let Some(val) = &args.source_volid {
            volume_builder.source_volid(Some(val.into()));
        }

        if let Some(val) = &args.consistencygroup_id {
            volume_builder.consistencygroup_id(Some(val.into()));
        }

        if let Some(val) = &args.size {
            volume_builder.size(*val);
        }

        if let Some(val) = &args.availability_zone {
            volume_builder.availability_zone(Some(val.into()));
        }

        if let Some(val) = &args.multiattach {
            volume_builder.multiattach(*val);
        }

        if let Some(val) = &args.image_id {
            volume_builder.image_id(Some(val.into()));
        }

        if let Some(val) = &args.image_ref {
            volume_builder.image_ref(Some(val.into()));
        }

        if let Some(val) = &args.group_id {
            volume_builder.group_id(Some(val.into()));
        }

        if let Some(val) = &args.backup_id {
            volume_builder.backup_id(Some(val.into()));
        }

        ep_builder.volume(volume_builder.build().unwrap());

        // Set Request.os_sch_hnt_scheduler_hints data
        if let Some(args) = &self.args.os_sch_hnt_scheduler_hints {
            ep_builder.os_sch_hnt_scheduler_hints(args.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
