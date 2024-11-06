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

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use dialoguer::FuzzySelect;
use eyre::eyre;
use tracing::{warn, Level};

use openstack_sdk::{
    auth::authtoken::AuthTokenScope, types::identity::v3::Project, AsyncOpenStack,
};

mod api;
mod auth;
mod block_storage;
mod catalog;
mod common;
mod compute;
mod dns;
mod identity;
mod image;
mod load_balancer;
mod network;
mod object_store;
mod placement;

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

    if let TopLevelCommands::Completion(args) = &cli.command {
        generate(
            args.shell,
            &mut Cli::command(),
            Cli::command().get_name().to_string(),
            &mut io::stdout(),
        );
        return Ok(());
    }

    tracing_subscriber::fmt()
        .with_writer(io::stderr)
        .with_max_level(match cli.global_opts.verbose {
            0 => Level::WARN,
            1 => Level::INFO,
            2 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .init();

    let cfg = openstack_sdk::config::ConfigFile::new_with_user_specified_configs(
        cli.global_opts.os_client_config_file.as_deref(),
        cli.global_opts.os_client_secure_file.as_deref(),
    )?;

    // Identify target cloud to connect to
    let cloud_name = match cli.global_opts.os_cloud {
        Some(ref cloud) => cloud.clone(),
        None => {
            if std::io::stdin().is_terminal() {
                // Cloud was not selected and we are in the potentially interactive mode (terminal)
                let mut profiles = cfg.get_available_clouds();
                profiles.sort();
                let selected_cloud_idx = FuzzySelect::new()
                    .with_prompt("Please select cloud you want to connect to (use `--os-cloud` next time for efficiency)?")
                    .items(&profiles)
                    .interact()?;
                profiles[selected_cloud_idx].clone()
            } else {
                return Err(
                    eyre!("`--os-cloud` or `OS_CLOUD` environment variable must be given").into(),
                );
            }
        }
    };
    // Get the connection details
    let profile = cfg
        .get_cloud_config(&cloud_name)?
        .ok_or(OpenStackCliError::ConnectionNotFound(cloud_name))?;
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
        // Interactive session (may ask for password/MFA/SSO)
        session = AsyncOpenStack::new_interactive(&profile, renew_auth)
            .await
            .map_err(|err| OpenStackCliError::Auth { source: err })?;
    } else {
        // Non-interactive session if i.e. scripted with chaining
        session = AsyncOpenStack::new(&profile)
            .await
            .map_err(|err| OpenStackCliError::Auth { source: err })?;
    }
    if cli.global_opts.os_project_id.is_some() || cli.global_opts.os_project_name.is_some() {
        warn!(
            "Cloud config is being chosen with arguments overriding project. Result may be not as expected."
        );
        let current_auth = session
            .get_auth_info()
            .expect("Already authenticated")
            .token;
        let project = Project {
            id: cli.global_opts.os_project_id.clone(),
            name: cli.global_opts.os_project_name.clone(),
            domain: match (current_auth.project, current_auth.domain) {
                // New project is in the same domain as the original
                (Some(project), _) => project.domain,
                // domain scope was used
                (None, Some(domain)) => Some(domain),
                // There was no scope thus using user domain
                _ => current_auth.user.domain,
            },
        };
        let scope = AuthTokenScope::Project(project.clone());
        session
            .authorize(
                Some(scope.clone()),
                std::io::stdin().is_terminal(),
                renew_auth,
            )
            .await
            .map_err(|err| OpenStackCliError::ReScope { scope, source: err })?;
    }

    // Invoke the command
    cli.take_action(&mut session).await
}
