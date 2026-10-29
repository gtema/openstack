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

use clap::{CommandFactory, FromArgMatches};
use clap_complete::Generator;
use dialoguer::FuzzySelect;
use std::sync::{Arc, Mutex};
use tracing::warn;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::{Layer, prelude::*};

use openstack_cli_core::cli::{ConnectionRequirementsProvider, MicroVersionStrategyArg};
use openstack_cli_core::error::OpenStackCliError;
use openstack_cli_core::{
    build_http_requests_timing_table,
    tracing_stats::{HttpRequestStats, RequestTracingCollector},
};
use openstack_sdk::config::selector::SelectionError;
use openstack_sdk::{
    AsyncOpenStack, MicroVersionStrategy,
    auth::auth_helper::{Dialoguer, ExternalCmd, Fallback, Noop},
    auth::authtoken::AuthTokenScope,
    types::identity::v3::Project,
};

/// Show the problems of loading the configuration on the standard error: the log level may hide
/// them.
fn print_warnings(warnings: &[String]) {
    for warning in warnings {
        eprintln!("WARNING: {warning}");
    }
}

fn microversion_strategy_from_arg(value: MicroVersionStrategyArg) -> MicroVersionStrategy {
    match value {
        MicroVersionStrategyArg::Floor => MicroVersionStrategy::Floor,
        MicroVersionStrategyArg::Ceiling => MicroVersionStrategy::Ceiling,
    }
}

pub mod cli;
pub mod error;

pub use cli::Cli;
use cli::TopLevelCommands;

/// Entry point for the CLI wrapper
pub async fn entry_point() -> Result<(), OpenStackCliError> {
    let mut cmd = Cli::command();
    let matches = cmd.clone().get_matches();
    let mut cli = Cli::from_arg_matches(&matches).unwrap_or_else(|err| err.exit());
    cli.global_opts
        .connection
        .apply_context_precedence(&matches, &mut cmd);

    if let TopLevelCommands::Completion(args) = &cli.command {
        // generate completion output
        let mut cmd = Cli::command();
        cmd.set_bin_name(cmd.get_name().to_string());
        cmd.build();
        // Ignore any error during writing the completion
        args.shell.try_generate(&cmd, &mut io::stdout()).ok();
        return Ok(());
    }

    if let TopLevelCommands::Plugin(args) = &cli.command {
        // Plugin management never needs a cloud connection.
        return args.take_action(&cli).await;
    }

    if let TopLevelCommands::Config(args) = &cli.command {
        // `osc config` commands edit/read local configuration files and
        // must not trigger cloud authentication.
        return args.take_action(&cli).await;
    }

    if let TopLevelCommands::Auth(args) = &cli.command
        && let Some(cmd) = args.as_offline_cache_clear_all()
    {
        // `auth cache clear --all` is profile-independent and must run
        // before any cloud is resolved (it may be run with no clouds
        // configured at all).
        return cmd.take_action_offline(&cli);
    }

    // Initialize tracing layers
    // fmt for console logging
    let log_layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(
            // Cap noisy dependencies at INFO regardless of verbosity
            Targets::new()
                .with_default(match cli.global_opts.output.verbose {
                    0 => LevelFilter::WARN,
                    1 => LevelFilter::INFO,
                    2 => LevelFilter::DEBUG,
                    _ => LevelFilter::TRACE,
                })
                .with_target("cranelift_codegen", LevelFilter::INFO)
                .with_target("cranelift_frontend", LevelFilter::INFO)
                .with_target("wasmtime", LevelFilter::INFO)
                .with_target("extism", LevelFilter::INFO)
                .with_target("mio", LevelFilter::INFO),
        )
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

    // Select the connection: contexts.yaml, environment, clouds.yaml
    let conn = &cli.global_opts.connection;
    let mut selector = openstack_sdk::config::selector::ConnectionSelector {
        context: conn.os_context.clone(),
        cloud: conn.os_cloud.clone(),
        from_env: conn.cloud_config_from_env,
        env_cloud_name: conn.os_cloud_name.clone(),
        project_override: conn.os_project_id.is_some() || conn.os_project_name.is_some(),
        skip_implicit_context: false,
        region_name: conn.os_region_name.clone(),
        contexts_file: conn.os_context_file.as_ref().map(Into::into),
        config_file: conn.os_client_config_file.as_ref().map(Into::into),
        secure_file: conn.os_client_secure_file.as_ref().map(Into::into),
    };
    let resolved = match selector.resolve() {
        Err(SelectionError::NoSelection {
            clouds,
            contexts,
            warnings,
        }) => {
            print_warnings(&warnings);
            if clouds.is_empty() && contexts.is_empty() {
                return Err(OpenStackCliError::NoConnectionAvailable);
            }
            if !io::stdin().is_terminal() {
                return Err(OpenStackCliError::NoConnectionSelected);
            }
            // Cloud was not selected and we are in the potentially interactive mode (terminal)
            let items: Vec<String> = clouds
                .iter()
                .cloned()
                .chain(contexts.iter().map(|c| format!("{c} (context)")))
                .collect();
            let selected = FuzzySelect::new()
                .with_prompt("Please select the connection you want to use (use `--os-cloud` or `--os-context` next time for efficiency)?")
                .items(&items)
                .interact()?;
            if selected < clouds.len() {
                selector.cloud = Some(clouds[selected].clone());
            } else {
                selector.context = Some(contexts[selected - clouds.len()].clone());
            }
            selector.resolve()?
        }
        other => other?,
    };
    print_warnings(&resolved.warnings);
    let context_auth_helper = resolved.auth_helper();
    let cloud_config = resolved.cloud;

    // Certain commands (e.g. `auth login --renew`, `auth status`) need different
    // pre-connect behavior than the default "connect with a live authenticated
    // session". See `ConnectionRequirementsProvider`.
    let connection_requirements = cli.command.connection_requirements();
    let renew_auth = connection_requirements.renew;

    // Connect to the selected cloud with the possible AuthHelper. Commands that
    // don't need a live/valid session (e.g. `auth status`) use a cache-only,
    // network-free path so they keep working even with an expired token or an
    // unreachable cloud.
    let microversion_strategy =
        microversion_strategy_from_arg(cli.global_opts.connection.os_microversion_strategy);

    let mut session = if !connection_requirements.needs_auth {
        AsyncOpenStack::new_cache_only(&cloud_config)
            .map_err(|err| OpenStackCliError::Auth { source: err })?
    } else {
        if let Some(external_auth_helper) = &cli.global_opts.connection.auth_helper_cmd {
            AsyncOpenStack::builder(&cloud_config)
                .auth_helper(ExternalCmd::new(external_auth_helper.clone()))
                .renew_auth(renew_auth)
                .microversion_strategy(microversion_strategy)
                .connect()
                .await
        } else if let Some(context_helper) = context_auth_helper {
            if std::io::stdin().is_terminal() {
                // What the helper cannot provide (i.e. MFA passcode) is asked interactively
                AsyncOpenStack::builder(&cloud_config)
                    .auth_helper(Fallback::new(context_helper, Dialoguer::default()))
                    .renew_auth(renew_auth)
                    .microversion_strategy(microversion_strategy)
                    .connect()
                    .await
            } else {
                AsyncOpenStack::builder(&cloud_config)
                    .auth_helper(context_helper)
                    .renew_auth(renew_auth)
                    .microversion_strategy(microversion_strategy)
                    .connect()
                    .await
            }
        } else if std::io::stdin().is_terminal() {
            AsyncOpenStack::builder(&cloud_config)
                .auth_helper(Dialoguer::default())
                .renew_auth(renew_auth)
                .microversion_strategy(microversion_strategy)
                .connect()
                .await
        } else {
            AsyncOpenStack::builder(&cloud_config)
                .auth_helper(Noop::default())
                .renew_auth(renew_auth)
                .microversion_strategy(microversion_strategy)
                .connect()
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
