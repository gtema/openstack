//! List Images
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
use crate::common::VecString;
use crate::common::VecValue;
use openstack_sdk::api::image::v2::images::get;
use openstack_sdk::api::QueryAsync;
use openstack_sdk::api::{paged, Pagination};
use serde_json::Value;

/// List Images
#[derive(Args, Clone, Debug)]
pub struct ImagesArgs {
    /// limit filter parameter
    #[arg(long)]
    limit: Option<String>,

    /// marker filter parameter
    #[arg(long)]
    marker: Option<String>,

    /// id filter parameter
    #[arg(long)]
    id: Option<String>,

    /// name filter parameter
    #[arg(long)]
    name: Option<String>,

    /// visibility filter parameter
    #[arg(long)]
    visibility: Option<String>,

    /// member_status filter parameter
    #[arg(long)]
    member_status: Option<String>,

    /// owner filter parameter
    #[arg(long)]
    owner: Option<String>,

    /// status filter parameter
    #[arg(long)]
    status: Option<String>,

    /// size_min filter parameter
    #[arg(long)]
    size_min: Option<String>,

    /// size_max filter parameter
    #[arg(long)]
    size_max: Option<String>,

    /// protected filter parameter
    #[arg(long)]
    protected: Option<String>,

    /// is_hidden filter parameter
    #[arg(long, action=clap::ArgAction::SetTrue)]
    is_hidden: Option<bool>,

    /// sort_key filter parameter
    #[arg(long)]
    sort_key: Option<String>,

    /// sort_dir filter parameter
    #[arg(long)]
    sort_dir: Option<String>,

    /// sort filter parameter
    #[arg(long)]
    sort: Option<String>,

    /// tag filter parameter
    #[arg(long, action=clap::ArgAction::Append)]
    tag: Option<Vec<String>>,

    /// created_at filter parameter
    #[arg(long)]
    created_at: Option<String>,

    /// updated_at filter parameter
    #[arg(long)]
    updated_at: Option<String>,

    /// Total limit of entities count to return. Use this when there are too many entries.
    #[arg(long, default_value_t = 10000)]
    max_items: usize,
}

pub struct ImagesCmd {
    pub args: ImagesArgs,
}

/// Images
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Images {
    /// The CPU architecture that must be supported by the hypervisor.
    #[structable(optional, wide)]
    architecture: Option<String>,

    /// If true, the root partition on the disk is automatically resized before
    /// the instance boots.
    #[serde(rename = "auto_disk_config")]
    #[structable(optional, wide)]
    has_auto_disk_config: Option<String>,

    /// Hash of the image data used. The Image service uses this value for
    /// verification.
    #[structable(optional, wide)]
    checksum: Option<String>,

    /// The container format refers to whether the VM image is in a file format
    /// that also contains metadata about the actual VM. Container formats
    /// include OVF and Amazon AMI. In addition, a VM image might not have a
    /// container format - instead, the image is just a blob of unstructured
    /// data.
    #[structable(optional, wide)]
    container_format: Option<String>,

    /// The date and time when the image was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The URL to access the image file kept in external store. It appears
    /// when you set the show_image_direct_url option to true in the Image
    /// service's configuration file.
    #[structable(optional, wide)]
    direct_url: Option<String>,

    /// Valid values are: aki, ari, ami, raw, iso, vhd, vdi, qcow2, or vmdk.
    /// The disk format of a VM image is the format of the underlying disk
    /// image. Virtual appliance vendors have different formats for laying out
    /// the information contained in a VM disk image.
    #[structable(optional, wide)]
    disk_format: Option<String>,

    /// The URL for the virtual machine image file.
    #[structable(optional, wide)]
    file: Option<String>,

    //    /// If true, enables the BIOS bootmenu.
    //    #[serde(rename = "hw_boot_menu")]
    //    #[structable(optional, wide)]
    //    is_hw_boot_menu_enabled: Option<bool>,
    /// The preferred number of cores to expose to the guest.
    #[structable(optional, wide)]
    hw_cpu_cores: Option<u32>,

    /// Used to pin the virtual CPUs (vCPUs) of instances to the host's
    /// physical CPU cores (pCPUs).
    #[structable(optional, wide)]
    hw_cpu_policy: Option<String>,

    /// The preferred number of sockets to expose to the guest.
    #[structable(optional, wide)]
    hw_cpu_sockets: Option<String>,

    /// Defines how hardware CPU threads in a simultaneous multithreading-based
    /// (SMT) architecture be used.
    #[structable(optional, wide)]
    hw_cpu_thread_policy: Option<String>,

    /// The preferred number of threads to expose to the guest.
    #[structable(optional, wide)]
    hw_cpu_threads: Option<u32>,

    /// Specifies the type of disk controller to attach disk devices to. One of
    /// scsi, virtio, uml, xen, ide, or usb.
    #[structable(optional, wide)]
    hw_disk_bus: Option<String>,

    /// For libvirt: Enables booting an ARM system using the specified machine
    /// type. For Hyper-V: Specifies whether the Hyper-V instance will be a
    /// generation 1 or generation 2 VM.
    #[structable(optional, wide)]
    hw_machine_type: Option<String>,

    /// A string boolean, which if "true", QEMU guest agent will be exposed to
    /// the instance.
    #[structable(optional, wide)]
    hw_qemu_guest_agent: Option<String>,

    /// Adds a random-number generator device to the image's instances.
    #[structable(optional, wide)]
    hw_rng_model: Option<String>,

    /// Enables the use of VirtIO SCSI (virtio-scsi) to provide block device
    /// access for compute instances; by default, instances use VirtIO Block
    /// (virtio-blk).
    #[structable(optional, wide)]
    hw_scsi_model: Option<String>,

    /// Specifies the count of serial ports that should be provided.
    #[structable(optional, wide)]
    hw_serial_port_count: Option<u32>,

    /// The video image driver used.
    #[structable(optional, wide)]
    hw_video_model: Option<String>,

    /// Maximum RAM for the video image.
    #[structable(optional, wide)]
    hw_video_ram: Option<u32>,

    /// Specifies the model of virtual network interface device to use.
    #[structable(optional, wide)]
    hw_vif_model: Option<String>,

    //    /// If true, this enables the virtio-net multiqueue feature. In this case,
    //    /// the driver sets the number of queues equal to the number of guest
    //    /// vCPUs. This makes the network performance scale across a number of
    //    /// vCPUs.
    //    #[serde(rename = "hw_vif_multiqueue_enabled")]
    //    #[structable(optional, wide)]
    //    is_hw_vif_multiqueue_enabled: Option<bool>,
    /// Enables a virtual hardware watchdog device that carries out the
    /// specified action if the server hangs.
    #[structable(optional, wide)]
    hw_watchdog_action: Option<String>,

    /// The hypervisor type. Note that qemu is used for both QEMU and KVM
    /// hypervisor types.
    #[structable(optional, wide)]
    hypervisor_type: Option<String>,

    /// Id of the resource
    id: String,

    /// Specifies whether the image needs a config drive. `mandatory` or
    /// `optional` (default if property is not used).
    #[serde(rename = "img_config_drive")]
    #[structable(optional, wide)]
    needs_config_drive: Option<String>,

    /// Optional property allows created servers to have a different bandwidth
    /// cap than that defined in the network they are attached to.
    #[structable(optional, wide)]
    instance_type_rxtx_factor: Option<f32>,

    /// create this image.
    #[structable(optional, wide)]
    instance_uuid: Option<String>,

    /// The ID of an image stored in the Image service that should be used as
    /// the kernel when booting an AMI-style image.
    #[structable(optional, wide)]
    kernel_id: Option<String>,

    /// A list of URLs to access the image file in external store. This list
    /// appears if the show_multiple_locations option is set to true in the
    /// Image service's configuration file.
    #[structable(optional, wide)]
    locations: Option<VecValue>,

    /// The minimum disk size in GB that is required to boot the image.
    #[structable(optional, wide)]
    min_disk: Option<u32>,

    /// The minimum amount of RAM in MB that is required to boot the image.
    #[structable(optional, wide)]
    min_ram: Option<u32>,

    /// The name of the image.
    #[structable(optional)]
    name: Option<String>,

    /// The operating system admin username.
    #[structable(optional, wide)]
    os_admin_user: Option<u32>,

    /// The kernel command line to be used by the libvirt driver, instead of
    /// the default.
    #[structable(optional, wide)]
    os_command_line: Option<String>,

    /// The common name of the operating system distribution in lowercase
    #[structable(optional, wide)]
    os_distro: Option<String>,

    /// The algorithm used to compute a secure hash of the image data for this
    /// image
    #[serde(rename = "os_hash_algo")]
    #[structable(optional, wide)]
    hash_algo: Option<String>,

    /// The hexdigest of the secure hash of the image data computed using the
    /// algorithm whose name is the value of the os_hash_algo property.
    #[serde(rename = "os_hash_value")]
    #[structable(optional, wide)]
    hash_value: Option<String>,

    /// This field controls whether an image is displayed in the default image-
    /// list response
    #[serde(rename = "os_hidden")]
    #[structable(optional, wide)]
    is_hidden: Option<bool>,

    /// If true, require quiesce on snapshot via QEMU guest agent.
    #[structable(optional, wide)]
    os_require_quiesce: Option<bool>,

    /// Secure Boot is a security standard. When the instance starts, Secure
    /// Boot first examines software such as firmware and OS by their signature
    /// and only allows them to run if the signatures are valid.
    #[serde(rename = "os_secure_boot")]
    #[structable(optional, wide)]
    needs_secure_boot: Option<String>,

    /// Time for graceful shutdown
    #[structable(optional, wide)]
    os_shutdown_timeout: Option<u32>,

    /// The operating system installed on the image.
    #[structable(optional, wide)]
    os_type: Option<String>,

    /// The operating system version as specified by the distributor.
    #[structable(optional, wide)]
    os_version: Option<String>,

    /// The ID of the owner, or project, of the image. (backwards compat)
    #[serde(rename = "owner")]
    #[structable(optional, wide)]
    owner_id: Option<String>,

    /// Defines whether the image can be deleted.
    #[serde(rename = "protected")]
    #[structable(optional, wide)]
    is_protected: Option<bool>,

    /// The ID of image stored in the Image service that should be used as the
    /// ramdisk when booting an AMI-style image.
    #[structable(optional, wide)]
    ramdisk_id: Option<String>,

    /// The URL for the schema describing a virtual machine image.
    #[structable(optional, wide)]
    schema: Option<String>,

    /// The size of the image data, in bytes.
    #[structable(optional, wide)]
    size: Option<u64>,

    /// The image status.
    #[structable(optional, wide)]
    status: Option<String>,

    /// When present, Glance will attempt to store the disk image data in the
    /// backing store indicated by the value of the header. When not present,
    /// Glance will store the disk image data in the backing store that is
    /// marked default. Valid values are: file, s3, rbd, swift, cinder, gridfs,
    /// sheepdog, or vsphere.
    #[structable(optional, wide)]
    store: Option<String>,

    /// List of tags for this image, possibly an empty list.
    #[structable(optional, wide)]
    tags: Option<VecString>,

    /// The date and time when the image was updated.
    #[structable(optional)]
    updated_at: Option<String>,

    /// The URL to access the image file kept in external store.
    #[structable(optional, wide)]
    url: Option<String>,

    /// The virtual size of the image.
    #[structable(optional, wide)]
    virtual_size: Option<u64>,

    /// The image visibility.
    #[structable(optional, wide)]
    visibility: Option<String>,

    /// The virtual machine mode. This represents the host/guest ABI
    /// (application binary interface) used for the virtual machine.
    #[structable(optional, wide)]
    vm_mode: Option<String>,

    /// The virtual SCSI or IDE controller used by the hypervisor.
    #[structable(optional, wide)]
    vmware_adaptertype: Option<String>,

    /// A VMware GuestID which describes the operating system installed in the
    /// image.
    #[structable(optional, wide)]
    vmware_ostype: Option<String>,
}

#[async_trait]
impl Command for ImagesCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get Images with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = get::Images::builder();
        // Set path parameters
        // Set query parameters
        if let Some(val) = &self.args.limit {
            ep_builder.limit(val);
        }
        if let Some(val) = &self.args.marker {
            ep_builder.marker(val);
        }
        if let Some(val) = &self.args.id {
            ep_builder.id(val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.visibility {
            ep_builder.visibility(val);
        }
        if let Some(val) = &self.args.member_status {
            ep_builder.member_status(val);
        }
        if let Some(val) = &self.args.owner {
            ep_builder.owner(val);
        }
        if let Some(val) = &self.args.status {
            ep_builder.status(val);
        }
        if let Some(val) = &self.args.size_min {
            ep_builder.size_min(val);
        }
        if let Some(val) = &self.args.size_max {
            ep_builder.size_max(val);
        }
        if let Some(val) = &self.args.protected {
            ep_builder.protected(val);
        }
        //        if let Some(val) = &self.args.is_hidden {
        //            ep_builder.is_hidden(*val);
        //        }
        if let Some(val) = &self.args.sort_key {
            ep_builder.sort_key(val);
        }
        if let Some(val) = &self.args.sort_dir {
            ep_builder.sort_dir(val);
        }
        if let Some(val) = &self.args.sort {
            ep_builder.sort(val);
        }
        if let Some(val) = &self.args.tag {
            ep_builder.tag(val.iter());
        }
        if let Some(val) = &self.args.created_at {
            ep_builder.created_at(val);
        }
        if let Some(val) = &self.args.updated_at {
            ep_builder.updated_at(val);
        }
        // Set body parameters
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Image)
            .await?;
        let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(self.args.max_items))
            .query_async(client)
            .await?;

        op.output_list::<Images>(data)?;
        Ok(())
    }
}
