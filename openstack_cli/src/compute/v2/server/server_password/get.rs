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

use openstack_sdk::api::compute::v2::server::server_password::get;
use openstack_sdk::api::QueryAsync;

/// Shows the administrative password for a server.
///
/// This operation calls the metadata service to query metadata information and
/// does not read password information from the server itself.
///
/// The password saved in the metadata service is typically encrypted using the
/// public SSH key injected into this server, so the SSH private key is needed
/// to
/// read the password.
///
/// Policy defaults enable only users with the administrative role or the owner
/// of the server to perform this operation. Cloud providers can change these
/// permissions through the `policy.json` file.
///
/// Normal response codes: 200
///
/// Error response codes: unauthorized(401), forbidden(403), itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Show Server Password")]
pub struct ServerPasswordArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,
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

/// ServerPassword get command
pub struct ServerPasswordCmd {
    pub args: ServerPasswordArgs,
}
/// ServerPassword response representation
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct ResponseData(String);

impl StructTable for ResponseData {
    fn build(&self, _: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Value".to_string()]);
        let res: Vec<Vec<String>> = Vec::from([Vec::from([self.0.to_string()])]);
        (headers, res)
    }
}

impl StructTable for Vec<ResponseData> {
    fn build(&self, _: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>) {
        let headers: Vec<String> = Vec::from(["Values".to_string()]);
        let res: Vec<Vec<String>> = Vec::from([Vec::from([self
            .iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")])]);
        (headers, res)
    }
}

#[async_trait]
impl OSCCommand for ServerPasswordCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Get ServerPassword with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = get::Request::builder();

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
