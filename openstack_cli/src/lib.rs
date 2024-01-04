//! OpenStackCLI - a.k.a. `osc` rewritten in Rust.
//!
//! This is a new OpenStackCLI written in Rust.
//!
//! Facts:
//!  - not specifying `-o` will cause a Table output with a CLI
//!  normalized and known attribute names only. Requesting
//!  unsupported fields present in the API response is not going
//!  to be supported (due to the name convention collision)
//!
//! - `-o wide` is still considered a human response and support
//! normalized/supported names only
//!
//! - `--plain` may be implemented to output a text form table
//! without borders and separators
//!
//! - `-o json` is treated as machine response and returns
//! server side names and does not support requesting certain
//! fields (they are not known in advance). This decision may be
//! re-evaluated
//!
//! More description to come
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use std::io::{self, Write};

use clap::{Args, Parser, Subcommand, ValueEnum};

use async_trait::async_trait;
use std::collections::BTreeSet;

// use thiserror::Error;
use anyhow::Context;

use tracing::Level;

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use cli_table::{print_stdout, Table};

mod api;
mod block_storage;
mod catalog;
mod common;
mod compute;
pub mod error;
mod image;
mod network;
mod object_store;
mod output;

use crate::error::OpenStackCliError;

use crate::api::{ApiArgs, ApiCommand};
use crate::block_storage::v3::{BlockStorageSrvArgs, BlockStorageSrvCommand};
use crate::catalog::{CatalogArgs, CatalogCommand};
use crate::compute::v2::{ComputeSrvArgs, ComputeSrvCommand};
use crate::image::v2::{ImageSrvArgs, ImageSrvCommand};
use crate::network::v2::{NetworkSrvArgs, NetworkSrvCommand};
use crate::object_store::v1::{ObjectStoreSrvArgs, ObjectStoreSrvCommand};

/// OpenStack CLI rewritten in Rust
#[derive(Parser)]
#[command(name="osc", author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Global options
    #[command(flatten)]
    pub global_opts: GlobalOpts,

    /// Command
    #[command(subcommand)]
    pub command: TopLevelCommands,
}

/// Supported Top Level commands
#[derive(Subcommand)]
pub enum TopLevelCommands {
    /// Block Storage (Volume) service (Cinder) commands
    BlockStorage(BlockStorageSrvArgs),
    /// Compute service (Nova) commands
    Compute(ComputeSrvArgs),
    /// Image (Glance) commands
    Image(ImageSrvArgs),
    /// Network (Neutron) commands
    Network(NetworkSrvArgs),
    /// Object Store service (Swift) commands
    ObjectStore(ObjectStoreSrvArgs),
    /// Shows current catalog information
    Catalog(CatalogArgs),
    /// Perform direct REST API requests with authorization
    Api(ApiArgs),
}

/// Global CLI options
#[derive(Debug, Args, Clone)]
pub struct GlobalOpts {
    /// Name reference to the clouds.yaml entry for the cloud configuration
    #[arg(long, env = "OS_CLOUD", global = true)]
    pub os_cloud: Option<String>,

    /// Output format
    #[arg(short, long, global = true, value_enum)]
    pub output: Option<OutputFormat>,

    /// Fields to return in the output (only in normal and wide mode)
    #[arg(short, long, global=true, action=clap::ArgAction::Append)]
    pub fields: Vec<String>,

    /// Verbosity level. Repeat to increase level.
    #[arg(short, long, global=true, action = clap::ArgAction::Count)]
    verbose: u8,
}

/// Output configuration data structure
#[derive(Clone, Debug, Default)]
pub(crate) struct OutputConfig {
    /// Set of fields to be included in the response
    fields: BTreeSet<String>,
    /// Flag whether to include additional attributes in the output
    wide: bool,
}

/// Trait for structures that should be represented as a table in the human output mode
pub(crate) trait StructTable {
    fn build(&self, options: &OutputConfig) -> (Vec<String>, Vec<Vec<String>>);
}

/// Output format
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Json output
    Json,
    /// YAML output
    Yaml,
    /// Wide (Human readable table with extra attributes)
    Wide,
}

/// Command trait for individual resource command implementation
#[async_trait]
pub trait Command {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError>;
}

/// Service trait as service resources wrapper
pub trait ServiceCommands {
    fn get_command(&self) -> Box<dyn Command>;
}

/// Individual resource trait
pub trait ResourceCommands {
    fn get_command(&self) -> Box<dyn Command>;
}

/// Entry point for the CLI wrapper
pub async fn entry_point() -> Result<(), OpenStackCliError> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(match cli.global_opts.verbose {
            0 => Level::ERROR,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .init();

    let cmd = match &cli.command {
        TopLevelCommands::BlockStorage(args) => {
            BlockStorageSrvCommand { args: args.clone() }.get_command()
        }
        TopLevelCommands::Compute(args) => ComputeSrvCommand { args: args.clone() }.get_command(),
        TopLevelCommands::Image(args) => ImageSrvCommand { args: args.clone() }.get_command(),
        TopLevelCommands::Network(args) => NetworkSrvCommand { args: args.clone() }.get_command(),
        TopLevelCommands::ObjectStore(args) => {
            ObjectStoreSrvCommand { args: args.clone() }.get_command()
        }
        TopLevelCommands::Catalog(args) => {
            catalog::CatalogCommand { args: args.clone() }.get_command()
        }
        TopLevelCommands::Api(args) => Box::new(api::ApiCommand { args: args.clone() }),
    };
    let cfg = openstack_sdk::config::ConfigFile::new().unwrap();
    let profile = cfg
        .get_cloud_config(
            cli.global_opts
                .os_cloud
                .clone()
                .expect("--os-cloud or OS_CLOUD env must be given"),
        )?
        .ok_or(OpenStackCliError::ConnectionNotFound(
            cli.global_opts.os_cloud.clone().unwrap(),
        ))?;

    let mut session = AsyncOpenStack::new(&profile).await?;
    match &cli.command {
        TopLevelCommands::BlockStorage(args) => {
            session
                .discover_service_endpoint(&ServiceType::BlockStorage)
                .await?;
        }
        TopLevelCommands::Compute(args) => {
            session
                .discover_service_endpoint(&ServiceType::Compute)
                .await?;
        }
        TopLevelCommands::Image(args) => {
            session
                .discover_service_endpoint(&ServiceType::Image)
                .await?;
        }
        TopLevelCommands::Network(args) => {
            session
                .discover_service_endpoint(&ServiceType::Network)
                .await?;
        }
        _ => {}
    };
    cmd.take_action(&cli, &mut session).await?;
    Ok(())
}
