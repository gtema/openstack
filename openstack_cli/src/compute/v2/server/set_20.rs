// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.

//! Set Server command [microversion = 2.0]
//!
//! Wraps invoking of the `v2.1/servers/{id}` with `PUT` method

use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OpenStackCliError;
use crate::OutputConfig;
use crate::StructTable;

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::find;
use openstack_sdk::api::compute::v2::server::set_20;
use openstack_sdk::api::find;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use structable_derive::StructTable;

/// Updates the editable attributes of an existing server.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401),
/// forbidden(403), itemNotFound(404)
#[derive(Args)]
#[command(about = "Update Server (microversion = 2.0)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    server: Server,
}

/// Query parameters
#[derive(Args)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum OsDcfDiskConfig {
    Auto,
    Manual,
}

/// Server Body data
#[derive(Args)]
struct Server {
    /// The server name.
    #[arg(long)]
    name: Option<String>,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers.
    /// A server inherits the `OS-DCF:diskConfig` value from the image from
    /// which it
    /// was created, and an image inherits the `OS-DCF:diskConfig` value from
    /// the server
    /// from which it was created. To override the inherited setting, you can
    /// include
    /// this attribute in the request body of a server create, rebuild, or
    /// resize request. If
    /// the `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot
    /// create
    /// a server from that image and set its `OS-DCF:diskConfig` value to
    /// `AUTO`.
    /// A valid value is:
    ///
    ///
    /// * `AUTO`. The API builds the server with a single partition the size of
    /// the
    /// target flavor disk. The API automatically adjusts the file system to
    /// fit the
    /// entire partition.
    /// * `MANUAL`. The API builds the server by using whatever partition
    /// scheme and
    /// file system is in the source image. If the target flavor disk is
    /// larger, the API
    /// does not partition the remaining disk space.
    #[arg(long)]
    os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// IPv4 address that should be used to access this server.
    #[arg(long)]
    access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server.
    #[arg(long)]
    access_ipv6: Option<String>,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
pub struct ResponseData {
    /// IPv4 address that should be used to access this server. May be
    /// automatically set by the provider.
    #[serde(rename = "accessIPv4")]
    #[structable(optional, title = "accessIPv4")]
    access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server. May be
    /// automatically set by the provider.
    #[serde(rename = "accessIPv6")]
    #[structable(optional, title = "accessIPv6")]
    access_ipv6: Option<String>,

    /// The addresses for the server. Servers with status `BUILD` hide their
    /// addresses information.
    #[serde()]
    #[structable(optional)]
    addresses: Option<HashMapStringVecResponseAddresses>,

    /// The attached volumes, if any.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    #[structable(optional, title = "os-extended-volumes:volumes_attached")]
    os_extended_volumes_volumes_attached: Option<VecHashMapStringValue>,

    /// The availability zone name.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    #[structable(optional, title = "OS-EXT-AZ:availability_zone")]
    os_ext_az_availability_zone: Option<String>,

    /// The name of the compute host on which this instance is running.
    /// Appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:host")]
    os_ext_srv_attr_host: Option<String>,

    /// Indicates whether or not a config drive was used for this server.
    /// The value is `True` or an empty string. An empty string stands for
    /// `False`.
    ///
    ///
    /// **New in version 2.75**
    #[serde()]
    #[structable(optional)]
    config_drive: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    created: Option<String>,

    /// The description of the server.
    /// Before microversion 2.19 this was set to the server name.
    ///
    ///
    /// **New in version 2.19**
    #[serde()]
    #[structable(optional)]
    description: Option<String>,

    /// Disk configuration. The value is either:
    ///
    ///
    /// * `AUTO`. The API builds the server with a single partition the size of
    /// the target flavor disk. The API automatically adjusts the file system
    /// to
    /// fit the entire partition.
    /// * `MANUAL`. The API builds the server by using the partition scheme and
    /// file system that is in the source image. If the target flavor disk is
    /// larger, The API does not partition the remaining disk space.
    #[serde(rename = "OS-DCF:diskConfig")]
    #[structable(optional, title = "OS-DCF:diskConfig")]
    os_dcf_disk_config: Option<String>,

    /// A fault object. Only displayed when the server status is `ERROR` or
    /// `DELETED` and a fault occurred.
    #[serde()]
    #[structable(optional)]
    fault: Option<ResponseFault>,

    /// Before microversion 2.47 this contains the ID and links for the flavor
    /// used to boot the server instance. This can be an empty object in case
    /// flavor information is no longer present in the system.
    ///
    ///
    /// As of microversion 2.47 this contains a subset of the actual flavor
    /// information used to create the server instance, represented as a nested
    /// dictionary.
    #[serde()]
    #[structable(optional)]
    flavor: Option<ResponseFlavor>,

    /// An ID string representing the host. This is a hashed value so will not
    /// actually look like
    /// a hostname, and is hashed with data from the project\_id, so the same
    /// physical host as seen
    /// by two different project\_ids, will be different. It is useful when
    /// within the same project you
    /// need to determine if two instances are on the same or different
    /// physical hosts for the
    /// purposes of availability or performance.
    #[serde(rename = "hostId")]
    #[structable(optional, title = "hostId")]
    host_id: Option<String>,

    /// The host status. Values where next value in list can override the
    /// previous:
    ///
    ///
    /// * `UP` if nova-compute up.
    /// * `UNKNOWN` if nova-compute not reported by servicegroup driver.
    /// * `DOWN` if nova-compute forced down.
    /// * `MAINTENANCE` if nova-compute is disabled.
    /// * Empty string indicates there is no host for server.
    ///
    ///
    /// This attribute appears in the response only if the policy permits.
    /// By default, only administrators can get this parameter.
    ///
    ///
    /// **New in version 2.75**
    #[serde()]
    #[structable(optional)]
    host_status: Option<String>,

    /// The hostname of the instance reported in the metadata service.
    /// This parameter only appears in responses for administrators until
    /// microversion 2.90, after which it is shown for all users.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// This information is published via the metadata service and requires
    /// application such as `cloud-init` to propogate it through to the
    /// instance.
    ///
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:hostname")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:hostname")]
    os_ext_srv_attr_hostname: Option<String>,

    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic driver,
    /// it is the Ironic node uuid. Appears in the response for administrative
    /// users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    os_ext_srv_attr_hypervisor_hostname: Option<String>,

    /// Id of the server
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// The UUID and links for the image for your server instance. The `image`
    /// object
    /// will be an empty string when you boot the server from a volume.
    #[serde()]
    #[structable(optional)]
    image: Option<ResponseImage>,

    /// The instance name. The Compute API generates the instance name from the
    /// instance
    /// name template. Appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:instance_name")]
    os_ext_srv_attr_instance_name: Option<String>,

    /// True if the instance is locked otherwise False.
    ///
    ///
    /// **New in version 2.9**
    #[serde()]
    #[structable(optional)]
    locked: Option<bool>,

    /// The UUID of the kernel image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:kernel_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:kernel_id")]
    os_ext_srv_attr_kernel_id: Option<String>,

    /// The name of associated key pair, if any.
    ///
    ///
    /// **New in version 2.75**
    #[serde()]
    #[structable(optional)]
    key_name: Option<String>,

    /// When servers are launched via multiple create, this is the
    /// sequence in which the servers were launched.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:launch_index")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:launch_index")]
    os_ext_srv_attr_launch_index: Option<i32>,

    /// The date and time when the server was launched.
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
    /// The `hh±:mm` value, if included, is the time zone as an offset from
    /// UTC.
    /// If the `deleted\_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-SRV-USG:launched_at")]
    #[structable(optional, title = "OS-SRV-USG:launched_at")]
    os_srv_usg_launched_at: Option<String>,

    /// Links to the resources in question. See [API Guide / Links and
    /// References](https://docs.openstack.org/api-
    /// guide/compute/links_and_references.html) for more info.
    #[serde()]
    #[structable(optional)]
    links: Option<Value>,

    /// A dictionary of metadata key-and-value pairs, which is maintained for
    /// backward
    /// compatibility.
    #[serde()]
    #[structable(optional)]
    metadata: Option<HashMapStringString>,

    /// The server name.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The power state of the instance. This is an enum value that is mapped
    /// as:
    ///
    ///
    ///
    /// ```text
    /// 0: NOSTATE
    /// 1: RUNNING
    /// 3: PAUSED
    /// 4: SHUTDOWN
    /// 6: CRASHED
    /// 7: SUSPENDED
    ///
    /// ```
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-STS:power_state")]
    #[structable(optional, title = "OS-EXT-STS:power_state")]
    os_ext_sts_power_state: Option<i32>,

    /// A percentage value of the operation progress.
    /// This parameter only appears when the server status is `ACTIVE`,
    /// `BUILD`, `REBUILD`, `RESIZE`, `VERIFY\_RESIZE` or `MIGRATING`.
    #[serde()]
    #[structable(optional)]
    progress: Option<i32>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The UUID of the ramdisk image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:ramdisk_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:ramdisk_id")]
    os_ext_srv_attr_ramdisk_id: Option<String>,

    /// The reservation id for the server. This is an id that can
    /// be useful in tracking groups of servers created with multiple
    /// create, that will all have the same reservation\_id.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:reservation_id")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:reservation_id")]
    os_ext_srv_attr_reservation_id: Option<String>,

    /// The root device name for the instance
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:root_device_name")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:root_device_name")]
    os_ext_srv_attr_root_device_name: Option<String>,

    /// One or more security groups objects.
    ///
    ///
    /// **New in version 2.75**
    #[serde()]
    #[structable(optional)]
    security_groups: Option<VecResponseSecurityGroups>,

    /// The UUIDs of the server groups to which the server belongs. Currently
    /// this can contain at most one entry.
    ///
    ///
    /// **New in version 2.71**
    #[serde()]
    #[structable(optional)]
    server_groups: Option<VecString>,

    /// The server status.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// A list of tags. The maximum count of tags in this list is 50.
    ///
    ///
    /// **New in version 2.26**
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// The task state of the instance.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-STS:task_state")]
    #[structable(optional, title = "OS-EXT-STS:task_state")]
    os_ext_sts_task_state: Option<String>,

    /// The date and time when the server was deleted.
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
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    /// If the `deleted\_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    #[structable(optional, title = "OS-SRV-USG:terminated_at")]
    os_srv_usg_terminated_at: Option<String>,

    /// A list of trusted certificate IDs, that were used during image
    /// signature
    /// verification to verify the signing certificate. The list is restricted
    /// to a maximum of 50 IDs. The value is `null` if trusted certificate IDs
    /// are not set.
    ///
    ///
    /// **New in version 2.63**
    #[serde()]
    #[structable(optional)]
    trusted_image_certificates: Option<VecString>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm`
    /// value, if included, is the time zone as an offset from UTC. In
    /// the previous example, the offset value is `-05:00`.
    #[serde()]
    #[structable(optional)]
    updated: Option<String>,

    /// The user\_data the instance was created with.
    /// By default, it appears in the response for administrative users only.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-SRV-ATTR:user_data")]
    #[structable(optional, title = "OS-EXT-SRV-ATTR:user_data")]
    os_ext_srv_attr_user_data: Option<String>,

    /// The user ID of the user who owns the server.
    #[serde()]
    #[structable(optional)]
    user_id: Option<String>,

    /// The VM state.
    ///
    ///
    /// **New in version 2.75**
    #[serde(rename = "OS-EXT-STS:vm_state")]
    #[structable(optional, title = "OS-EXT-STS:vm_state")]
    os_ext_sts_vm_state: Option<String>,
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseAddresses {
    addr: Option<String>,
    version: Option<i32>,
}

impl fmt::Display for ResponseAddresses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "addr={}",
                self.addr
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "version={}",
                self.version
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseAddresses response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseAddresses(Vec<ResponseAddresses>);
impl fmt::Display for VecResponseAddresses {
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
/// HashMap of VecResponseAddresses response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct HashMapStringVecResponseAddresses(HashMap<String, VecResponseAddresses>);
impl fmt::Display for HashMapStringVecResponseAddresses {
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
/// HashMap of Value response type
#[derive(Default, Clone, Deserialize, Serialize)]
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
/// Vector of HashMapStringValue response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecHashMapStringValue(Vec<HashMapStringValue>);
impl fmt::Display for VecHashMapStringValue {
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
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseFault {
    code: Option<i32>,
    created: Option<String>,
    message: Option<String>,
    details: Option<String>,
}

impl fmt::Display for ResponseFault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "code={}",
                self.code.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "created={}",
                self.created
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "message={}",
                self.message
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "details={}",
                self.details
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseLinksStructResponseStructResponse {
    href: Option<String>,
    rel: Option<String>,
}

impl fmt::Display for ResponseLinksStructResponseStructResponse {
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
/// HashMap of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
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
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseFlavor {
    id: Option<String>,
    links: Option<Value>,
    vcpus: Option<i32>,
    ram: Option<i32>,
    disk: Option<i32>,
    ephemeral: Option<i32>,
    swap: Option<i32>,
    original_name: Option<String>,
    extra_specs: Option<HashMapStringString>,
}

impl fmt::Display for ResponseFlavor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "links={}",
                self.links
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "vcpus={}",
                self.vcpus.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "ram={}",
                self.ram.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "disk={}",
                self.disk.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "ephemeral={}",
                self.ephemeral
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "swap={}",
                self.swap.map(|v| v.to_string()).unwrap_or("".to_string())
            ),
            format!(
                "original_name={}",
                self.original_name
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "extra_specs={}",
                self.extra_specs
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseImage {
    id: Option<String>,
    links: Option<Value>,
}

impl fmt::Display for ResponseImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "id={}",
                self.id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "links={}",
                self.links
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
/// struct response type
#[derive(Default, Clone, Deserialize, Serialize)]
struct ResponseSecurityGroups {
    name: Option<String>,
}

impl fmt::Display for ResponseSecurityGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([format!(
            "name={}",
            self.name
                .clone()
                .map(|v| v.to_string())
                .unwrap_or("".to_string())
        )]);
        write!(f, "{}", data.join(";"))
    }
}
/// Vector of ResponseSecurityGroups response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecResponseSecurityGroups(Vec<ResponseSecurityGroups>);
impl fmt::Display for VecResponseSecurityGroups {
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
/// Vector of String response type
#[derive(Default, Clone, Deserialize, Serialize)]
pub struct VecString(Vec<String>);
impl fmt::Display for VecString {
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

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.path.id);
        find_builder.header("OpenStack-API-Version", "compute 2.0");
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set_20::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.0");

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.server data
        let args = &self.server;
        let mut server_builder = set_20::ServerBuilder::default();
        if let Some(val) = &args.name {
            server_builder.name(val.clone());
        }

        if let Some(val) = &args.os_dcf_disk_config {
            let tmp = match val {
                OsDcfDiskConfig::Auto => set_20::OsDcfDiskConfig::Auto,
                OsDcfDiskConfig::Manual => set_20::OsDcfDiskConfig::Manual,
            };
            server_builder.os_dcf_disk_config(tmp);
        }

        if let Some(val) = &args.access_ipv4 {
            server_builder.access_ipv4(val.clone());
        }

        if let Some(val) = &args.access_ipv6 {
            server_builder.access_ipv6(val.clone());
        }

        ep_builder.server(server_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
