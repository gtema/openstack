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

//! Show catalog
//!
//!

use async_trait::async_trait;
use clap::{Args, Subcommand};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::fmt;
use tracing::info;

use anyhow::Result;

use openstack_sdk::AsyncOpenStack;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

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

impl OSCCommand for CatalogCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            CatalogCommands::List(args) => Ok(Box::new(ListCmd { args: args.clone() })),
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
impl OSCCommand for ListCmd {
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
            .unwrap_or_default()
            .into_iter()
            .map(|x| serde_json::to_value(x).unwrap())
            .collect();

        op.output_list::<Catalog>(data).unwrap();
        Ok(())
    }
}
