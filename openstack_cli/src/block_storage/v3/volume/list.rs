//! Returns a detailed list of volumes.
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

use openstack_sdk::api::block_storage::v3::volume::find;
use openstack_sdk::api::block_storage::v3::volume::list_detailed;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct VolumesArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {
    /// Shows details for all project. Admin only.
    #[arg(long)]
    all_tenans: Option<bool>,

    /// Comma-separated list of sort keys and optional sort directions in the
    /// form of < key > [: < direction > ]. A valid direction is asc
    /// (ascending) or desc (descending).
    #[arg(long)]
    sort: Option<String>,

    /// Sorts by an attribute. A valid value is name, status, container_format,
    /// disk_format, size, id, created_at, or updated_at. Default is
    /// created_at. The API uses the natural sorting direction of the sort_key
    /// attribute value. Deprecated in favour of the combined sort parameter.
    #[arg(long)]
    sort_key: Option<String>,

    /// Sorts by one or more sets of attribute and sort direction combinations.
    /// If you omit the sort direction in a set, default is desc. Deprecated in
    /// favour of the combined sort parameter.
    #[arg(long)]
    sort_dir: Option<String>,

    /// Requests a page size of items. Returns a number of items up to a limit
    /// value. Use the limit parameter to make an initial limited request and
    /// use the ID of the last-seen item from the response as the marker
    /// parameter value in a subsequent limited request.
    #[arg(long)]
    limit: Option<i32>,

    /// Used in conjunction with limit to return a slice of items. offset is
    /// where to start in the list.
    #[arg(long)]
    offset: Option<i32>,

    /// The ID of the last-seen item. Use the limit parameter to make an
    /// initial limited request and use the ID of the last-seen item from the
    /// response as the marker parameter value in a subsequent limited request.
    #[arg(long)]
    marker: Option<String>,

    /// Whether to show count in API response or not, default is False.
    #[arg(long)]
    with_count: Option<bool>,

    /// Filters reuslts by a time that resources are created at with time
    /// comparison operators: gt/gte/eq/neq/lt/lte.
    #[arg(long)]
    created_at: Option<String>,

    /// Filters reuslts by a time that resources are updated at with time
    /// comaprison operators: gt/gte/eq/neq/lt/lte.
    #[arg(long)]
    updated_at: Option<String>,

    /// Filters results by consumes_quota field. Resources that don’t use
    /// quotas are usually temporary internal resources created to perform an
    /// operation. Default is to not filter by it. Filtering by this option may
    /// not be always possible in a cloud, see List Resource Filters to
    /// determine whether this filter is available in your cloud.
    #[arg(long)]
    consumes_quota: Option<bool>,
}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {}

/// Volumes list command
pub struct VolumesCmd {
    pub args: VolumesArgs,
}
/// Volumes response representation
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
impl Command for VolumesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Volumes with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list_detailed::Request::builder();

        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.query.all_tenans {
            ep_builder.all_tenans(*val);
        }
        if let Some(val) = &self.args.query.sort {
            ep_builder.sort(val);
        }
        if let Some(val) = &self.args.query.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.args.query.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.args.query.limit {
            ep_builder.limit(*val);
        }
        if let Some(val) = &self.args.query.offset {
            ep_builder.offset(*val);
        }
        if let Some(val) = &self.args.query.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.query.with_count {
            ep_builder.with_count(*val);
        }
        if let Some(val) = &self.args.query.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.args.query.updated_at {
            ep_builder.updated_at(val);
        }
        if let Some(val) = &self.args.query.consumes_quota {
            ep_builder.consumes_quota(*val);
        }
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
