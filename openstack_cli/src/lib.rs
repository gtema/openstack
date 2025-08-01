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
use clap_complete::Generator;
use dialoguer::FuzzySelect;
use eyre::eyre;
use std::sync::{Arc, Mutex};
use tracing::warn;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{Layer, prelude::*};

use openstack_sdk::{
    AsyncOpenStack,
    auth::auth_helper::{Dialoguer, ExternalCmd, Noop},
    auth::authtoken::AuthTokenScope,
    types::identity::v3::Project,
};

pub mod api;
pub mod auth;
pub mod block_storage;
pub mod catalog;
mod common;
pub mod compute;
pub mod config;
pub mod container_infrastructure_management;
pub mod dns;
pub mod identity;
pub mod image;
pub mod load_balancer;
pub mod network;
pub mod object_store;
pub mod placement;

mod tracing_stats;

pub mod cli;
pub mod error;
pub mod output;

use crate::error::OpenStackCliError;
use crate::tracing_stats::{HttpRequestStats, RequestTracingCollector};

pub use cli::Cli;
use cli::TopLevelCommands;

use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::presets::UTF8_FULL_CONDENSED;

/// Entry point for the CLI wrapper
pub async fn entry_point() -> Result<(), OpenStackCliError> {
    let cli = Cli::parse();

    if let TopLevelCommands::Completion(args) = &cli.command {
        // generate completion output
        let mut cmd = Cli::command();
        cmd.set_bin_name(cmd.get_name().to_string());
        cmd.build();
        // Ignore any error during writing the completion
        args.shell.try_generate(&cmd, &mut io::stdout()).ok();
        return Ok(());
    }

    // Initialize tracing layers
    // fmt for console logging
    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(match cli.global_opts.output.verbose {
            0 => LevelFilter::WARN,
            1 => LevelFilter::INFO,
            2 => LevelFilter::DEBUG,
            _ => LevelFilter::TRACE,
        })
        .boxed();

    // RequestTracingCollector for capturing http statistics
    let request_stats = Arc::new(Mutex::new(HttpRequestStats::default()));
    let rtl = RequestTracingCollector {
        stats: request_stats.clone(),
    }
    .boxed();

    // build the tracing registry
    tracing_subscriber::registry()
        .with(log_layer)
        .with(rtl)
        .init();

    let cloud_config = if cli.global_opts.connection.cloud_config_from_env {
        // Environment variables should be used to get the cloud configuration
        tracing::debug!("Using environment variables for the cloud connection");
        let cloud_name = cli
            .global_opts
            .connection
            .os_cloud_name
            .clone()
            .unwrap_or(String::from("envvars"));
        let mut cloud_config = openstack_sdk::config::CloudConfig::from_env()?;
        cloud_config.name = Some(cloud_name.clone());
        cloud_config
    } else {
        // prepare cloud config parsing
        let cfg = openstack_sdk::config::ConfigFile::new_with_user_specified_configs(
            cli.global_opts.connection.os_client_config_file.as_deref(),
            cli.global_opts.connection.os_client_secure_file.as_deref(),
        )?;

        // Identify target cloud to connect to
        let cloud_name = match cli.global_opts.connection.os_cloud {
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
                    eyre!("`--os-cloud` or `OS_CLOUD` environment variable must be given, or at least `--cloud-config-from-env` should be used.").into(),
                );
                }
            }
        };
        cfg.get_cloud_config(&cloud_name)?
            .ok_or(OpenStackCliError::ConnectionNotFound(cloud_name.clone()))?
    };
    let mut renew_auth: bool = false;

    // Login command need to be analyzed before authorization
    if let TopLevelCommands::Auth(args) = &cli.command {
        if let auth::AuthCommands::Login(login_args) = &args.command {
            if login_args.renew {
                renew_auth = true;
            }
        }
    }

    // Connect to the selected cloud with the possible AuthHelper
    let mut session =
        if let Some(external_auth_helper) = &cli.global_opts.connection.auth_helper_cmd {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                &mut ExternalCmd::new(external_auth_helper.clone()),
                renew_auth,
            )
            .await
        } else if std::io::stdin().is_terminal() {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                &mut Dialoguer::default(),
                renew_auth,
            )
            .await
        } else {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                &mut Noop::default(),
                renew_auth,
            )
            .await
        }
        .map_err(|err| OpenStackCliError::Auth { source: err })?;

    // Does the user want to connect to different project?
    if cli.global_opts.connection.os_project_id.is_some()
        || cli.global_opts.connection.os_project_name.is_some()
    {
        warn!(
            "Cloud config is being chosen with arguments overriding project. Result may be not as expected."
        );
        let current_auth = session
            .get_auth_info()
            .expect("Already authenticated")
            .token;
        let project = Project {
            id: cli.global_opts.connection.os_project_id.clone(),
            name: cli.global_opts.connection.os_project_name.clone(),
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
    let res = cli.take_action(&mut session).await;

    // If HTTP timing was requested dump stats into STDERR
    if cli.global_opts.output.timing {
        if let Ok(data) = request_stats.lock() {
            let table = build_http_requests_timing_table(&data);
            eprintln!("\nHTTP statistics:");
            eprintln!("{table}");
        }
    }

    res
}

/// Build a table of HTTP request timings
fn build_http_requests_timing_table(data: &HttpRequestStats) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(Vec::from(["Url", "Method", "Duration (ms)"]));

    let mut total_http_duration: u128 = 0;
    for rec in data.summarize_by_url_method() {
        total_http_duration += rec.2;
        table.add_row(vec![rec.0, rec.1, rec.2.to_string()]);
    }
    table.add_row(vec!["Total", "", &total_http_duration.to_string()]);
    table
}
