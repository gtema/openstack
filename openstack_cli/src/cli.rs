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
//! CLI top level command and processing
//!
use clap::builder::{
    styling::{AnsiColor, Effects},
    Styles,
};
use clap::{Args, Parser, ValueEnum};

use openstack_sdk::AsyncOpenStack;

use crate::error::OpenStackCliError;

use crate::api;
use crate::auth;
use crate::block_storage::v3 as block_storage;
use crate::catalog;
use crate::compute::v2 as compute;
use crate::identity::v3 as identity;
use crate::image::v2 as image;
use crate::load_balancer::v2 as load_balancer;
use crate::network::v2 as network;
use crate::object_store::v1 as object_store;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::White.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
}

/// Main CLI command
#[derive(Parser)]
#[command(name="osc", author, version, about, long_about = None, styles = styles())]
#[command(propagate_version = true)]
pub struct Cli {
    /// Global cli arguments and options
    #[command(flatten)]
    pub global_opts: GlobalOpts,

    /// subcommand
    #[command(subcommand)]
    pub command: TopLevelCommands,
}

/// Global CLI options
#[derive(Args)]
#[command(next_display_order = 900, next_help_heading = "Global options")]
pub struct GlobalOpts {
    /// Name reference to the clouds.yaml entry for the cloud configuration
    #[arg(long, env = "OS_CLOUD", global = true, display_order = 900)]
    pub os_cloud: Option<String>,

    /// Project ID to use instead of the one in connection profile
    #[arg(long, env = "OS_PROJECT_ID", global = true, display_order = 901)]
    pub os_project_id: Option<String>,

    /// Project Name to use instead of the one in the connection profile
    #[arg(long, env = "OS_PROJECT_NAME", global = true, display_order = 901)]
    pub os_project_name: Option<String>,

    /// Output format
    #[arg(short, long, global = true, value_enum, display_order = 910)]
    pub output: Option<OutputFormat>,

    /// Fields to return in the output (only in normal and wide mode)
    #[arg(short, long, global=true, action=clap::ArgAction::Append, display_order = 910)]
    pub fields: Vec<String>,

    /// Pretty print the output
    #[arg(short, long, global=true, action = clap::ArgAction::SetTrue, display_order = 910)]
    pub pretty: bool,

    /// Verbosity level. Repeat to increase level.
    #[arg(short, long, global=true, action = clap::ArgAction::Count, display_order = 920)]
    pub verbose: u8,
}

/// Output format
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Json output
    Json,
    /// Wide (Human readable table with extra attributes). Note: this has
    /// effect only in list operations
    Wide,
}

/// Supported Top Level commands (services)
#[allow(missing_docs)]
#[derive(Parser)]
pub enum TopLevelCommands {
    Api(api::ApiCommand),
    Auth(auth::AuthCommand),
    BlockStorage(block_storage::BlockStorageCommand),
    Catalog(catalog::CatalogCommand),
    Compute(compute::ComputeCommand),
    Identity(identity::IdentityCommand),
    Image(image::ImageCommand),
    LoadBalancer(load_balancer::LoadBalancerCommand),
    Network(network::NetworkCommand),
    ObjectStore(object_store::ObjectStoreCommand),
}

impl Cli {
    /// Perform command action
    pub async fn take_action(&self, client: &mut AsyncOpenStack) -> Result<(), OpenStackCliError> {
        match &self.command {
            TopLevelCommands::Api(args) => args.take_action(self, client).await,
            TopLevelCommands::Auth(args) => args.take_action(self, client).await,
            TopLevelCommands::BlockStorage(args) => args.take_action(self, client).await,
            TopLevelCommands::Catalog(args) => args.take_action(self, client).await,
            TopLevelCommands::Compute(args) => args.take_action(self, client).await,
            TopLevelCommands::Identity(args) => args.take_action(self, client).await,
            TopLevelCommands::Image(args) => args.take_action(self, client).await,
            TopLevelCommands::LoadBalancer(args) => args.take_action(self, client).await,
            TopLevelCommands::Network(args) => args.take_action(self, client).await,
            TopLevelCommands::ObjectStore(args) => args.take_action(self, client).await,
        }
    }
}
