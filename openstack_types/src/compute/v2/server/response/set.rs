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
//! Response type for the put servers/{id} operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Server response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServerResponse {
    /// IPv4 address that should be used to access this server. May be
    /// automatically set by the provider.
    ///
    #[serde(rename = "accessIPv4")]
    pub access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server. May be
    /// automatically set by the provider.
    ///
    #[serde(rename = "accessIPv6")]
    pub access_ipv6: Option<String>,

    /// The addresses for the server. Servers with status `BUILD` hide their
    /// addresses information. This view is not updated immediately. Please
    /// consult with OpenStack Networking API for up-to-date information.
    ///
    pub addresses: Option<HashMap<String, Vec<Addresses>>>,

    /// Indicates whether or not a config drive was used for this server. The
    /// value is `True` or an empty string. An empty string stands for `False`.
    ///
    /// **New in version 2.75**
    ///
    pub config_drive: Option<String>,

    /// The date and time when the resource was created. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    pub created: Option<String>,

    /// The description of the server. Before microversion 2.19 this was set to
    /// the server name.
    ///
    /// **New in version 2.19**
    ///
    pub description: Option<String>,

    /// A fault object. Only displayed when the server status is `ERROR` or
    /// `DELETED` and a fault occurred.
    ///
    pub fault: Option<Fault>,

    /// Before microversion 2.47 this contains the ID and links for the flavor
    /// used to boot the server instance. This can be an empty object in case
    /// flavor information is no longer present in the system.
    ///
    /// As of microversion 2.47 this contains a subset of the actual flavor
    /// information used to create the server instance, represented as a nested
    /// dictionary.
    ///
    pub flavor: Flavor,

    /// The host status. Values where next value in list can override the
    /// previous:
    ///
    /// - `UP` if nova-compute up.
    /// - `UNKNOWN` if nova-compute not reported by servicegroup driver.
    /// - `DOWN` if nova-compute forced down.
    /// - `MAINTENANCE` if nova-compute is disabled.
    /// - Empty string indicates there is no host for server.
    ///
    /// This attribute appears in the response only if the policy permits. By
    /// default, only administrators can get this parameter.
    ///
    /// **New in version 2.75**
    ///
    pub host_status: Option<HostStatus>,

    /// An ID string representing the host. This is a hashed value so will not
    /// actually look like a hostname, and is hashed with data from the
    /// project_id, so the same physical host as seen by two different
    /// project_ids, will be different. It is useful when within the same
    /// project you need to determine if two instances are on the same or
    /// different physical hosts for the purposes of availability or
    /// performance.
    ///
    #[serde(rename = "hostId")]
    pub host_id: Option<String>,

    /// Id of the server
    ///
    pub id: String,

    /// The UUID and links for the image for your server instance. The `image`
    /// object will be an empty string when you boot the server from a volume.
    ///
    pub image: Image,

    /// The name of associated key pair, if any.
    ///
    /// **New in version 2.75**
    ///
    pub key_name: Option<String>,

    /// Links to the resources in question. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    pub links: Option<Vec<Links>>,

    /// True if the instance is locked otherwise False.
    ///
    /// **New in version 2.9**
    ///
    pub locked: Option<bool>,

    /// A dictionary of metadata key-and-value pairs, which is maintained for
    /// backward compatibility.
    ///
    pub metadata: Option<HashMap<String, String>>,

    /// The server name.
    ///
    pub name: String,

    /// Disk configuration. The value is either:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using the partition scheme and
    ///   file system that is in the source image. If the target flavor disk is
    ///   larger, The API does not partition the remaining disk space.
    ///
    #[serde(rename = "OS-DCF:diskConfig")]
    pub os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// The availability zone name.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-AZ:availability_zone")]
    pub os_ext_az_availability_zone: Option<String>,

    /// The name of the compute host on which this instance is running. Appears
    /// in the response for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:host")]
    pub os_ext_srv_attr_host: Option<String>,

    /// The hostname of the instance reported in the metadata service. This
    /// parameter only appears in responses for administrators until
    /// microversion 2.90, after which it is shown for all users.
    ///
    /// Note
    ///
    /// This information is published via the metadata service and requires
    /// application such as `cloud-init` to propagate it through to the
    /// instance.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:hostname")]
    pub os_ext_srv_attr_hostname: Option<String>,

    /// The hypervisor host name provided by the Nova virt driver. For the
    /// Ironic driver, it is the Ironic node uuid. Appears in the response for
    /// administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:hypervisor_hostname")]
    pub os_ext_srv_attr_hypervisor_hostname: Option<String>,

    /// The instance name. The Compute API generates the instance name from the
    /// instance name template. Appears in the response for administrative
    /// users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:instance_name")]
    pub os_ext_srv_attr_instance_name: Option<String>,

    /// The UUID of the kernel image when using an AMI. Will be null if not. By
    /// default, it appears in the response for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:kernel_id")]
    pub os_ext_srv_attr_kernel_id: Option<String>,

    /// When servers are launched via multiple create, this is the sequence in
    /// which the servers were launched. By default, it appears in the response
    /// for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:launch_index")]
    pub os_ext_srv_attr_launch_index: Option<i32>,

    /// The UUID of the ramdisk image when using an AMI. Will be null if not.
    /// By default, it appears in the response for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:ramdisk_id")]
    pub os_ext_srv_attr_ramdisk_id: Option<String>,

    /// The reservation id for the server. This is an id that can be useful in
    /// tracking groups of servers created with multiple create, that will all
    /// have the same reservation_id. By default, it appears in the response
    /// for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:reservation_id")]
    pub os_ext_srv_attr_reservation_id: Option<String>,

    /// The root device name for the instance By default, it appears in the
    /// response for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:root_device_name")]
    pub os_ext_srv_attr_root_device_name: Option<String>,

    /// The user_data the instance was created with. By default, it appears in
    /// the response for administrative users only.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-SRV-ATTR:user_data")]
    pub os_ext_srv_attr_user_data: Option<String>,

    /// The power state of the instance. This is an enum value that is mapped
    /// as:
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
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-STS:power_state")]
    pub os_ext_sts_power_state: Option<i32>,

    /// The task state of the instance.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-STS:task_state")]
    pub os_ext_sts_task_state: Option<String>,

    /// The VM state.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-EXT-STS:vm_state")]
    pub os_ext_sts_vm_state: Option<String>,

    /// The attached volumes, if any.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "os-extended-volumes:volumes_attached")]
    pub os_extended_volumes_volumes_attached: Option<Vec<HashMap<String, Value>>>,

    /// The date and time when the server was launched.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `hh±:mm` value, if included, is the time zone as an offset from
    /// UTC. If the `deleted_at` date and time stamp is not set, its value is
    /// `null`.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-SRV-USG:launched_at")]
    pub os_srv_usg_launched_at: Option<String>,

    /// The date and time when the server was deleted.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. If the `deleted_at`
    /// date and time stamp is not set, its value is `null`.
    ///
    /// **New in version 2.75**
    ///
    #[serde(rename = "OS-SRV-USG:terminated_at")]
    pub os_srv_usg_terminated_at: Option<String>,

    /// A percentage value of the operation progress. This parameter only
    /// appears when the server status is `ACTIVE`, `BUILD`, `REBUILD`,
    /// `RESIZE`, `VERIFY_RESIZE` or `MIGRATING`.
    ///
    pub progress: Option<i32>,

    /// One or more security groups objects.
    ///
    /// **New in version 2.75**
    ///
    pub security_groups: Option<Vec<SecurityGroups>>,

    /// The UUIDs of the server groups to which the server belongs. Currently
    /// this can contain at most one entry.
    ///
    /// **New in version 2.71**
    ///
    pub server_groups: Option<Vec<String>>,

    /// The server status.
    ///
    pub status: Option<String>,

    /// A list of tags. The maximum count of tags in this list is 50.
    ///
    /// **New in version 2.26**
    ///
    pub tags: Option<Vec<String>>,

    /// The UUID of the tenant in a multi-tenancy cloud.
    ///
    pub tenant_id: Option<String>,

    /// A list of trusted certificate IDs, that were used during image
    /// signature verification to verify the signing certificate. The list is
    /// restricted to a maximum of 50 IDs. The value is `null` if trusted
    /// certificate IDs are not set.
    ///
    /// **New in version 2.63**
    ///
    pub trusted_image_certificates: Option<Vec<String>>,

    /// The date and time when the resource was updated. The date and time
    /// stamp format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    pub updated: Option<String>,

    /// The user ID of the user who owns the server.
    ///
    pub user_id: Option<String>,
}

/// `Addresses` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Addresses {
    pub addr: Option<String>,
    pub version: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum OsDcfDiskConfig {
    // Auto
    #[serde(rename = "AUTO")]
    Auto,

    // Manual
    #[serde(rename = "MANUAL")]
    Manual,
}

/// A fault object. Only displayed when the server status is `ERROR` or
/// `DELETED` and a fault occurred.
///
/// `Fault` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Fault {
    pub code: Option<i32>,
    pub created: Option<String>,
    pub details: Option<String>,
    pub message: Option<String>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub href: Option<String>,
    pub rel: Option<String>,
}

/// Before microversion 2.47 this contains the ID and links for the flavor used
/// to boot the server instance. This can be an empty object in case flavor
/// information is no longer present in the system.
///
/// As of microversion 2.47 this contains a subset of the actual flavor
/// information used to create the server instance, represented as a nested
/// dictionary.
///
/// `Flavor` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Flavor {
    pub disk: Option<i32>,
    pub ephemeral: Option<i32>,
    pub extra_specs: Option<HashMap<String, String>>,
    pub id: Option<String>,
    pub links: Option<Vec<Links>>,
    pub original_name: Option<String>,
    pub ram: Option<i32>,
    pub swap: Option<i32>,
    pub vcpus: Option<i32>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum HostStatus {
    // Empty
    #[serde(rename = "")]
    Empty,

    // Down
    #[serde(rename = "DOWN")]
    Down,

    // Maintenance
    #[serde(rename = "MAINTENANCE")]
    Maintenance,

    // Null
    #[serde(rename = "null")]
    Null,

    // Unknown
    #[serde(rename = "UNKNOWN")]
    Unknown,

    // Up
    #[serde(rename = "UP")]
    Up,
}

/// The UUID and links for the image for your server instance. The `image`
/// object will be an empty string when you boot the server from a volume.
///
/// `Image` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub id: Option<String>,
    pub links: Option<Vec<Links>>,
}

/// `SecurityGroups` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SecurityGroups {
    pub name: Option<String>,
}
