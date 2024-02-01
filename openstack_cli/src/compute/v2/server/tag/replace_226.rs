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

use openstack_sdk::api::compute::v2::server::tag::replace_226;
use openstack_sdk::api::QueryAsync;

/// Replaces all tags on specified server with the new set of tags.
///
/// Normal response codes: 200
///
/// Error response codes: badRequest(400), unauthorized(401), forbidden(403),
/// itemNotFound(404)
#[derive(Args, Clone, Debug)]
#[command(about = "Replace Tags (microversion = 2.26)")]
pub struct TagArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(action=clap::ArgAction::Append, long)]
    tags: Vec<String>,
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

/// Tag set command
pub struct TagCmd {
    pub args: TagArgs,
}
/// Tag response representation
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
impl OSCCommand for TagCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Tag with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = replace_226::Request::builder();
        ep_builder.header("OpenStack-API-Version", "compute 2.26");

        // Set path parameters
        ep_builder.server_id(&self.args.path.server_id);
        // Set query parameters
        // Set body parameters
        // Set Request.tags data
        let args = &self.args.tags;

        ep_builder.tags(args.iter().map(|v| v.into()).collect::<Vec<_>>());

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data: Vec<serde_json::Value> = ep.query_async(client).await?;
        op.output_list::<ResponseData>(data)?;
        Ok(())
    }
}
