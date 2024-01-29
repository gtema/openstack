//! Updates a network.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 403, 404, 412
//!
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
use crate::{error::OpenStackCliError, OSCCommand};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::BoolString;
use crate::common::IntString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::network::find;
use openstack_sdk::api::network::v2::network::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct NetworkArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    network: Network,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// network_id parameter for /v2.0/networks/{network_id} API
    #[arg()]
    id: String,
}
/// Network Body data
#[derive(Args, Debug, Clone)]
struct Network {
    /// Human-readable name of the network.
    #[arg(long)]
    name: Option<String>,

    /// The administrative state of the network, which is
    /// up (`true`) or down (`false`).
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// Indicates whether this resource is shared across all projects.
    /// By default, only administrative users can change this value.
    #[arg(action=clap::ArgAction::Set, long)]
    shared: Option<bool>,

    /// Indicates whether the network has an external routing facility that’s
    /// not
    /// managed by the networking service.
    #[arg(action=clap::ArgAction::Set, long)]
    router_external: Option<bool>,

    /// A list of provider `segment` objects.
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    segments: Option<Vec<Value>>,

    /// The maximum transmission unit (MTU) value to
    /// address fragmentation. Minimum value is 68 for IPv4, and 1280 for
    /// IPv6.
    #[arg(long)]
    mtu: Option<i32>,

    /// The port security status of the network. Valid values are
    /// enabled (`true`) and disabled (`false`).
    /// This value is used as the default value of `port\_security\_enabled`
    /// field of a newly created port.
    #[arg(action=clap::ArgAction::Set, long)]
    port_security_enabled: Option<bool>,

    #[arg(long)]
    provider_network_type: Option<String>,

    #[arg(long)]
    provider_physical_network: Option<String>,

    #[arg(long)]
    provider_segmentation_id: Option<String>,

    /// The ID of the QoS policy associated with the network.
    #[arg(long)]
    qos_policy_id: Option<String>,

    /// The network is default or not.
    #[arg(action=clap::ArgAction::Set, long)]
    is_default: Option<bool>,

    /// A valid DNS domain.
    #[arg(long)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,
}

/// Network set command
pub struct NetworkCmd {
    pub args: NetworkArgs,
}
/// Network response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the network.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the network.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The associated subnets.
    #[serde()]
    #[structable(optional)]
    subnets: Option<VecString>,

    /// The administrative state of the network, which is
    /// up (`true`) or down (`false`).
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The network status. Values are `ACTIVE`, `DOWN`, `BUILD` or `ERROR`.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// Indicates whether this network is shared across all tenants. By
    /// default,
    /// only administrative users can change this value.
    #[serde()]
    #[structable(optional)]
    shared: Option<BoolString>,

    /// The ID of the IPv4 address scope that the network is associated with.
    #[serde()]
    #[structable(optional)]
    ipv4_address_scope: Option<String>,

    /// The ID of the IPv6 address scope that the network is associated with.
    #[serde()]
    #[structable(optional)]
    ipv6_address_scope: Option<String>,

    /// Defines whether the network may be used for creation of floating IPs.
    /// Only
    /// networks with this flag may be an external gateway for routers.
    /// The network must have an external routing facility that is not managed
    /// by
    /// the networking service. If the network is updated from external to
    /// internal
    /// the unused floating IPs of this network are automatically deleted when
    /// extension `floatingip-autodelete-internal` is present.
    #[serde(rename = "router:external")]
    #[structable(optional, title = "router:external")]
    router_external: Option<BoolString>,

    /// Indicates whether L2 connectivity is available throughout
    /// the `network`.
    #[serde()]
    #[structable(optional)]
    l2_adjacency: Option<String>,

    /// A list of provider `segment` objects.
    #[serde()]
    #[structable(optional)]
    segments: Option<VecResponseSegments>,

    /// The maximum transmission unit (MTU) value to
    /// address fragmentation. Minimum value is 68 for IPv4, and 1280 for
    /// IPv6.
    #[serde()]
    #[structable(optional)]
    mtu: Option<i32>,

    /// The availability zone for the network.
    #[serde()]
    #[structable(optional)]
    availability_zones: Option<VecString>,

    /// The availability zone candidate for the network.
    #[serde()]
    #[structable(optional)]
    availability_zone_hints: Option<VecString>,

    /// The port security status of the network. Valid values are
    /// enabled (`true`) and disabled (`false`).
    /// This value is used as the default value of `port\_security\_enabled`
    /// field of a newly created port.
    #[serde()]
    #[structable(optional)]
    port_security_enabled: Option<BoolString>,

    #[serde(rename = "provider:network_type")]
    #[structable(optional, title = "provider:network_type")]
    provider_network_type: Option<String>,

    #[serde(rename = "provider:physical_network")]
    #[structable(optional, title = "provider:physical_network")]
    provider_physical_network: Option<String>,

    #[serde(rename = "provider:segmentation_id")]
    #[structable(optional, title = "provider:segmentation_id")]
    provider_segmentation_id: Option<IntString>,

    /// The ID of the QoS policy associated with the network.
    #[serde()]
    #[structable(optional)]
    qos_policy_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The list of tags on the resource.
    #[serde()]
    #[structable(optional)]
    tags: Option<VecString>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The network is default pool or not.
    #[serde()]
    #[structable(optional)]
    is_default: Option<BoolString>,

    /// A valid DNS domain.
    #[serde()]
    #[structable(optional)]
    dns_domain: Option<String>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
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
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseSegments {
    provider_segmentation_id: Option<i32>,
    provider_physical_network: Option<String>,
    provider_network_type: Option<String>,
}

impl fmt::Display for ResponseSegments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "provider_segmentation_id={}",
                self.provider_segmentation_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "provider_physical_network={}",
                self.provider_physical_network
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "provider_network_type={}",
                self.provider_network_type
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseSegments(Vec<ResponseSegments>);
impl fmt::Display for VecResponseSegments {
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

#[async_trait]
impl OSCCommand for NetworkCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Network with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut find_builder = find::Request::builder();

        find_builder.id(&self.args.path.id);
        let find_ep = find_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;
        let find_data: serde_json::Value = find(find_ep).query_async(client).await?;
        let mut ep_builder = set::Request::builder();

        // Set path parameters
        let resource_id = find_data["id"]
            .as_str()
            .expect("Resource ID is a string")
            .to_string();
        ep_builder.id(resource_id.clone());
        // Set query parameters
        // Set body parameters
        // Set Request.network data
        let args = &self.args.network;
        let mut network_builder = set::NetworkBuilder::default();
        if let Some(val) = &args.name {
            network_builder.name(val.clone());
        }

        if let Some(val) = &args.admin_state_up {
            network_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.shared {
            network_builder.shared(*val);
        }

        if let Some(val) = &args.router_external {
            network_builder.router_external(*val);
        }

        if let Some(val) = &args.segments {
            let segments_builder: Vec<set::Segments> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::Segments>(v.clone()))
                .collect::<Vec<set::Segments>>();
            network_builder.segments(segments_builder);
        }

        if let Some(val) = &args.mtu {
            network_builder.mtu(*val);
        }

        if let Some(val) = &args.port_security_enabled {
            network_builder.port_security_enabled(*val);
        }

        if let Some(val) = &args.provider_network_type {
            network_builder.provider_network_type(val.clone());
        }

        if let Some(val) = &args.provider_physical_network {
            network_builder.provider_physical_network(val.clone());
        }

        if let Some(val) = &args.provider_segmentation_id {
            network_builder.provider_segmentation_id(val.clone());
        }

        if let Some(val) = &args.qos_policy_id {
            network_builder.qos_policy_id(Some(val.into()));
        }

        if let Some(val) = &args.is_default {
            network_builder.is_default(*val);
        }

        if let Some(val) = &args.dns_domain {
            network_builder.dns_domain(val.clone());
        }

        if let Some(val) = &args.description {
            network_builder.description(val.clone());
        }

        ep_builder.network(network_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
