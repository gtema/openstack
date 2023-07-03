//! Show catalog
//!
//!

use async_trait::async_trait;
use clap::{Args, Subcommand};
use http::{Response, Uri};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use tracing::{debug, info};

use anyhow::Result;
use url::Url;

use openstack_sdk::{
    api::{AsyncClient, RestClient},
    AsyncOpenStack,
};

use crate::common::parse_key_val;
use crate::error::OpenStackCliError;
use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{Command, ResourceCommands};
use structable_derive::StructTable;

use openstack_sdk::types::identity::v3::ServiceEndpoints;

/// List catalog command arguments
#[derive(Args, Clone, Debug)]
pub struct ListArgs {}

/// List Command
pub struct ListCmd {
    pub args: ListArgs,
}

/// Catalog commands args
#[derive(Args, Clone, Debug)]
pub struct CatalogArgs {
    #[command(subcommand)]
    command: CatalogCommands,
}

/// Catalog command types
#[derive(Subcommand, Clone, Debug)]
pub enum CatalogCommands {
    List(ListArgs),
}

/// Catalog command
pub struct CatalogCommand {
    pub args: CatalogArgs,
}

impl ResourceCommands for CatalogCommand {
    fn get_command(&self) -> Box<dyn Command> {
        match &self.args.command {
            CatalogCommands::List(args) => Box::new(ListCmd { args: args.clone() }),
        }
    }
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct VecCatalogEndpoints(pub Vec<CatalogEndpoint>);
/// Catalog
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct Catalog {
    /// Service type
    #[structable(title = "service_type")]
    #[serde(rename = "type")]
    service_type: String,

    /// Service name
    #[structable(title = "service_name")]
    name: String,

    /// Service endpoints
    endpoints: VecCatalogEndpoints,
}

#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct CatalogEndpoint {
    /// id
    id: String,
    /// Interface
    interface: String,
    ///Region
    region: String,
    /// URL
    url: String,
}

impl fmt::Display for CatalogEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "interface: {}, region: {}, url: {}",
            self.interface, self.region, self.url
        )
    }
}

impl fmt::Display for VecCatalogEndpoints {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[async_trait]
impl Command for ListCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show Catalog");

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;

        let data: Vec<Value> = client
            .get_token_catalog()
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|x| serde_json::to_value(x).unwrap())
            .collect();

        op.output_list::<Catalog>(data).unwrap();
        Ok(())
    }
}
