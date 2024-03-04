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

//! Create Server command [microversion = 2.74]
//!
//! Wraps invoking of the `v2.1/servers` with `POST` method

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

use crate::common::parse_json;
use crate::common::parse_key_val;
use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::create_274;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;
use structable_derive::StructTable;

/// Creates a server.
///
/// The progress of this operation depends on the location of the requested
/// image, network I/O, host load, selected flavor, and other factors.
///
/// To check the progress of the request, make a `GET /servers/{id}` request.
/// This call returns a progress attribute, which is a percentage value from 0
/// to 100.
///
/// The `Location` header returns the full URL to the newly created server and
/// is available as a `self` and `bookmark` link in the server representation.
///
/// When you create a server, the response shows only the server ID, its links,
/// and the admin password. You can get additional attributes through
/// subsequent `GET` requests on the server.
///
/// Include the `block_device_mapping_v2` parameter in the create request body
/// to boot a server from a volume.
///
/// Include the `key_name` parameter in the create request body to add a
/// keypair to the server when you create it. To create a keypair, make a
/// [create keypair](https://docs.openstack.org/api-ref/compute/#create-or-import-keypair)
/// request.
///
/// **Preconditions**
///
/// **Asynchronous postconditions**
///
/// **Troubleshooting**
///
/// Normal response codes: 202
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409)
///
#[derive(Args)]
#[command(about = "Create Server (microversion = 2.74)")]
pub struct ServerCommand {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    server: Server,
    #[command(flatten)]
    os_scheduler_hints: Option<OsSchedulerHints>,
}

/// Query parameters
#[derive(Args)]
struct QueryParameters {}

/// Path parameters
#[derive(Args)]
struct PathParameters {}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum NetworksStringEnum {
    Auto,
    None,
}

/// NetworksEnumGroupStruct Body data
#[derive(Args)]
#[group(required = true, multiple = false)]
struct NetworksEnumGroupStruct {
    #[arg(action=clap::ArgAction::SetTrue, long, required=false)]
    auto_networks: bool,

    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    networks: Option<Vec<Value>>,

    #[arg(action=clap::ArgAction::SetTrue, long, required=false)]
    none_networks: bool,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum OsDcfDiskConfig {
    Auto,
    Manual,
}

/// Server Body data
#[derive(Args)]
struct Server {
    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[arg(long)]
    name: String,

    /// The UUID of the image to use for your server instance. This is not
    /// required in case of boot from volume. In all other cases it is required
    /// and must be a valid UUID otherwise API will return 400.
    ///
    #[arg(long)]
    image_ref: Option<String>,

    /// The flavor reference, as an ID (including a UUID) or full URL, for the
    /// flavor for your server instance.
    ///
    #[arg(long)]
    flavor_ref: String,

    /// The administrative password of the server. If you omit this parameter,
    /// the operation generates a new password.
    ///
    #[arg(long)]
    admin_pass: Option<String>,

    /// Metadata key and value pairs. The maximum size of the metadata key and
    /// value is 255 bytes each.
    ///
    #[arg(long, value_name="key=value", value_parser=parse_key_val::<String, String>)]
    metadata: Option<Vec<(String, String)>>,

    /// A list of `network` object. Required parameter when there are multiple
    /// networks defined for the tenant. When you do not specify the networks
    /// parameter, the server attaches to the only network created for the
    /// current tenant. Optionally, you can create one or more NICs on the
    /// server. To provision the server instance with a NIC for a network,
    /// specify the UUID of the network in the `uuid` attribute in a `networks`
    /// object. To provision the server instance with a NIC for an already
    /// existing port, specify the port-id in the `port` attribute in a
    /// `networks` object.
    ///
    /// If multiple networks are defined, the order in which they appear in the
    /// guest operating system will not necessarily reflect the order in which
    /// they are given in the server boot request. Guests should therefore not
    /// depend on device order to deduce any information about their network
    /// devices. Instead, device role tags should be used: introduced in 2.32,
    /// broken in 2.37, and re-introduced and fixed in 2.42, the `tag` is an
    /// optional, string attribute that can be used to assign a tag to a
    /// virtual network interface. This tag is then exposed to the guest in the
    /// metadata API and the config drive and is associated to hardware
    /// metadata for that network interface, such as bus (ex: PCI), bus address
    /// (ex: 0000:00:02.0), and MAC address.
    ///
    /// A bug has caused the `tag` attribute to no longer be accepted starting
    /// with version 2.37. Therefore, network interfaces could only be tagged
    /// in versions 2.32 to 2.36 inclusively. Version 2.42 has restored the
    /// `tag` attribute.
    ///
    /// Starting with microversion 2.37, this field is required and the special
    /// string values *auto* and *none* can be specified for networks. *auto*
    /// tells the Compute service to use a network that is available to the
    /// project, if one exists. If one does not exist, the Compute service will
    /// attempt to automatically allocate a network for the project (if
    /// possible). *none* tells the Compute service to not allocate a network
    /// for the instance. The *auto* and *none* values cannot be used with any
    /// other network values, including other network uuids, ports, fixed IPs
    /// or device tags. These are requested as strings for the networks value,
    /// not in a list. See the associated example.
    ///
    #[command(flatten)]
    networks: NetworksEnumGroupStruct,

    /// Controls how the API partitions the disk when you create, rebuild, or
    /// resize servers. A server inherits the `OS-DCF:diskConfig` value from
    /// the image from which it was created, and an image inherits the
    /// `OS-DCF:diskConfig` value from the server from which it was created. To
    /// override the inherited setting, you can include this attribute in the
    /// request body of a server create, rebuild, or resize request. If the
    /// `OS-DCF:diskConfig` value for an image is `MANUAL`, you cannot create a
    /// server from that image and set its `OS-DCF:diskConfig` value to `AUTO`.
    /// A valid value is:
    ///
    /// - `AUTO`. The API builds the server with a single partition the size of
    ///   the target flavor disk. The API automatically adjusts the file system
    ///   to fit the entire partition.
    /// - `MANUAL`. The API builds the server by using whatever partition
    ///   scheme and file system is in the source image. If the target flavor
    ///   disk is larger, the API does not partition the remaining disk space.
    ///
    #[arg(long)]
    os_dcf_disk_config: Option<OsDcfDiskConfig>,

    /// IPv4 address that should be used to access this server.
    ///
    #[arg(long)]
    access_ipv4: Option<String>,

    /// IPv6 address that should be used to access this server.
    ///
    #[arg(long)]
    access_ipv6: Option<String>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[arg(long)]
    availability_zone: Option<String>,

    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    block_device_mapping: Option<Vec<Value>>,

    /// Enables fine grained control of the block device mapping for an
    /// instance. This is typically used for booting servers from volumes. An
    /// example format would look as follows:
    ///
    /// > ```text
    /// > "block_device_mapping_v2": [{
    /// >  "boot_index": "0",
    /// >  "uuid": "ac408821-c95a-448f-9292-73986c790911",
    /// >  "source_type": "image",
    /// >  "volume_size": "25",
    /// >  "destination_type": "volume",
    /// >  "delete_on_termination": true,
    /// >  "tag": "disk1",
    /// >  "disk_bus": "scsi"}]
    /// >
    /// > ```
    ///
    /// In microversion 2.32, `tag` is an optional string attribute that can be
    /// used to assign a tag to the block device. This tag is then exposed to
    /// the guest in the metadata API and the config drive and is associated to
    /// hardware metadata for that block device, such as bus (ex: SCSI), bus
    /// address (ex: 1:0:2:0), and serial.
    ///
    /// A bug has caused the `tag` attribute to no longer be accepted starting
    /// with version 2.33. It has been restored in version 2.42.
    ///
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    block_device_mapping_v2: Option<Vec<Value>>,

    /// Indicates whether a config drive enables metadata injection. The
    /// config_drive setting provides information about a drive that the
    /// instance can mount at boot time. The instance reads files from the
    /// drive to get information that is normally available through the
    /// metadata service. This metadata is different from the user data. Not
    /// all cloud providers enable the `config_drive`. Read more in the
    /// [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html).
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    config_drive: Option<bool>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[arg(long)]
    key_name: Option<String>,

    #[arg(long)]
    min_count: Option<i32>,

    #[arg(long)]
    max_count: Option<i32>,

    /// Indicates whether a config drive enables metadata injection. The
    /// config_drive setting provides information about a drive that the
    /// instance can mount at boot time. The instance reads files from the
    /// drive to get information that is normally available through the
    /// metadata service. This metadata is different from the user data. Not
    /// all cloud providers enable the `config_drive`. Read more in the
    /// [OpenStack End User Guide](https://docs.openstack.org/nova/latest/user/config-drive.html).
    ///
    #[arg(action=clap::ArgAction::Set, long)]
    return_reservation_id: Option<bool>,

    /// One or more security groups. Specify the name of the security group in
    /// the `name` attribute. If you omit this attribute, the API creates the
    /// server in the `default` security group. Requested security groups are
    /// not applied to pre-existing ports.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    security_groups: Option<Vec<String>>,

    /// Configuration information or scripts to use upon launch. Must be Base64
    /// encoded. Restricted to 65535 bytes.
    ///
    /// Note
    ///
    /// The `null` value allowed in Nova legacy v2 API, but due to the strict
    /// input validation, it isn’t allowed in Nova v2.1 API.
    ///
    #[arg(long)]
    user_data: Option<String>,

    /// A free form description of the server. Limited to 255 characters in
    /// length. Before microversion 2.19 this was set to the server name.
    ///
    /// **New in version 2.19**
    ///
    #[arg(long)]
    description: Option<String>,

    /// A list of tags. Tags have the following restrictions:
    ///
    /// - Tag is a Unicode bytestring no longer than 60 characters.
    /// - Tag is a non-empty string.
    /// - ‘/’ is not allowed to be in a tag name
    /// - Comma is not allowed to be in a tag name in order to simplify
    ///   requests that specify lists of tags
    /// - All other characters are allowed to be in a tag name
    /// - Each server can have up to 50 tags.
    ///
    /// **New in version 2.52**
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    tags: Option<Vec<String>>,

    /// A list of trusted certificate IDs, which are used during image
    /// signature verification to verify the signing certificate. The list is
    /// restricted to a maximum of 50 IDs. This parameter is optional in server
    /// create requests if allowed by policy, and is not supported for
    /// volume-backed instances.
    ///
    /// **New in version 2.63**
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    trusted_image_certificates: Option<Vec<String>>,

    /// The hostname of the hypervisor on which the server is to be created.
    /// The API will return 400 if no hypervisors are found with the given
    /// hostname. By default, it can be specified by administrators only.
    ///
    /// **New in version 2.74**
    ///
    #[arg(long)]
    host: Option<String>,

    /// The hostname of the hypervisor on which the server is to be created.
    /// The API will return 400 if no hypervisors are found with the given
    /// hostname. By default, it can be specified by administrators only.
    ///
    /// **New in version 2.74**
    ///
    #[arg(long)]
    hypervisor_hostname: Option<String>,
}

/// OsSchedulerHints Body data
#[derive(Args)]
struct OsSchedulerHints {
    /// The server group UUID. Schedule the server according to a policy of the
    /// server group (`anti-affinity`, `affinity`, `soft-anti-affinity` or
    /// `soft-affinity`). It is available when `ServerGroupAffinityFilter`,
    /// `ServerGroupAntiAffinityFilter`, `ServerGroupSoftAntiAffinityWeigher`,
    /// `ServerGroupSoftAffinityWeigher` are available on cloud side.
    ///
    #[arg(long)]
    group: Option<String>,

    /// A list of server UUIDs or a server UUID. Schedule the server on a
    /// different host from a set of servers. It is available when
    /// `DifferentHostFilter` is available on cloud side.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    different_host: Option<Vec<String>>,

    /// A list of server UUIDs or a server UUID. Schedule the server on the
    /// same host as another server in a set of servers. It is available when
    /// `SameHostFilter` is available on cloud side.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    same_host: Option<Vec<String>>,

    /// Schedule the server by using a custom filter in JSON format. For
    /// example:
    ///
    /// ```text
    /// "query": "[\">=\",\"$free_ram_mb\",1024]"
    ///
    /// ```
    ///
    /// It is available when `JsonFilter` is available on cloud side.
    ///
    #[arg(long, value_name="JSON", value_parser=parse_json)]
    query: Option<Value>,

    /// A target cell name. Schedule the server in a host in the cell
    /// specified. It is available when `TargetCellFilter` is available on
    /// cloud side that is cell v1 environment.
    ///
    #[arg(long)]
    target_cell: Option<String>,

    /// A list of cell routes or a cell route (string). Schedule the server in
    /// a cell that is not specified. It is available when
    /// `DifferentCellFilter` is available on cloud side that is cell v1
    /// environment.
    ///
    #[arg(action=clap::ArgAction::Append, long)]
    different_cell: Option<Vec<String>>,

    /// Schedule the server on a host in the network specified with this
    /// parameter and a cidr (`os:scheduler_hints.cidr`). It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[arg(long)]
    build_near_host_ip: Option<String>,

    /// Schedule the server on a host in the network specified with an IP
    /// address (`os:scheduler_hints:build_near_host_ip`) and this parameter.
    /// If `os:scheduler_hints:build_near_host_ip` is specified and this
    /// parameter is omitted, `/24` is used. It is available when
    /// `SimpleCIDRAffinityFilter` is available on cloud side.
    ///
    #[arg(long)]
    cidr: Option<String>,
}

/// Server response representation
#[derive(Deserialize, Serialize, Clone, StructTable)]
struct ResponseData {
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
    #[structable(optional, title = "OS-DCF:diskConfig")]
    os_dcf_disk_config: Option<String>,

    /// The administrative password for the server. If you set
    /// `enable_instance_password` configuration option to `False`, the API
    /// wouldn’t return the `adminPass` field in response.
    ///
    #[serde(rename = "adminPass")]
    #[structable(optional, title = "adminPass")]
    admin_pass: Option<String>,

    /// The UUID of the server.
    ///
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// One or more security groups objects.
    ///
    #[serde()]
    #[structable(optional)]
    security_groups: Option<Value>,

    /// Links pertaining to usage. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    /// **New in version 2.40**
    ///
    #[serde()]
    #[structable(optional)]
    links: Option<Value>,
}

impl ServerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Server");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let mut ep_builder = create_274::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.74");

        // Set path parameters
        // Set query parameters
        // Set body parameters
        // Set Request.server data
        let args = &self.server;
        let mut server_builder = create_274::ServerBuilder::default();

        server_builder.name(&args.name);

        if let Some(val) = &args.image_ref {
            server_builder.image_ref(val);
        }

        server_builder.flavor_ref(&args.flavor_ref);

        if let Some(val) = &args.admin_pass {
            server_builder.admin_pass(val);
        }

        if let Some(val) = &args.metadata {
            server_builder.metadata(val.iter().cloned());
        }

        if let Some(data) = &args.networks.networks {
            let networks_builder: Vec<create_274::Networks> = data
                .iter()
                .flat_map(|v| serde_json::from_value::<create_274::Networks>(v.to_owned()))
                .collect();
            server_builder.networks(create_274::NetworksEnum::F1(networks_builder));
        }
        if args.networks.auto_networks {
            server_builder.networks(create_274::NetworksEnum::F2(
                create_274::NetworksStringEnum::Auto,
            ));
        }
        if args.networks.none_networks {
            server_builder.networks(create_274::NetworksEnum::F2(
                create_274::NetworksStringEnum::None,
            ));
        }

        if let Some(val) = &args.os_dcf_disk_config {
            let tmp = match val {
                OsDcfDiskConfig::Auto => create_274::OsDcfDiskConfig::Auto,
                OsDcfDiskConfig::Manual => create_274::OsDcfDiskConfig::Manual,
            };
            server_builder.os_dcf_disk_config(tmp);
        }

        if let Some(val) = &args.access_ipv4 {
            server_builder.access_ipv4(val);
        }

        if let Some(val) = &args.access_ipv6 {
            server_builder.access_ipv6(val);
        }

        if let Some(val) = &args.availability_zone {
            server_builder.availability_zone(val);
        }

        if let Some(val) = &args.block_device_mapping {
            let block_device_mapping_builder: Vec<create_274::BlockDeviceMapping> = val
                .iter()
                .flat_map(|v| {
                    serde_json::from_value::<create_274::BlockDeviceMapping>(v.to_owned())
                })
                .collect::<Vec<create_274::BlockDeviceMapping>>();
            server_builder.block_device_mapping(block_device_mapping_builder);
        }

        if let Some(val) = &args.block_device_mapping_v2 {
            let block_device_mapping_v2_builder: Vec<create_274::BlockDeviceMappingV2> = val
                .iter()
                .flat_map(|v| {
                    serde_json::from_value::<create_274::BlockDeviceMappingV2>(v.to_owned())
                })
                .collect::<Vec<create_274::BlockDeviceMappingV2>>();
            server_builder.block_device_mapping_v2(block_device_mapping_v2_builder);
        }

        if let Some(val) = &args.config_drive {
            server_builder.config_drive(*val);
        }

        if let Some(val) = &args.key_name {
            server_builder.key_name(val);
        }

        if let Some(val) = &args.min_count {
            server_builder.min_count(*val);
        }

        if let Some(val) = &args.max_count {
            server_builder.max_count(*val);
        }

        if let Some(val) = &args.return_reservation_id {
            server_builder.return_reservation_id(*val);
        }

        if let Some(val) = &args.security_groups {
            let security_groups_builder: Vec<create_274::SecurityGroups> = val
                .iter()
                .flat_map(|v| create_274::SecurityGroupsBuilder::default().name(v).build())
                .collect();
            server_builder.security_groups(security_groups_builder);
        }

        if let Some(val) = &args.user_data {
            server_builder.user_data(val);
        }

        if let Some(val) = &args.description {
            server_builder.description(Some(val.into()));
        }

        if let Some(val) = &args.tags {
            server_builder.tags(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.trusted_image_certificates {
            server_builder
                .trusted_image_certificates(val.iter().map(|v| v.into()).collect::<Vec<_>>());
        }

        if let Some(val) = &args.host {
            server_builder.host(val);
        }

        if let Some(val) = &args.hypervisor_hostname {
            server_builder.hypervisor_hostname(val);
        }

        ep_builder.server(server_builder.build().unwrap());

        // Set Request.os_scheduler_hints data
        if let Some(args) = &self.os_scheduler_hints {
            let mut os_scheduler_hints_builder = create_274::OsSchedulerHintsBuilder::default();
            if let Some(val) = &args.group {
                os_scheduler_hints_builder.group(val);
            }

            if let Some(val) = &args.different_host {
                os_scheduler_hints_builder
                    .different_host(val.iter().map(|v| v.into()).collect::<Vec<_>>());
            }

            if let Some(val) = &args.same_host {
                os_scheduler_hints_builder
                    .same_host(val.iter().map(|v| v.into()).collect::<Vec<_>>());
            }

            if let Some(val) = &args.query {
                os_scheduler_hints_builder.query(val.clone());
            }

            if let Some(val) = &args.target_cell {
                os_scheduler_hints_builder.target_cell(val);
            }

            if let Some(val) = &args.different_cell {
                os_scheduler_hints_builder
                    .different_cell(val.iter().map(|v| v.into()).collect::<Vec<_>>());
            }

            if let Some(val) = &args.build_near_host_ip {
                os_scheduler_hints_builder.build_near_host_ip(val);
            }

            if let Some(val) = &args.cidr {
                os_scheduler_hints_builder.cidr(val);
            }

            ep_builder.os_scheduler_hints(os_scheduler_hints_builder.build().unwrap());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
