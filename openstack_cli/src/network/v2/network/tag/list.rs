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
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use openstack_sdk::api::network::v2::network::tag::list;
use openstack_sdk::api::QueryAsync;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct TagsArgs {
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
    /// network_id parameter for /v2.0/networks/{network_id} API
    #[arg()]
    network_id: String,
}

/// Tags list command
pub struct TagsCmd {
    pub args: TagsArgs,
}
/// Tags response representation
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
            .into_iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<_>>()
            .join(", ")])]);
        (headers, res)
    }
}

#[async_trait]
impl OSCCommand for TagsCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("List Tags with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = list::Request::builder();

        // Set path parameters
        ep_builder.network_id(&self.args.path.network_id);
        // Set query parameters
        // Set body parameters

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        Ok(())
    }
}
