//! Get single Server
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
use openstack_sdk::api::compute::v2::server::find;
use openstack_sdk::api::compute::v2::server::get;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Get single Server
#[derive(Args, Clone, Debug)]
pub struct ServerArgs {
    /// Server ID
    #[arg()]
    id: String,
}

pub struct ServerCmd {
    pub args: ServerArgs,
}

/// Server
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Server {
    /// The disk configuration. Either AUTO or MANUAL.
    #[serde(rename = "OS-DCF:diskConfig")]
    #[structable(optional)]
    disk_config: Option<String>,

    /// The name of the availability zone this server is a part of.
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    #[structable(optional)]
    availability_zone: Option<String>,

    /// The name of the compute host on which this instance is running. Appears
    /// in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    #[structable(optional)]
    compute_host: Option<String>,

    /// The hostname set on the instance when it is booted. By default, it
    /// appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:hostname")]
    #[structable(optional)]
    hostname: Option<String>,

    /// The hypervisor host name. Appears in the response for administrative
    /// users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    #[structable(optional)]
    hypervisor_hostname: Option<String>,

    /// The instance name. The Compute API generates the instance name from the
    /// instance name template. Appears in the response for administrative
    /// users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    #[structable(optional)]
    instance_name: Option<String>,

    /// The UUID of the kernel image when using an AMI. Will be null if not. By
    /// default, it appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:kernel_id")]
    #[structable(optional)]
    kernel_id: Option<String>,

    /// When servers are launched via multiple create, this is the sequence in
    /// which the servers were launched. By default, it appears in the response
    /// for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:launch_index")]
    #[structable(optional)]
    launch_index: Option<u32>,

    /// The UUID of the ramdisk image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:ramdisk_id")]
    #[structable(optional)]
    ramdisk_id: Option<String>,

    /// The reservation id for the server. This is an id that can be useful in
    /// tracking groups of servers created with multiple create, that will all
    /// have the same reservation_id. By default, it appears in the response
    /// for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:reservation_id")]
    #[structable(optional)]
    reservation_id: Option<String>,

    /// The root device name for the instance By default, it appears in the
    /// response for administrative users only.
    #[serde(rename = "OS-EXT-SRV-ATTR:root_device_name")]
    #[structable(optional)]
    root_device_name: Option<String>,

    /// Configuration information or scripts to use upon launch. Must be Base64
    /// encoded.
    #[serde(rename = "OS-EXT-SRV-ATTR:user_data")]
    #[structable(optional)]
    user_data: Option<String>,

    /// The power state of this server.
    #[serde(rename = "OS-EXT-STS:power_state")]
    #[structable(optional)]
    power_state: Option<String>,

    /// The task state of this server.
    #[serde(rename = "OS-EXT-STS:task_state")]
    #[structable(optional)]
    task_state: Option<String>,

    /// The VM state of this server.
    #[serde(rename = "OS-EXT-STS:vm_state")]
    #[structable(optional)]
    vm_state: Option<String>,

    /// The dictionary of data to send to the scheduler.
    #[serde(rename = "OS-SCH-HNT:scheduler_hints")]
    scheduler_hints: HashMapStringString,

    /// The timestamp when the server was launched.
    #[serde(rename = "OS-SRV-USG:launched_at")]
    #[structable(optional)]
    launched_at: Option<String>,

    /// The timestamp when the server was terminated (if it has been).
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    #[structable(optional)]
    terminated_at: Option<u32>,

    /// None
    #[serde(rename = "accessIPv4")]
    #[structable(optional)]
    access_ipv4: Option<String>,

    /// None
    #[serde(rename = "accessIPv6")]
    #[structable(optional)]
    access_ipv6: Option<String>,

    /// A dictionary of addresses this server can be accessed through. The
    /// dictionary contains keys such as ``private`` and ``public``, each
    /// containing a list of dictionaries for addresses of that type. The
    /// addresses are contained in a dictionary with keys ``addr`` and
    /// ``version``, which is either 4 or 6 depending on the protocol of the IP
    /// address.
    addresses: HashMapStringString,

    /// When a server is first created, it provides the administrator password.
    #[serde(rename = "adminPass")]
    #[structable(optional)]
    admin_password: Option<u32>,

    /// Enables fine grained control of the block device mapping for an
    /// instance. This is typically used for booting servers from volumes.
    #[serde(rename = "block_device_mapping_v2")]
    #[structable(optional)]
    block_device_mapping: Option<String>,

    /// Indicates whether a configuration drive enables metadata injection. Not
    /// all cloud providers enable this feature.
    #[serde(rename = "config_drive")]
    #[structable(optional)]
    has_config_drive: Option<String>,

    /// Timestamp of when the server was created.
    #[serde(rename = "created")]
    #[structable(optional)]
    created_at: Option<String>,

    /// The description of the server. Before microversion 2.19 this was set to
    /// the server name.
    #[structable(optional)]
    description: Option<String>,

    /// A fault object. Only available when the server status is ERROR or
    /// DELETED and a fault occurred.
    #[structable(optional)]
    fault: Option<String>,

    /// The flavor property as returned from server.
    flavor: HashMapStringString,

    /// The flavor reference, as a ID or full URL, for the flavor to use for
    /// this server.
    #[serde(rename = "flavorRef")]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// An ID representing the host of this server.
    #[serde(rename = "hostId")]
    #[structable(optional)]
    host_id: Option<String>,

    /// The host status.
    #[structable(optional)]
    host_status: Option<String>,

    /// Id of the resource
    #[structable(optional)]
    id: Option<String>,

    /// The image property as returned from server.
    image: HashMapStringString,

    /// The image reference, as a ID or full URL, for the image to use for this
    /// server.
    #[serde(rename = "imageRef")]
    #[structable(optional)]
    image_id: Option<String>,

    /// The name of an associated keypair
    #[structable(optional)]
    key_name: Option<String>,

    /// A list of dictionaries holding links relevant to this server.
    #[structable(optional)]
    links: Option<String>,

    /// True if the instance is locked otherwise False.
    /// New in version 2.9
    #[serde(rename = "locked")]
    #[structable(optional)]
    is_locked: Option<bool>,

    /// The maximum number of servers to create.
    #[structable(optional)]
    max_count: Option<u32>,

    /// A dictionary of metadata key-and-value pairs, which is maintained for
    /// backward compatibility.
    metadata: HashMapStringString,

    /// The minimum number of servers to create.
    #[structable(optional)]
    min_count: Option<u32>,

    /// Name
    #[structable(optional)]
    name: Option<String>,

    /// A networks object. Required parameter when there are multiple networks
    /// defined for the tenant. When you do not specify the networks parameter,
    /// the server attaches to the only network created for the current tenant.
    #[structable(optional)]
    networks: Option<String>,

    /// A list of an attached volumes. Each item in the list contains at least
    /// an "id" key to identify the specific volumes.
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    #[structable(optional)]
    attached_volumes: Option<VecValue>,

    /// While the server is building, this value represents the percentage of
    /// completion. Once it is completed, it will be 100.
    #[structable(optional)]
    progress: Option<u32>,

    /// A list of applicable security groups. Each group contains keys for
    /// description, name, id, and rules.
    #[structable(optional)]
    security_groups: Option<VecValue>,

    /// The UUIDs of the server groups to which the server belongs. Currently
    /// this can contain at most one entry.
    #[structable(optional)]
    server_groups: Option<VecString>,

    /// The state this server is in. Valid values include ``ACTIVE``,
    /// ``BUILDING``, ``DELETED``, ``ERROR``, ``HARD_REBOOT``, ``PASSWORD``,
    /// ``PAUSED``, ``REBOOT``, ``REBUILD``, ``RESCUED``, ``RESIZED``,
    /// ``REVERT_RESIZE``, ``SHUTOFF``, ``SOFT_DELETED``, ``STOPPED``,
    /// ``SUSPENDED``, ``UNKNOWN``, or ``VERIFY_RESIZE``.
    #[structable(optional)]
    status: Option<String>,

    /// Server Tags.
    #[structable(optional)]
    tags: Option<VecString>,

    /// The ID of the project this server is associated with.
    #[serde(rename = "tenant_id")]
    #[structable(optional)]
    project_id: Option<String>,

    /// A list of trusted certificate IDs, that were used during image
    /// signature verification to verify the signing certificate.
    #[structable(optional)]
    trusted_image_certificates: Option<VecString>,

    /// Timestamp of when this server was last updated.
    #[serde(rename = "updated")]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The ID of the owners of this server.
    #[structable(optional)]
    user_id: Option<String>,
}

#[async_trait]
impl Command for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = find::Server::builder();
        // Set path parameters
        ep_builder.id(&self.args.id);
        // Set query parameters
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Compute)
            .await?;
        let data = find(ep).query_async(client).await?;
        op.output_single::<Server>(data)?;
        Ok(())
    }
}
