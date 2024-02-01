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

use openstack_sdk::api::compute::v2::server::os_get_console_output;
use openstack_sdk::api::QueryAsync;
use structable_derive::StructTable;

/// Shows console output for a server.
///
/// This API returns the text of the console since boot.
/// The content returned may be large. Limit the lines of console
/// text, beginning at the tail of the content, by setting
/// the optional `length` parameter in the request body.
///
/// The server to get console log from should set
/// `export LC\_ALL=en\_US.UTF-8` in order to avoid incorrect unicode error.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403),
/// notFound(404), conflict(409), methodNotImplemented(501)
#[derive(Args, Clone, Debug)]
#[command(about = "Show Console Output (os-getConsoleOutput Action)")]
pub struct ServerArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[command(flatten)]
    os_get_console_output: OsGetConsoleOutput,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// id parameter for /v2.1/servers/{id}/action API
    #[arg(value_name = "ID", id = "path_param_id")]
    id: String,
}
/// OsGetConsoleOutput Body data
#[derive(Args, Debug, Clone)]
struct OsGetConsoleOutput {
    /// The number of lines to fetch from the end of console log. All
    /// lines will be returned if this is not specified.
    ///
    ///
    ///
    /// Note
    ///
    ///
    /// This parameter can be specified as not only ‘integer’ but also
    /// ‘string’.
    #[arg(long)]
    length: Option<String>,
}

/// Server action command
pub struct ServerCmd {
    pub args: ServerArgs,
}
/// Server response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The console output as a string. Control characters will be escaped
    /// to create a valid JSON string.
    #[serde()]
    #[structable()]
    output: String,
}

#[async_trait]
impl OSCCommand for ServerCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Action Server with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = os_get_console_output::Request::builder();

        // Set path parameters
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        // Set Request.os_get_console_output data
        let args = &self.args.os_get_console_output;
        let mut os_get_console_output_builder =
            os_get_console_output::OsGetConsoleOutputBuilder::default();
        if let Some(val) = &args.length {
            os_get_console_output_builder.length(Some(val.into()));
        }

        ep_builder.os_get_console_output(os_get_console_output_builder.build().unwrap());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
