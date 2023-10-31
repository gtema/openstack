//! Creates a catalog record for an operating system disk image. (Since Image
//! API v2.0)
//! The Location response header contains the URI for the image.
//! A multiple store backend support is introduced in the Rocky release as a
//! part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header
//! OpenStack-image-store-ids which contains the list of available stores will
//! be included in response. This header is only included if multiple backend
//! stores are supported.
//! The response body contains the new image entity.
//! Synchronous Postconditions
//! With correct permissions, you can see the image status as queued through
//! API calls.
use async_trait::async_trait;
use clap::Args;
use http::Response;
use serde::{Deserialize, Serialize};
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
use openstack_sdk::api::image::v2::images::post;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Creates a catalog record for an operating system disk image. (Since Image
/// API v2.0)
/// The Location response header contains the URI for the image.
/// A multiple store backend support is introduced in the Rocky release as a
/// part of the EXPERIMENTAL Image API v2.8. Since Image API v2.8 a new header
/// OpenStack-image-store-ids which contains the list of available stores will
/// be included in response. This header is only included if multiple backend
/// stores are supported.
/// The response body contains the new image entity.
/// Synchronous Postconditions
/// With correct permissions, you can see the image status as queued through
/// API calls.
#[derive(Args, Clone, Debug)]
pub struct ImageArgs {
    /// The container format refers to whether the VM image is in a file format
    /// that also contains metadata about the actual VM. Container formats
    /// include OVF and Amazon AMI. In addition, a VM image might not have a
    /// container format - instead, the image is just a blob of unstructured
    /// data.
    #[arg(long)]
    container_format: Option<String>,

    /// The format of the disk.
    /// Values may vary based on the configuration available in a particular
    /// OpenStack cloud. See the Image Schema response from the cloud itself
    /// for the valid values available.
    /// Example formats are: ami, ari, aki, vhd, vhdx, vmdk, raw, qcow2,
    ///   vdi, ploop or iso.
    ///
    /// The value might be null (JSON null data type).
    /// Newton changes: The vhdx disk format is a supported value. Ocata
    /// changes: The ploop disk format is a supported value.
    #[arg(long)]
    disk_format: Option<String>,

    /// Amount of disk space in GB that is required to boot the image.
    #[arg(long)]
    min_disk: Option<u32>,

    /// Amount of RAM in MB that is required to boot the image.
    #[arg(long)]
    min_ram: Option<u32>,

    /// The name of the image.
    #[arg(long)]
    name: Option<String>,

    /// Image protection for deletion. Valid value is true or false. Default is
    /// false.
    #[arg(long)]
    is_protected: Option<bool>,

    /// List of tags for this image. Each tag is a string of at most 255 chars.
    /// The maximum number of tags allowed on an image is set by the operator.
    #[arg(long, action=clap::ArgAction::Append)]
    tags: Option<Vec<String>>,

    /// Visibility for this image. Valid value is one of: ``public``,
    /// ``private``, ``shared``, or ``community``. At most sites, only an
    /// administrator can make an image public. Some sites may restrict what
    /// users can make an image community. Some sites may restrict what users
    /// can perform member operations on a shared image. Since the Image API
    /// v2.5, the default value is ``shared``.
    #[arg(long)]
    visibility: Option<String>,
}

pub struct ImageCmd {
    pub args: ImageArgs,
}

/// Image
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Image {
    /// The CPU architecture that must be supported by the hypervisor.
    #[structable(optional)]
    architecture: Option<String>,

    /// If true, the root partition on the disk is automatically resized before
    /// the instance boots.
    #[serde(rename = "auto_disk_config")]
    #[structable(optional)]
    has_auto_disk_config: Option<String>,

    /// Hash of the image data used. The Image service uses this value for
    /// verification.
    #[structable(optional)]
    checksum: Option<String>,

    /// The container format refers to whether the VM image is in a file format
    /// that also contains metadata about the actual VM. Container formats
    /// include OVF and Amazon AMI. In addition, a VM image might not have a
    /// container format - instead, the image is just a blob of unstructured
    /// data.
    #[structable(optional)]
    container_format: Option<String>,

    /// The date and time when the image was created.
    #[structable(optional)]
    created_at: Option<String>,

    /// The URL to access the image file kept in external store. It appears
    /// when you set the show_image_direct_url option to true in the Image
    /// service's configuration file.
    #[structable(optional)]
    direct_url: Option<String>,

    /// Valid values are: aki, ari, ami, raw, iso, vhd, vdi, qcow2, or vmdk.
    /// The disk format of a VM image is the format of the underlying disk
    /// image. Virtual appliance vendors have different formats for laying out
    /// the information contained in a VM disk image.
    #[structable(optional)]
    disk_format: Option<String>,

    /// The URL for the virtual machine image file.
    #[structable(optional)]
    file: Option<String>,

    /// The preferred number of cores to expose to the guest.
    #[structable(optional)]
    hw_cpu_cores: Option<u32>,

    /// Used to pin the virtual CPUs (vCPUs) of instances to the host's
    /// physical CPU cores (pCPUs).
    #[structable(optional)]
    hw_cpu_policy: Option<String>,

    /// Defines how hardware CPU threads in a simultaneous multithreading-based
    /// (SMT) architecture be used.
    #[structable(optional)]
    hw_cpu_thread_policy: Option<String>,

    /// The preferred number of threads to expose to the guest.
    #[structable(optional)]
    hw_cpu_threads: Option<u32>,

    /// Specifies the type of disk controller to attach disk devices to. One of
    /// scsi, virtio, uml, xen, ide, or usb.
    #[structable(optional)]
    hw_disk_bus: Option<String>,

    /// For libvirt: Enables booting an ARM system using the specified machine
    /// type. For Hyper-V: Specifies whether the Hyper-V instance will be a
    /// generation 1 or generation 2 VM.
    #[structable(optional)]
    hw_machine_type: Option<String>,

    /// A string boolean, which if "true", QEMU guest agent will be exposed to
    /// the instance.
    #[structable(optional)]
    hw_qemu_guest_agent: Option<String>,

    /// Adds a random-number generator device to the image's instances.
    #[structable(optional)]
    hw_rng_model: Option<String>,

    /// Enables the use of VirtIO SCSI (virtio-scsi) to provide block device
    /// access for compute instances; by default, instances use VirtIO Block
    /// (virtio-blk).
    #[structable(optional)]
    hw_scsi_model: Option<String>,

    /// Specifies the count of serial ports that should be provided.
    #[structable(optional)]
    hw_serial_port_count: Option<u32>,

    /// The video image driver used.
    #[structable(optional)]
    hw_video_model: Option<String>,

    /// Maximum RAM for the video image.
    #[structable(optional)]
    hw_video_ram: Option<u32>,

    /// Specifies the model of virtual network interface device to use.
    #[structable(optional)]
    hw_vif_model: Option<String>,

    /// Enables a virtual hardware watchdog device that carries out the
    /// specified action if the server hangs.
    #[structable(optional)]
    hw_watchdog_action: Option<String>,

    /// The hypervisor type. Note that qemu is used for both QEMU and KVM
    /// hypervisor types.
    #[structable(optional)]
    hypervisor_type: Option<String>,

    /// Id of the resource
    id: String,

    /// Specifies whether the image needs a config drive. `mandatory` or
    /// `optional` (default if property is not used).
    #[serde(rename = "img_config_drive")]
    #[structable(optional)]
    needs_config_drive: Option<String>,

    /// Optional property allows created servers to have a different bandwidth
    /// cap than that defined in the network they are attached to.
    #[structable(optional)]
    instance_type_rxtx_factor: Option<f32>,

    /// create this image.
    #[structable(optional)]
    instance_uuid: Option<String>,

    /// The ID of an image stored in the Image service that should be used as
    /// the kernel when booting an AMI-style image.
    #[structable(optional)]
    kernel_id: Option<String>,

    /// A list of URLs to access the image file in external store. This list
    /// appears if the show_multiple_locations option is set to true in the
    /// Image service's configuration file.
    #[structable(optional)]
    locations: Option<VecValue>,

    /// The minimum disk size in GB that is required to boot the image.
    #[structable(optional)]
    min_disk: Option<u32>,

    /// The minimum amount of RAM in MB that is required to boot the image.
    #[structable(optional)]
    min_ram: Option<u32>,

    /// The name of the image.
    #[structable(optional)]
    name: Option<String>,

    /// The operating system admin username.
    #[structable(optional)]
    os_admin_user: Option<u32>,

    /// The kernel command line to be used by the libvirt driver, instead of
    /// the default.
    #[structable(optional)]
    os_command_line: Option<String>,

    /// The common name of the operating system distribution in lowercase
    #[structable(optional)]
    os_distro: Option<String>,

    /// The algorithm used to compute a secure hash of the image data for this
    /// image
    #[serde(rename = "os_hash_algo")]
    #[structable(optional)]
    hash_algo: Option<String>,

    /// The hexdigest of the secure hash of the image data computed using the
    /// algorithm whose name is the value of the os_hash_algo property.
    #[serde(rename = "os_hash_value")]
    #[structable(optional)]
    hash_value: Option<String>,

    /// This field controls whether an image is displayed in the default image-
    /// list response
    #[serde(rename = "os_hidden")]
    #[structable(optional)]
    is_hidden: Option<bool>,

    /// If true, require quiesce on snapshot via QEMU guest agent.
    #[structable(optional)]
    os_require_quiesce: Option<bool>,

    /// Secure Boot is a security standard. When the instance starts, Secure
    /// Boot first examines software such as firmware and OS by their signature
    /// and only allows them to run if the signatures are valid.
    #[serde(rename = "os_secure_boot")]
    #[structable(optional)]
    needs_secure_boot: Option<String>,

    /// Time for graceful shutdown
    #[structable(optional)]
    os_shutdown_timeout: Option<u32>,

    /// The operating system installed on the image.
    #[structable(optional)]
    os_type: Option<String>,

    /// The operating system version as specified by the distributor.
    #[structable(optional)]
    os_version: Option<String>,

    /// The ID of the owner, or project, of the image. (backwards compat)
    #[serde(rename = "owner")]
    #[structable(optional)]
    owner_id: Option<String>,

    /// Defines whether the image can be deleted.
    #[serde(rename = "protected")]
    #[structable(optional)]
    is_protected: Option<bool>,

    /// The ID of image stored in the Image service that should be used as the
    /// ramdisk when booting an AMI-style image.
    #[structable(optional)]
    ramdisk_id: Option<String>,

    /// The URL for the schema describing a virtual machine image.
    #[structable(optional)]
    schema: Option<String>,

    /// The size of the image data, in bytes.
    #[structable(optional)]
    size: Option<u64>,

    /// The image status.
    #[structable(optional)]
    status: Option<String>,

    /// When present, Glance will attempt to store the disk image data in the
    /// backing store indicated by the value of the header. When not present,
    /// Glance will store the disk image data in the backing store that is
    /// marked default. Valid values are: file, s3, rbd, swift, cinder, gridfs,
    /// sheepdog, or vsphere.
    #[structable(optional)]
    store: Option<String>,

    /// List of tags for this image, possibly an empty list.
    #[structable(optional)]
    tags: Option<VecString>,

    /// The date and time when the image was updated.
    #[structable(optional)]
    updated_at: Option<String>,

    /// The URL to access the image file kept in external store.
    #[structable(optional)]
    url: Option<String>,

    /// The virtual size of the image.
    #[structable(optional)]
    virtual_size: Option<u64>,

    /// The image visibility.
    #[structable(optional)]
    visibility: Option<String>,

    /// The virtual machine mode. This represents the host/guest ABI
    /// (application binary interface) used for the virtual machine.
    #[structable(optional)]
    vm_mode: Option<String>,

    /// The virtual SCSI or IDE controller used by the hypervisor.
    #[structable(optional)]
    vmware_adaptertype: Option<String>,

    /// A VMware GuestID which describes the operating system installed in the
    /// image.
    #[structable(optional)]
    vmware_ostype: Option<String>,
}

#[async_trait]
impl Command for ImageCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Post Image with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        let mut ep_builder = post::Image::builder();
        // Set path parameters
        // Set query parameters
        // Set body parameters
        if let Some(val) = &self.args.container_format {
            ep_builder.container_format(val);
        }
        if let Some(val) = &self.args.disk_format {
            ep_builder.disk_format(val);
        }
        if let Some(val) = &self.args.min_disk {
            ep_builder.min_disk(*val);
        }
        if let Some(val) = &self.args.min_ram {
            ep_builder.min_ram(*val);
        }
        if let Some(val) = &self.args.name {
            ep_builder.name(val);
        }
        if let Some(val) = &self.args.is_protected {
            ep_builder.is_protected(*val);
        }
        if let Some(val) = &self.args.tags {
            ep_builder.tags(val.iter().cloned());
        }
        if let Some(val) = &self.args.visibility {
            ep_builder.visibility(val);
        }
        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        client
            .discover_service_endpoint(&ServiceType::Image)
            .await?;
        let data = ep.query_async(client).await?;
        op.output_single::<Image>(data)?;
        Ok(())
    }
}
