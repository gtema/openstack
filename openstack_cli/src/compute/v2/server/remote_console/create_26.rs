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

use clap::ValueEnum;
use openstack_sdk::api::compute::v2::server::remote_console::create_26;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// The API provides a unified request for creating a remote console. The user
/// can
/// get a URL to connect the console from this API. The URL includes the token
/// which is used to get permission to access the console. Servers may support
/// different console protocols. To return a remote console using a specific
/// protocol, such as RDP, set the `protocol` parameter to `rdp`.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404),
/// conflict(409), notImplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Create Console (microversion = 2.6)")]
pub struct RemoteConsoleArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    remote_console: RemoteConsole,
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

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Protocol {
    Rdp,
    Serial,
    Spice,
    Vnc,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum Type {
    Novnc,
    RdpHtml5,
    Serial,
    SpiceHtml5,
    Xvpvnc,
}

/// RemoteConsole Body data
#[derive(Args, Debug, Clone)]
struct RemoteConsole {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `rdp`, `serial` and `mks`. The protocol `mks` is added since
    /// Microversion `2.8`.
    #[arg(long)]
    protocol: Protocol,

    /// The type of remote console. The valid values are `novnc`,
    /// `rdp-html5`, `spice-html5`, `serial`, and `webmks`. The type
    /// `webmks` is added since Microversion `2.8`.
    #[arg(long)]
    _type: Type,
}

/// RemoteConsole create command
pub struct RemoteConsoleCmd {
    pub args: RemoteConsoleArgs,
}
/// RemoteConsole response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The protocol of remote console. The valid values are `vnc`, `spice`,
    /// `rdp`, `serial` and `mks`. The protocol `mks` is added since
    /// Microversion `2.8`.
    #[serde()]
    #[structable(optional)]
    protocol: Option<String>,

    /// The type of remote console. The valid values are `novnc`,
    /// `rdp-html5`, `spice-html5`, `serial`, and `webmks`. The type
    /// `webmks` is added since Microversion `2.8`.
    #[serde(rename = "type")]
    #[structable(optional, title = "type")]
    _type: Option<String>,

    /// The URL is used to connect the console.
    #[serde()]
    #[structable(optional)]
    url: Option<String>,
}

#[async_trait]
impl OSCCommand for RemoteConsoleCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Create RemoteConsole with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = create_26::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.6");

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.remote_console data
        let args = &self.args.remote_console;
        let mut remote_console_builder = create_26::RemoteConsoleBuilder::default();

        let tmp = match &args.protocol {
            Protocol::Rdp => create_26::Protocol::Rdp,
            Protocol::Serial => create_26::Protocol::Serial,
            Protocol::Spice => create_26::Protocol::Spice,
            Protocol::Vnc => create_26::Protocol::Vnc,
        };
        remote_console_builder.protocol(tmp);

        let tmp = match &args._type {
            Type::Novnc => create_26::Type::Novnc,
            Type::RdpHtml5 => create_26::Type::RdpHtml5,
            Type::Serial => create_26::Type::Serial,
            Type::SpiceHtml5 => create_26::Type::SpiceHtml5,
            Type::Xvpvnc => create_26::Type::Xvpvnc,
        };
        remote_console_builder._type(tmp);

        ep_builder.remote_console(remote_console_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
