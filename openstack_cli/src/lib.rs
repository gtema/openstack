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

#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
// Allow Enum variant to end with enum's name
// enum Type {
//   ...
//   FileType
//   ...
// }
#![allow(clippy::enum_variant_names)]
use std::io::{self, IsTerminal};

use anyhow::Context;
use clap::Parser;
use tracing::Level;

use openstack_sdk::{
    auth::authtoken::AuthTokenScope, types::identity::v3::Project, AsyncOpenStack,
};

mod api;
mod auth;
mod block_storage;
mod catalog;
mod common;
mod compute;
mod identity;
mod image;
mod load_balancer;
mod network;
mod object_store;

pub mod cli;
pub mod error;
pub mod output;

use crate::error::OpenStackCliError;

pub use cli::Cli;
use cli::TopLevelCommands;

pub(crate) use output::OutputConfig;
pub(crate) use output::StructTable;

/// Entry point for the CLI wrapper
pub async fn entry_point() -> Result<(), OpenStackCliError> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(match cli.global_opts.verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .init();

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
        ))
        .with_context(|| "Error loading the connection configuration")?;
    let mut renew_auth: bool = false;

    // Login command need to be analyzed before authorization
    if let TopLevelCommands::Auth(args) = &cli.command {
        if let auth::AuthCommands::Login(login_args) = &args.command {
            if login_args.renew {
                renew_auth = true;
            }
        }
    }

    let mut session;
    if std::io::stdin().is_terminal() {
        session = AsyncOpenStack::new_interactive(&profile, renew_auth)
            .await
            .with_context(|| "Error during authenticating")?;
    } else {
        session = AsyncOpenStack::new(&profile)
            .await
            .with_context(|| "Error during authenticating")?;
    }
    if cli.global_opts.os_project_id.is_some() || cli.global_opts.os_project_name.is_some() {
        let current_project = session
            .get_auth_info()
            .expect("Already authenticated")
            .token
            .project;
        let project = Project {
            id: cli.global_opts.os_project_id.clone(),
            name: cli.global_opts.os_project_name.clone(),
            domain: current_project.expect("Current scope is project").domain,
        };
        let scope = AuthTokenScope::Project(project.clone());
        session
            .authorize(Some(scope), std::io::stdin().is_terminal(), renew_auth)
            .await
            .with_context(|| format!("Error during changing scope to {:?}", project))?;
    }

    // Invoke the command
    cli.take_action(&mut session).await
}
