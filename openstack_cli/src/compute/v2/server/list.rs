//! List detailed Servers
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
use crate::common::VecString;
use crate::common::VecValue;
use openstack_sdk::api::compute::v2::servers::detail::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// List detailed Servers
#[derive(Args, Clone, Debug)]
pub struct ServersArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// auto_disk_config filter parameter
    #[arg(long)]
    auto_disk_config: Option<String>,

    /// availability_zone filter parameter
    #[arg(long)]
    availability_zone: Option<String>,

    /// created_at filter parameter
    #[arg(long)]
    created_at: Option<String>,

    /// description filter parameter
    #[arg(long)]
    description: Option<String>,

    /// flavor filter parameter
    #[arg(long)]
    flavor: Option<String>,

    /// hostname filter parameter
    #[arg(long)]
    hostname: Option<String>,

    /// image filter parameter
    #[arg(long)]
    image: Option<String>,

    /// kernel_id filter parameter
    #[arg(long)]
    kernel_id: Option<String>,

    /// key_name filter parameter
    #[arg(long)]
    key_name: Option<String>,

    /// launch_index filter parameter
    #[arg(long)]
    launch_index: Option<u32>,

    /// launched_at filter parameter
    #[arg(long)]
    launched_at: Option<String>,

    /// locked_by filter parameter
    #[arg(long)]
    locked_by: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// node filter parameter
    #[arg(long)]
    node: Option<String>,

    /// power_state filter parameter
    #[arg(long)]
    power_state: Option<String>,

    /// progress filter parameter
    #[arg(long)]
    progress: Option<u32>,

    /// project_id filter parameter
    #[arg(long)]
    project_id: Option<String>,

    /// ramdisk_id filter parameter
    #[arg(long)]
    ramdisk_id: Option<String>,

    /// reservation_id filter parameter
    #[arg(long)]
    reservation_id: Option<String>,

    /// root_device_name filter parameter
    #[arg(long)]
    root_device_name: Option<String>,

    /// status filter parameter
    #[arg(long)]
    status: Option<String>,

    /// task_state filter parameter
    #[arg(long)]
    task_state: Option<String>,

    /// terminated_at filter parameter
    #[arg(long)]
    terminated_at: Option<String>,

    /// user_id filter parameter
    #[arg(long)]
    user_id: Option<String>,

    /// vm_state filter parameter
    #[arg(long)]
    vm_state: Option<String>,

    /// sort_key filter parameter
    #[arg(long)]
    sort_key: Option<String>,

    /// sort_dir filter parameter
    #[arg(long)]
    sort_dir: Option<String>,

    /// access_ipv4 filter parameter
    #[arg(long)]
    access_ip_v4: Option<String>,

    /// access_ipv6 filter parameter
    #[arg(long)]
    access_ip_v6: Option<String>,

    /// has_config_drive filter parameter
    #[arg(long)]
    config_drive: Option<String>,

    /// deleted_only filter parameter
    #[arg(long)]
    deleted: Option<String>,

    /// compute_host filter parameter
    #[arg(long)]
    host: Option<String>,

    /// is_soft_deleted filter parameter
    #[arg(long)]
    soft_deleted: Option<String>,

    /// ipv4_address filter parameter
    #[arg(long)]
    ip: Option<String>,

    /// ipv6_address filter parameter
    #[arg(long)]
    ip6: Option<String>,

    /// changes_since filter parameter
    #[arg(long)]
    changes_since: Option<String>,

    /// changes_before filter parameter
    #[arg(long)]
    changes_before: Option<String>,

    /// id filter parameter
    #[arg(long)]
    uuid: Option<String>,

    /// all_projects filter parameter
    #[arg(long)]
    all_tenants: Option<String>,

    /// tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    tags_any: Option<Vec<String>>,

    /// not_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_tags: Option<Vec<String>>,

    /// not_any_tags filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    not_tags_any: Option<Vec<String>>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct ServersCmd {
    pub args: ServersArgs,
}

/// Servers
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Servers {
    /// The disk configuration. Either AUTO or MANUAL.
    #[serde(rename = "OS-DCF:diskConfig")]
    #[structable(optional, wide)]
    disk_config: Option<String>,

    /// The name of the availability zone this server is a part of.
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    #[structable(optional, wide)]
    availability_zone: Option<String>,

    /// The name of the compute host on which this instance is running. Appears
    /// in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    #[structable(optional, wide)]
    compute_host: Option<String>,

    /// The hostname set on the instance when it is booted. By default, it
    /// appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:hostname")]
    #[structable(optional, wide)]
    hostname: Option<String>,

    /// The hypervisor host name. Appears in the response for administrative
    /// users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    #[structable(optional, wide)]
    hypervisor_hostname: Option<String>,

    /// The instance name. The Compute API generates the instance name from the
    /// instance name template. Appears in the response for administrative
    /// users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    #[structable(optional, wide)]
    instance_name: Option<String>,

    /// The UUID of the kernel image when using an AMI. Will be null if not. By
    /// default, it appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:kernel_id")]
    #[structable(optional, wide)]
    kernel_id: Option<String>,

    /// When servers are launched via multiple create, this is the sequence in
    /// which the servers were launched. By default, it appears in the response
    /// for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:launch_index")]
    #[structable(optional, wide)]
    launch_index: Option<u32>,

    /// The UUID of the ramdisk image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:ramdisk_id")]
    #[structable(optional, wide)]
    ramdisk_id: Option<String>,

    /// The reservation id for the server. This is an id that can be useful in
    /// tracking groups of servers created with multiple create, that will all
    /// have the same reservation_id. By default, it appears in the response
    /// for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:reservation_id")]
    #[structable(optional, wide)]
    reservation_id: Option<String>,

    /// The root device name for the instance By default, it appears in the
    /// response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:root_device_name")]
    #[structable(optional, wide)]
    root_device_name: Option<String>,

    /// Configuration information or scripts to use upon launch. Must be Base64
    /// encoded.
    #[serde(rename = "OS-EXT-SRV-ATTR:user_data")]
    #[structable(optional, wide)]
    user_data: Option<String>,

    /// The power state of this server.
    #[serde(rename = "OS-EXT-STS:power_state")]
    #[structable(optional, wide)]
    power_state: Option<String>,

    /// The task state of this server.
    #[serde(rename = "OS-EXT-STS:task_state")]
    #[structable(optional, wide)]
    task_state: Option<String>,

    /// The VM state of this server.
    #[serde(rename = "OS-EXT-STS:vm_state")]
    #[structable(optional, wide)]
    vm_state: Option<String>,

    /// The dictionary of data to send to the scheduler.
    #[serde(rename = "OS-SCH-HNT:scheduler_hints")]
    #[structable(wide)]
    scheduler_hints: HashMapStringString,

    /// The timestamp when the server was launched.
    #[serde(rename = "OS-SRV-USG:launched_at")]
    #[structable(optional, wide)]
    launched_at: Option<String>,

    /// The timestamp when the server was terminated (if it has been).
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    #[structable(optional, wide)]
    terminated_at: Option<u32>,

    /// None
    #[serde(rename = "accessIPv4")]
    #[structable(optional, wide)]
    access_ipv4: Option<String>,

    /// None
    #[serde(rename = "accessIPv6")]
    #[structable(optional, wide)]
    access_ipv6: Option<String>,

    /// A dictionary of addresses this server can be accessed through. The
    /// dictionary contains keys such as ``private`` and ``public``, each
    /// containing a list of dictionaries for addresses of that type. The
    /// addresses are contained in a dictionary with keys ``addr`` and
    /// ``version``, which is either 4 or 6 depending on the protocol of the IP
    /// address.
    #[structable(wide)]
    addresses: HashMapStringString,

    /// When a server is first created, it provides the administrator password.
    #[serde(rename = "adminPass")]
    #[structable(optional, wide)]
    admin_password: Option<u32>,

    /// Enables fine grained control of the block device mapping for an
    /// instance. This is typically used for booting servers from volumes.
    #[serde(rename = "block_device_mapping_v2")]
    #[structable(optional, wide)]
    block_device_mapping: Option<String>,

    /// Indicates whether a configuration drive enables metadata injection. Not
    /// all cloud providers enable this feature.
    #[serde(rename = "config_drive")]
    #[structable(optional, wide)]
    has_config_drive: Option<String>,

    /// Timestamp of when the server was created.
    #[serde(rename = "created")]
    #[structable(optional)]
    created_at: Option<String>,

    /// The description of the server. Before microversion 2.19 this was set to
    /// the server name.
    #[structable(optional, wide)]
    description: Option<String>,

    /// A fault object. Only available when the server status is ERROR or
    /// DELETED and a fault occurred.
    #[structable(optional, wide)]
    fault: Option<String>,

    /// The flavor property as returned from server.
    #[structable(wide)]
    flavor: HashMapStringString,

    /// The flavor reference, as a ID or full URL, for the flavor to use for
    /// this server.
    #[serde(rename = "flavorRef")]
    #[structable(optional, wide)]
    flavor_id: Option<String>,

    /// An ID representing the host of this server.
    #[serde(rename = "hostId")]
    #[structable(optional, wide)]
    host_id: Option<String>,

    /// The host status.
    #[structable(optional, wide)]
    host_status: Option<String>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The image property as returned from server.
    #[structable(wide)]
    image: HashMapStringString,

    /// The image reference, as a ID or full URL, for the image to use for this
    /// server.
    #[serde(rename = "imageRef")]
    #[structable(optional, wide)]
    image_id: Option<String>,

    /// The name of an associated keypair
    #[structable(optional, wide)]
    key_name: Option<String>,

    /// A list of dictionaries holding links relevant to this server.
    #[structable(optional, wide)]
    links: Option<String>,

    /// True if the instance is locked otherwise False.
    /// New in version 2.9
    #[serde(rename = "locked")]
    #[structable(optional, wide)]
    is_locked: Option<bool>,

    /// The maximum number of servers to create.
    #[structable(optional, wide)]
    max_count: Option<u32>,

    /// A dictionary of metadata key-and-value pairs, which is maintained for
    /// backward compatibility.
    #[structable(wide)]
    metadata: HashMapStringString,

    /// The minimum number of servers to create.
    #[structable(optional, wide)]
    min_count: Option<u32>,

    /// Name
    #[structable(optional)]
    name: Option<String>,

    /// A networks object. Required parameter when there are multiple networks
    /// defined for the tenant. When you do not specify the networks parameter,
    /// the server attaches to the only network created for the current tenant.
    #[structable(optional, wide)]
    networks: Option<String>,

    /// A list of an attached volumes. Each item in the list contains at least
    /// an "id" key to identify the specific volumes.
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    #[structable(optional, wide)]
    attached_volumes: Option<VecValue>,

    /// While the server is building, this value represents the percentage of
    /// completion. Once it is completed, it will be 100.
    #[structable(optional, wide)]
    progress: Option<u32>,

    /// A list of applicable security groups. Each group contains keys for
    /// description, name, id, and rules.
    #[structable(optional, wide)]
    security_groups: Option<VecValue>,

    /// The UUIDs of the server groups to which the server belongs. Currently
    /// this can contain at most one entry.
    #[structable(optional, wide)]
    server_groups: Option<VecString>,

    /// The state this server is in. Valid values include ``ACTIVE``,
    /// ``BUILDING``, ``DELETED``, ``ERROR``, ``HARD_REBOOT``, ``PASSWORD``,
    /// ``PAUSED``, ``REBOOT``, ``REBUILD``, ``RESCUED``, ``RESIZED``,
    /// ``REVERT_RESIZE``, ``SHUTOFF``, ``SOFT_DELETED``, ``STOPPED``,
    /// ``SUSPENDED``, ``UNKNOWN``, or ``VERIFY_RESIZE``.
    #[structable(optional, wide)]
    status: Option<String>,

    /// Server Tags.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The ID of the project this server is associated with.
    #[serde(rename = "tenant_id")]
    #[structable(optional, wide)]
    project_id: Option<String>,

    /// A list of trusted certificate IDs, that were used during image
    /// signature verification to verify the signing certificate.
    #[structable(optional, wide)]
    trusted_image_certificates: Option<VecString>,

    /// Timestamp of when this server was last updated.
    #[serde(rename = "updated")]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The ID of the owners of this server.
    #[structable(optional, wide)]
    user_id: Option<String>,
}

#[async_trait]
impl Command for ServersCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Servers with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Servers::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.auto_disk_config {
            ep_builder.auto_disk_config(val);
        }
        if let Some(val) = &self.args.availability_zone {
            ep_builder.availability_zone(val);
        }
        if let Some(val) = &self.args.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.args.description {
            ep_builder.description(val);
        }
        if let Some(val) = &self.args.flavor {
            ep_builder.flavor(val);
        }
        if let Some(val) = &self.args.hostname {
            ep_builder.hostname(val);
        }
        if let Some(val) = &self.args.image {
            ep_builder.image(val);
        }
        if let Some(val) = &self.args.kernel_id {
            ep_builder.kernel_id(val);
        }
        if let Some(val) = &self.args.key_name {
            ep_builder.key_name(val);
        }
        if let Some(val) = &self.args.launch_index {
            ep_builder.launch_index(*val);
        }
        if let Some(val) = &self.args.launched_at {
            ep_builder.launched_at(val);
        }
        if let Some(val) = &self.args.locked_by {
            ep_builder.locked_by(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.node {
            ep_builder.node(val);
        }
        if let Some(val) = &self.args.power_state {
            ep_builder.power_state(val);
        }
        if let Some(val) = &self.args.progress {
            ep_builder.progress(*val);
        }
        if let Some(val) = &self.args.project_id {
            ep_builder.project_id(val);
        }
        if let Some(val) = &self.args.ramdisk_id {
            ep_builder.ramdisk_id(val);
        }
        if let Some(val) = &self.args.reservation_id {
            ep_builder.reservation_id(val);
        }
        if let Some(val) = &self.args.root_device_name {
            ep_builder.root_device_name(val);
        }
        if let Some(val) = &self.args.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.task_state {
            ep_builder.task_state(val);
        }
        if let Some(val) = &self.args.terminated_at {
            ep_builder.terminated_at(val);
        }
        if let Some(val) = &self.args.user_id {
            ep_builder.user_id(val);
        }
        if let Some(val) = &self.args.vm_state {
            ep_builder.vm_state(val);
        }
        if let Some(val) = &self.args.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.args.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.args.access_ip_v4 {
            ep_builder.access_ip_v4(val);
        }
        if let Some(val) = &self.args.access_ip_v6 {
            ep_builder.access_ip_v6(val);
        }
        if let Some(val) = &self.args.config_drive {
            ep_builder.config_drive(val);
        }
        if let Some(val) = &self.args.deleted {
            ep_builder.deleted(val);
        }
        if let Some(val) = &self.args.host {
            ep_builder.host(val);
        }
        if let Some(val) = &self.args.soft_deleted {
            ep_builder.soft_deleted(val);
        }
        if let Some(val) = &self.args.ip {
            ep_builder.ip(val);
        }
        if let Some(val) = &self.args.ip6 {
            ep_builder.ip6(val);
        }
        if let Some(val) = &self.args.changes_since {
            ep_builder.changes_since(val);
        }
        if let Some(val) = &self.args.changes_before {
            ep_builder.changes_before(val);
        }
        if let Some(val) = &self.args.uuid {
            ep_builder.uuid(val);
        }
        if let Some(val) = &self.args.all_tenants {
            ep_builder.all_tenants(val);
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter());
        }
        if let Some(val) = &self.args.tags_any {
            ep_builder.tags_any(val.iter());
        }
        if let Some(val) = &self.args.not_tags {
            ep_builder.not_tags(val.iter());
        }
        if let Some(val) = &self.args.not_tags_any {
            ep_builder.not_tags_any(val.iter());
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Servers>(data)?;
        Ok(())
    }
}
