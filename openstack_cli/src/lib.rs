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

use openstack_cli_core::cli::ConnectionRequirementsProvider;
use openstack_cli_core::error::OpenStackCliError;
use openstack_cli_core::{
    build_http_requests_timing_table,
    tracing_stats::{HttpRequestStats, RequestTracingCollector},
};
use openstack_sdk::{
    AsyncOpenStack,
    auth::auth_helper::{Dialoguer, ExternalCmd, Noop},
    auth::authtoken::AuthTokenScope,
    types::identity::v3::Project,
};

pub mod cli;
pub mod error;

pub use cli::Cli;
use cli::TopLevelCommands;

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

    let mut cloud_config = if cli.global_opts.connection.cloud_config_from_env {
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
    if let Some(region_name) = &cli.global_opts.connection.os_region_name {
        cloud_config.region_name = Some(region_name.clone());
    }

    // Certain commands (e.g. `auth login --renew`, `auth status`) need different
    // pre-connect behavior than the default "connect with a live authenticated
    // session". See `ConnectionRequirementsProvider`.
    let connection_requirements = cli.command.connection_requirements();
    let renew_auth = connection_requirements.renew;

    // Connect to the selected cloud with the possible AuthHelper. Commands that
    // don't need a live/valid session (e.g. `auth status`) use a cache-only,
    // network-free path so they keep working even with an expired token or an
    // unreachable cloud.
    let mut session = if !connection_requirements.needs_auth {
        AsyncOpenStack::new_cache_only(&cloud_config)
            .map_err(|err| OpenStackCliError::Auth { source: err })?
    } else {
        if let Some(external_auth_helper) = &cli.global_opts.connection.auth_helper_cmd {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                ExternalCmd::new(external_auth_helper.clone()),
                renew_auth,
            )
            .await
        } else if std::io::stdin().is_terminal() {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                Dialoguer::default(),
                renew_auth,
            )
            .await
        } else {
            AsyncOpenStack::new_with_authentication_helper(
                &cloud_config,
                Noop::default(),
                renew_auth,
            )
            .await
        }
        .map_err(|err| OpenStackCliError::Auth { source: err })?
    };

    // Does the user want to connect to different project?
    if connection_requirements.needs_auth
        && (cli.global_opts.connection.os_project_id.is_some()
            || cli.global_opts.connection.os_project_name.is_some())
    {
        warn!(
            "Cloud config is being chosen with arguments overriding project. Result may be not as expected."
        );
        let current_auth = session
            .get_auth_info()
            .ok_or(OpenStackCliError::MissingValidAuthenticationForRescope)?
            .token;
        let project = Project {
            id: cli.global_opts.connection.os_project_id.clone(),
            name: cli.global_opts.connection.os_project_name.clone(),
            domain: infer_project_override_domain(
                current_auth.project,
                current_auth.domain,
                current_auth.user.domain,
            ),
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
    if cli.global_opts.output.timing
        && let Ok(data) = request_stats.lock()
    {
        let table = build_http_requests_timing_table(&data);
        eprintln!("\nHTTP statistics:");
        eprintln!("{table}");
    }

    res
}

/// Infer the domain to scope a CLI-provided `--os-project-id`/`--os-project-name`
/// override to, based on the currently authenticated (unscoped/differently-scoped)
/// token.
///
/// Fallback chain (first match wins):
/// 1. Same domain as the currently scoped project, if any.
/// 2. The domain of the current domain-scoped token, if any.
/// 3. The authenticated user's own domain.
fn infer_project_override_domain(
    current_project: Option<openstack_sdk::types::identity::v3::Project>,
    current_domain: Option<openstack_sdk::types::identity::v3::Domain>,
    user_domain: Option<openstack_sdk::types::identity::v3::Domain>,
) -> Option<openstack_sdk::types::identity::v3::Domain> {
    match (current_project, current_domain) {
        // New project is in the same domain as the original
        (Some(project), _) => project.domain,
        // domain scope was used
        (None, Some(domain)) => Some(domain),
        // There was no scope thus using user domain
        _ => user_domain,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openstack_sdk::types::identity::v3::Domain;
    use openstack_sdk::types::identity::v3::Project as AuthProject;

    fn domain(id: &str) -> Domain {
        Domain {
            id: Some(id.to_string()),
            name: None,
        }
    }

    #[test]
    fn test_infer_domain_from_current_project() {
        let project = AuthProject {
            id: Some("p1".into()),
            name: None,
            domain: Some(domain("from-project")),
        };
        let result = infer_project_override_domain(Some(project), Some(domain("from-token")), None);
        assert_eq!(result.unwrap().id.as_deref(), Some("from-project"));
    }

    #[test]
    fn test_infer_domain_from_domain_scope() {
        let result = infer_project_override_domain(None, Some(domain("from-token")), None);
        assert_eq!(result.unwrap().id.as_deref(), Some("from-token"));
    }

    #[test]
    fn test_infer_domain_falls_back_to_user_domain() {
        let result = infer_project_override_domain(None, None, Some(domain("user-domain")));
        assert_eq!(result.unwrap().id.as_deref(), Some("user-domain"));
    }
}
