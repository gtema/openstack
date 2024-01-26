//! Updates a logical router.
//!
//! This operation does not enable the update of router interfaces.
//! To update a router interface, use the add router interface and
//! remove router interface operations.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 412
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
use crate::{error::OpenStackCliError, Command};
use std::fmt;
use structable_derive::StructTable;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::common::parse_json;
use crate::common::BoolString;
use openstack_sdk::api::find;
use openstack_sdk::api::network::v2::router::find;
use openstack_sdk::api::network::v2::router::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct RouterArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    router: Router,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.0/routers/{id} API
    #[arg()]
    id: String,
}
/// ExternalGatewayInfo Body data
#[derive(Args, Debug, Clone)]
#[group(required = false, multiple = true)]
struct ExternalGatewayInfo {
    #[arg(long, required = false)]
    network_id: String,

    #[arg(action=clap::ArgAction::Set, long)]
    enable_snat: Option<bool>,

    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    external_fixed_ips: Option<Vec<Value>>,
}

/// Router Body data
#[derive(Args, Debug, Clone)]
struct Router {
    /// Human-readable name of the resource.
    #[arg(long)]
    name: Option<String>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    #[arg(action=clap::ArgAction::Set, long)]
    admin_state_up: Option<bool>,

    /// The external gateway information of the router.
    /// If the router has an external gateway, this would be a dict with
    /// `network\_id`, `enable\_snat`, `external\_fixed\_ips` and
    /// `qos\_policy\_id`.
    /// Otherwise, this would be `null`.
    #[command(flatten)]
    external_gateway_info: Option<ExternalGatewayInfo>,

    /// `true` indicates a highly-available router.
    /// It is available when `l3-ha` extension is enabled.
    #[arg(action=clap::ArgAction::Set, long)]
    ha: Option<Option<bool>>,

    /// Enable NDP proxy attribute. Default is `false`, To persist this
    /// attribute
    /// value, set the `enable\_ndp\_proxy\_by\_default` option in the
    /// `neutron.conf` file. It is available when `router-extend-ndp-proxy`
    /// extension is enabled.
    #[arg(action=clap::ArgAction::Set, long)]
    enable_ndp_proxy: Option<Option<bool>>,

    /// `true` indicates a distributed router.
    /// It is available when `dvr` extension is enabled.
    #[arg(action=clap::ArgAction::Set, long)]
    distributed: Option<Option<bool>>,

    /// The extra routes configuration for L3 router.
    /// A list of dictionaries with `destination` and `nexthop` parameters.
    /// It is available when `extraroute` extension is enabled.
    /// Default is an empty list (`[]`).
    #[arg(action=clap::ArgAction::Append, long, value_name="JSON", value_parser=parse_json)]
    routes: Option<Vec<Value>>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[arg(long)]
    description: Option<String>,
}

/// Router set command
pub struct RouterCmd {
    pub args: RouterArgs,
}
/// Router response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the router.
    #[serde()]
    #[structable(optional)]
    id: Option<String>,

    /// Human-readable name of the resource.
    #[serde()]
    #[structable(optional)]
    name: Option<String>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    #[serde()]
    #[structable(optional)]
    admin_state_up: Option<BoolString>,

    /// The router status.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The ID of the project.
    #[serde()]
    #[structable(optional)]
    tenant_id: Option<String>,

    /// The external gateway information of the router.
    /// If the router has an external gateway, this would be a dict with
    /// `network\_id`, `enable\_snat`, `external\_fixed\_ips`,
    /// `qos\_policy\_id`, `enable\_default\_route\_ecmp` and
    /// `enable\_default\_route\_bfd`.
    /// Otherwise, this would be `null`.
    #[serde()]
    #[structable(optional)]
    external_gateway_info: Option<ResponseExternalGatewayInfo>,

    /// `true` indicates a highly-available router.
    /// It is available when `l3-ha` extension is enabled.
    #[serde()]
    #[structable(optional)]
    ha: Option<BoolString>,

    /// Enable NDP proxy attribute. `true` means NDP proxy is enabled for the
    /// router, the IPv6 address of internal subnets attached to the router can
    /// be
    /// published to external by create `ndp\_proxy`. `false` means NDP proxy
    /// is
    /// disabled, the IPv6 address of internal subnets attached to the router
    /// can
    /// not be published to external by `ndp\_proxy`. It is available when
    /// `router-extend-ndp-proxy` extension is enabled.
    #[serde()]
    #[structable(optional)]
    enable_ndp_proxy: Option<BoolString>,

    /// The ID of the flavor associated with the router.
    #[serde()]
    #[structable(optional)]
    flavor_id: Option<String>,

    /// The revision number of the resource.
    #[serde()]
    #[structable(optional)]
    revision_number: Option<i32>,

    /// The availability zone(s) for the router.
    /// It is available when `router\_availability\_zone` extension is enabled.
    #[serde()]
    #[structable(optional)]
    availability_zones: Option<VecString>,

    /// The availability zone candidates for the router.
    /// It is available when `router\_availability\_zone` extension is enabled.
    #[serde()]
    #[structable(optional)]
    availability_zone_hints: Option<VecString>,

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

    /// `true` indicates a distributed router.
    /// It is available when `dvr` extension is enabled.
    #[serde()]
    #[structable(optional)]
    distributed: Option<BoolString>,

    /// The associated conntrack helper resources for the roter. If the
    /// router has multiple conntrack helper resources, this field has
    /// multiple entries. Each entry consists of netfilter conntrack helper
    /// (`helper`), the network protocol (`protocol`), the network port
    /// (`port`).
    #[serde()]
    #[structable(optional)]
    conntrack_helpers: Option<String>,

    /// The extra routes configuration for L3 router.
    /// A list of dictionaries with `destination` and `nexthop` parameters.
    /// It is available when `extraroute` extension is enabled.
    #[serde()]
    #[structable(optional)]
    routes: Option<VecResponseRoutes>,

    /// A human-readable description for the resource.
    #[serde()]
    #[structable(optional)]
    description: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseExternalFixedIps {
    ip_address: Option<String>,
    subnet_id: Option<String>,
}

impl fmt::Display for ResponseExternalFixedIps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "ip_address={}",
                self.ip_address
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "subnet_id={}",
                self.subnet_id
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseExternalFixedIps(Vec<ResponseExternalFixedIps>);
impl fmt::Display for VecResponseExternalFixedIps {
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
struct ResponseExternalGatewayInfo {
    network_id: String,
    enable_snat: Option<bool>,
    external_fixed_ips: Option<VecResponseExternalFixedIps>,
}

impl fmt::Display for ResponseExternalGatewayInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!("network_id={}", self.network_id),
            format!(
                "enable_snat={}",
                self.enable_snat
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "external_fixed_ips={}",
                self.external_fixed_ips
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
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
struct ResponseRoutes {
    destination: Option<String>,
    nexthop: Option<String>,
}

impl fmt::Display for ResponseRoutes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = Vec::from([
            format!(
                "destination={}",
                self.destination
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
            format!(
                "nexthop={}",
                self.nexthop
                    .clone()
                    .map(|v| v.to_string())
                    .unwrap_or("".to_string())
            ),
        ]);
        write!(f, "{}", data.join(";"))
    }
}
#[derive(Deserialize, Default, Debug, Clone, Serialize)]
pub struct VecResponseRoutes(Vec<ResponseRoutes>);
impl fmt::Display for VecResponseRoutes {
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
impl Command for RouterCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Router with {:?}", self.args);

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
        // Set Request.router data
        let args = &self.args.router;
        let mut router_builder = set::RouterBuilder::default();
        if let Some(val) = &args.name {
            router_builder.name(val.clone());
        }

        if let Some(val) = &args.admin_state_up {
            router_builder.admin_state_up(*val);
        }

        if let Some(val) = &args.external_gateway_info {
            let mut external_gateway_info_builder = set::ExternalGatewayInfoBuilder::default();

            external_gateway_info_builder.network_id(val.network_id.clone());
            if let Some(val) = &val.enable_snat {
                external_gateway_info_builder.enable_snat(*val);
            }
            if let Some(val) = &val.external_fixed_ips {
                let external_fixed_ips_builder: Vec<set::ExternalFixedIps> = val
                    .iter()
                    .flat_map(|v| serde_json::from_value::<set::ExternalFixedIps>(v.clone()))
                    .collect::<Vec<set::ExternalFixedIps>>();
                external_gateway_info_builder.external_fixed_ips(external_fixed_ips_builder);
            }
            router_builder.external_gateway_info(
                external_gateway_info_builder
                    .build()
                    .expect("A valid object"),
            );
        }

        if let Some(val) = &args.ha {
            router_builder.ha(*val);
        }

        if let Some(val) = &args.enable_ndp_proxy {
            router_builder.enable_ndp_proxy(*val);
        }

        if let Some(val) = &args.distributed {
            router_builder.distributed(*val);
        }

        if let Some(val) = &args.routes {
            let routes_builder: Vec<set::Routes> = val
                .iter()
                .flat_map(|v| serde_json::from_value::<set::Routes>(v.clone()))
                .collect::<Vec<set::Routes>>();
            router_builder.routes(routes_builder);
        }

        if let Some(val) = &args.description {
            router_builder.description(val.clone());
        }

        ep_builder.router(router_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
