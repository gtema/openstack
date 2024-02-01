use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::compute::v2::server::interface::create_249;
use openstack_sdk::api::QueryAsync;
use std::fmt;
use structable_derive::StructTable;

/// Creates a port interface and uses it to attach a port to a server.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404), conflict(409), computeFault(500), NotImplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Create Interface (microversion = 2.49)")]
pub struct InterfaceArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    interface_attachment: InterfaceAttachment,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// server_id parameter for /v2.1/servers/{server_id}/topology API
    #[arg(value_name = "SERVER_ID", id = "path_param_server_id")]
    server_id: String,
}
/// InterfaceAttachment Body data
#[derive(Args, Debug, Clone)]
struct InterfaceAttachment {
    /// The ID of the network for which you want to create a port interface.
    /// The `net\_id`
    /// and `port\_id` parameters are mutually exclusive. If you do not specify
    /// the
    /// `net\_id` parameter, the OpenStack Networking API v2.0 uses the network
    /// information
    /// cache that is associated with the instance.
    #[arg(long)]
    net_id: Option<String>,

    /// The ID of the port for which you want to create an interface. The
    /// `net\_id`
    /// and `port\_id` parameters are mutually exclusive. If you do not specify
    /// the
    /// `port\_id` parameter, the OpenStack Networking API v2.0 allocates a
    /// port and
    /// creates an interface for it on the network.
    #[arg(long)]
    port_id: Option<String>,

    /// Fixed IP addresses. If you request a specific fixed IP address without
    /// a `net\_id`, the request returns a `Bad Request (400)` response code.
    #[arg(action=clap::ArgAction::Append, long)]
    fixed_ips: Option<Vec<String>>,

    /// A device role tag that can be applied to a network interface when
    /// attaching
    /// it to the VM. The guest OS of a server that has devices tagged in this
    /// manner can access hardware metadata about the tagged devices from the
    /// metadata API and on the config
    /// drive, if enabled.
    ///
    ///
    /// **New in version 2.49**
    #[arg(long)]
    tag: Option<String>,
}

/// Interface create command
pub struct InterfaceCmd {
    pub args: InterfaceArgs,
}
/// Interface response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// Fixed IP addresses with subnet IDs.
    #[serde()]
    #[structable(optional)]
    fixed_ips: Option<VecResponseFixedIps>,

    /// The MAC address.
    #[serde()]
    #[structable(optional)]
    mac_addr: Option<String>,

    /// The network ID.
    #[serde()]
    #[structable(optional)]
    net_id: Option<String>,

    /// The port ID.
    #[serde()]
    #[structable(optional)]
    port_id: Option<String>,

    /// The port state.
    #[serde()]
    #[structable(optional)]
    port_state: Option<String>,

    /// The device tag applied to the virtual network interface or `null`.
    ///
    ///
    /// **New in version 2.70**
    #[serde()]
    #[structable(optional)]
    tag: Option<String>,
}
#[derive(Deserialize, Debug, Default, Clone, Serialize)]
struct ResponseFixedIps {
    ip_address: Option<String>,
    subnet_id: Option<String>,
}

impl fmt::Display for ResponseFixedIps {
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
pub struct VecResponseFixedIps(Vec<ResponseFixedIps>);
impl fmt::Display for VecResponseFixedIps {
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
impl OSCCommand for InterfaceCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create Interface with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_249::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.49");

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.interface_attachment data
        let args = &self.args.interface_attachment;
        let mut interface_attachment_builder = create_249::InterfaceAttachmentBuilder::default();
        if let Some(val) = &args.net_id {
            interface_attachment_builder.net_id(val.clone());
        }

        if let Some(val) = &args.port_id {
            interface_attachment_builder.port_id(val.clone());
        }

        if let Some(val) = &args.fixed_ips {
            let fixed_ips_builder: Vec<create_249::FixedIps> = val
                .iter()
                .flat_map(|v| create_249::FixedIpsBuilder::default().ip_address(v).build())
                .collect();
            interface_attachment_builder.fixed_ips(fixed_ips_builder);
        }

        if let Some(val) = &args.tag {
            interface_attachment_builder.tag(val.clone());
        }

        ep_builder.interface_attachment(interface_attachment_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
