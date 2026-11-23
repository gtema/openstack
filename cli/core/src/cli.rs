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
    Styles,
    styling::{AnsiColor, Effects},
};
use clap::parser::ValueSource;
use clap::{ArgMatches, Args, Parser, ValueEnum, ValueHint};
use clap_complete::Shell;

use crate::error::OpenStackCliError;

use crate::config::{Config, ConfigError};

pub fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::White.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
}

/// Trait for getting Command execution context.
pub trait CliArgs {
    fn global_opts(&self) -> &GlobalOpts;
    fn config(&self) -> &Config;
}

/// Parse config file
pub fn parse_config(s: &str) -> Result<Config, OpenStackCliError> {
    let mut builder = Config::builder()?;
    if !s.is_empty() {
        builder = builder.add_source(s).map_err(ConfigError::builder)?;
    }
    Ok(builder.build()?)
}

/// Connection options.
#[derive(Args)]
#[command(next_display_order = 800, next_help_heading = "Connection options")]
pub struct ConnectionOpts {
    /// Name reference to the clouds.yaml entry for the cloud configuration.
    #[arg(long, env = "OS_CLOUD", global = true, display_order = 801, conflicts_with_all(["cloud_config_from_env", "os_cloud_name"]))]
    pub os_cloud: Option<String>,

    /// Name of the context from the `contexts.yaml` file.
    ///
    /// The file is searched in `~/.config/openstack` and `/etc/openstack`. A context combines a
    /// server, a user and a scope (and optionally a region). The `clouds.yaml` and `secure.yaml`
    /// are not involved. Secrets missing in the file are requested with the user `auth_helper` of
    /// the context unless `--auth-helper-cmd` is specified.
    ///
    /// Without `--os-context` and any option selecting or overriding the
    /// cloud connection (`--os-cloud`, `--cloud-config-from-env`, `--os-cloud-name`,
    /// `--os-project-id`, `--os-project-name`, `--os-region-name`) the `current-context` of the
    /// `contexts.yaml` is used when the file exists and sets it.
    ///
    /// Those options conflict with `--os-context` when both are given on the command line (the
    /// region of a context is set by its `region_name`). When the conflicting value comes from an
    /// environment variable (i.e. `OS_CLOUD` of an openrc file) the command line wins: the context
    /// is ignored when a conflicting option is given on the command line, and the conflicting
    /// environment variables are ignored when `--os-context` is.
    #[arg(
        long,
        env = "OS_CONTEXT",
        global = true,
        display_order = 801,
        value_name = "CONTEXT"
    )]
    pub os_context: Option<String>,

    /// Get the cloud config from environment variables.
    ///
    /// Conflicts with the `--os-cloud` option. No merging of environment variables with the
    /// options from the `clouds.yaml` file done. It is possible to rely on the `--auth-helper-cmd`
    /// command, but than the `--os-cloud-name` should be specified to give a reasonable connection
    /// name.
    #[arg(long, global = true, action = clap::ArgAction::SetTrue, display_order = 802)]
    pub cloud_config_from_env: bool,

    /// Cloud name used when configuration is retrieved from environment variables. When not
    /// specified the `envvars` would be used as a default. This value will be used eventually by
    /// the authentication helper when data need to be provided dynamically.
    #[arg(long, env = "OS_CLOUD_NAME", global = true, display_order = 802)]
    pub os_cloud_name: Option<String>,

    /// Project ID to use instead of the one in connection profile.
    #[arg(long, env = "OS_PROJECT_ID", global = true, display_order = 803)]
    pub os_project_id: Option<String>,

    /// Project Name to use instead of the one in the connection profile.
    #[arg(long, env = "OS_PROJECT_NAME", global = true, display_order = 803)]
    pub os_project_name: Option<String>,

    /// Region Name to use instead of the one in the connection profile.
    #[arg(long, env = "OS_REGION_NAME", global = true, display_order = 804)]
    pub os_region_name: Option<String>,

    /// Custom path to the `contexts.yaml` file.
    ///
    /// Used instead of searching `~/.config/openstack` and `/etc/openstack` (same as `--kubeconfig`
    /// of `kubectl`). The file must exist. Does not select the connection by itself: the context
    /// is selected with `--os-context` or by the `current-context` of the file.
    #[arg(
        long,
        env = "OS_CONTEXT_FILE",
        global = true,
        value_hint = ValueHint::FilePath,
        display_order = 801
    )]
    pub os_context_file: Option<String>,

    /// Custom path to the `clouds.yaml` config file.
    #[arg(
        long,
        env = "OS_CLIENT_CONFIG_FILE",
        global = true,
        value_hint = ValueHint::FilePath,
        display_order = 805
    )]
    pub os_client_config_file: Option<String>,

    /// Custom path to the `secure.yaml` config file.
    #[arg(
        long,
        env = "OS_CLIENT_SECURE_FILE",
        global = true,
        value_hint = ValueHint::FilePath,
        display_order = 805
    )]
    pub os_client_secure_file: Option<String>,

    /// External authentication helper command.
    ///
    /// Invoke external command to obtain necessary connection parameters. This is a path to the
    /// executable, which is called with first parameter being the attribute key (i.e. `password`)
    /// and a second parameter a cloud name (whatever is used in `--os-cloud` or
    /// `--os-cloud-name`).
    #[arg(long, global = true, value_hint = ValueHint::ExecutablePath, display_order = 810)]
    pub auth_helper_cmd: Option<String>,

    /// Microversion negotiation strategy used for every service.
    ///
    /// `floor` (default) requests the lowest microversion compatible with both the command and
    /// the cloud, matching this tool's historical behavior. `ceiling` requests the highest
    /// compatible microversion instead, which typically yields more complete data in responses
    /// (e.g. commands that only started returning a field in a later microversion).
    #[arg(
        long,
        env = "OS_MICROVERSION_STRATEGY",
        global = true,
        value_enum,
        default_value_t = MicroVersionStrategyArg::Floor,
        display_order = 806
    )]
    pub os_microversion_strategy: MicroVersionStrategyArg,
}

/// Microversion negotiation strategy (CLI-facing mirror of
/// `openstack_sdk::MicroVersionStrategy`, kept separate so `cli/core` does not need a hard
/// dependency on the full SDK crate for a single enum).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum MicroVersionStrategyArg {
    /// Request the lowest microversion compatible with both the command and the cloud.
    #[default]
    Floor,
    /// Request the highest microversion compatible with both the command and the cloud.
    Ceiling,
}

/// Output configuration.
#[derive(Args)]
#[command(next_display_order = 900, next_help_heading = "Output options")]
pub struct OutputOpts {
    /// Output format.
    #[arg(short, long, global = true, value_enum, display_order = 910)]
    pub output: Option<OutputFormat>,

    /// Fields to return in the output (only in normal and wide mode).
    #[arg(short, long, global=true, action=clap::ArgAction::Append, display_order = 910)]
    pub fields: Vec<String>,

    /// Pretty print the output.
    #[arg(short, long, global=true, action = clap::ArgAction::SetTrue, display_order = 910)]
    pub pretty: bool,

    /// Verbosity level. Repeat to increase level.
    #[arg(short, long, global=true, action = clap::ArgAction::Count, display_order = 920)]
    pub verbose: u8,

    /// Output Table arrangement.
    #[arg(long, global=true, default_value_t = TableArrangement::Dynamic, value_enum, display_order = 930)]
    pub table_arrangement: TableArrangement,

    /// Record HTTP request timings.
    #[arg(long, global=true, action = clap::ArgAction::SetTrue, display_order = 950)]
    pub timing: bool,
}

/// Global CLI options.
#[derive(Args)]
#[command(next_display_order = 900)]
pub struct GlobalOpts {
    /// Connection options.
    #[command(flatten)]
    pub connection: ConnectionOpts,

    /// Output config.
    #[command(flatten)]
    pub output: OutputOpts,
}

impl ConnectionOpts {
    /// Apply the precedence between `--os-context` and the options selecting or overriding the
    /// cloud connection (see [`context_precedence`]). Exits with the usage error on a conflict
    /// between two command line options.
    pub fn apply_context_precedence(&mut self, matches: &ArgMatches, cmd: &mut clap::Command) {
        let others = [
            "os_cloud",
            "cloud_config_from_env",
            "os_cloud_name",
            "os_project_id",
            "os_project_name",
            "os_region_name",
        ];
        match context_precedence(matches, "os_context", &others) {
            Ok(ignored) => {
                for id in ignored {
                    match id.as_str() {
                        "os_context" => self.os_context = None,
                        "os_cloud" => self.os_cloud = None,
                        "cloud_config_from_env" => self.cloud_config_from_env = false,
                        "os_cloud_name" => self.os_cloud_name = None,
                        "os_project_id" => self.os_project_id = None,
                        "os_project_name" => self.os_project_name = None,
                        "os_region_name" => self.os_region_name = None,
                        _ => {}
                    }
                }
            }
            Err(msg) => cmd
                .error(clap::error::ErrorKind::ArgumentConflict, msg)
                .exit(),
        }
    }
}

/// Highest precedence source (command line over environment) of the argument in the matches of
/// the command or any of its subcommands (global arguments may be given at any level).
fn arg_source(matches: &ArgMatches, id: &str) -> Option<ValueSource> {
    let own = if matches.try_contains_id(id).unwrap_or(false) {
        matches
            .value_source(id)
            .filter(|s| matches!(s, ValueSource::CommandLine | ValueSource::EnvVariable))
    } else {
        None
    };
    let sub = matches
        .subcommand()
        .and_then(|(_, sub)| arg_source(sub, id));
    match (own, sub) {
        (Some(ValueSource::CommandLine), _) | (_, Some(ValueSource::CommandLine)) => {
            Some(ValueSource::CommandLine)
        }
        (own, sub) => own.or(sub),
    }
}

/// Decide between the explicit context and the arguments selecting or overriding the cloud.
///
/// Values given on the command line win over the ones coming from the environment variables
/// (an openrc file exporting `OS_CLOUD` must not prevent `--os-context`). Two values given on the
/// command line conflict. Returns ids of the arguments which must be ignored, or the error text.
pub fn context_precedence(
    matches: &ArgMatches,
    context: &str,
    others: &[&str],
) -> Result<Vec<String>, String> {
    let ids = |source: ValueSource| -> Vec<&str> {
        others
            .iter()
            .copied()
            .filter(|id| arg_source(matches, id) == Some(source))
            .collect()
    };
    let (cli, env) = (ids(ValueSource::CommandLine), ids(ValueSource::EnvVariable));
    let ignore = |ids: Vec<&str>| ids.into_iter().map(String::from).collect::<Vec<_>>();
    match arg_source(matches, context) {
        Some(ValueSource::CommandLine) => match cli.first() {
            Some(other) => Err(format!(
                "the argument '--{}' cannot be used with '--{}'{}",
                context.replace('_', "-"),
                other.replace('_', "-"),
                if *other == "os_region_name" {
                    " (the region is defined by the `region_name` of the context)"
                } else {
                    ""
                }
            )),
            None => Ok(ignore(env)),
        },
        // Everything else wins over the context coming from the environment
        Some(_) if !cli.is_empty() || !env.is_empty() => {
            eprintln!(
                "WARNING: ignoring `--{}` (coming from the environment) since '--{}' is set",
                context.replace('_', "-"),
                cli.iter()
                    .chain(env.iter())
                    .next()
                    .map(|id| id.replace('_', "-"))
                    .unwrap_or_default()
            );
            Ok(vec![context.to_string()])
        }
        _ => Ok(Vec::new()),
    }
}

/// Resolve a per-service API version selector by scanning the real process
/// arguments directly. This has to happen before clap builds the `Command`
/// tree (inside `Args::augment_args`), i.e. before any `ArgMatches` exist —
/// so it can't go through clap itself. Once the tree is built, the actual
/// (clap-parsed, env-aware, default-applying) value is read normally from
/// `ArgMatches` in `FromArgMatches::from_arg_matches`; this function is only
/// ever used to decide *which* version's subcommand tree to attach.
pub fn resolve_api_version(long_flag: &str, env_var: &str, default: &str) -> String {
    let needle_eq = format!("--{long_flag}=");
    let needle = format!("--{long_flag}");
    let mut result = None;
    let mut args = std::env::args();
    while let Some(a) = args.next() {
        if let Some(v) = a.strip_prefix(&needle_eq) {
            result = Some(v.to_string());
        } else if a == needle
            && let Some(v) = args.next()
        {
            result = Some(v);
        }
    }
    result
        .or_else(|| std::env::var(env_var).ok())
        .unwrap_or_else(|| default.to_string())
}

/// Output format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    /// Json output.
    Json,
    /// Wide (Human readable table with extra attributes). Note: this has
    /// effect only in list operations.
    Wide,
}

/// Table arrangement.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TableArrangement {
    /// Dynamically determine the width of columns in regard to terminal width and content length.
    /// With this mode, the content in cells will wrap dynamically to get the the best column
    /// layout for the given content.
    #[default]
    Dynamic,
    /// This is mode is the same as the `Dynamic` arrangement, but it will always use as much space
    /// as it’s given. Any surplus space will be distributed between all columns.
    DynamicFullWidth,
    /// Don’t do any content arrangement. Tables with this mode might become wider than your output
    /// and look ugly.
    Disabled,
}

/// Output shell completion code for the specified shell (bash, zsh, fish, or powershell). The
/// shell code must be evaluated to provide interactive completion of `osc` commands.  This can
/// be done by sourcing it from the .bash_profile.
///
/// Examples:
///
///  Enable completion at a shell start:
///
/// `echo 'source <(osc completion bash)' >>~/.bashrc`
///
#[derive(Parser, Debug)]
pub struct CompletionCommand {
    /// If provided, outputs the completion file for given shell.
    #[arg(default_value_t = Shell::Bash)]
    pub shell: Shell,
}

/// Pre-connect behavior a parsed command wants from the CLI entry point.
///
/// Centralizes the "does this command need a live authenticated connection,
/// and should the token be force-renewed" decision so the entry point does
/// not need to hand-inspect the parsed command tree for every case (see
/// `osc auth login --renew` and `osc auth status`).
#[derive(Debug, Clone, Copy, Default)]
pub struct ConnectionRequirements {
    /// Whether the command requires a live, authenticated connection to the cloud.
    ///
    /// Commands that only read local/cached state (e.g. `osc auth status`) can set
    /// this to `false`.
    pub needs_auth: bool,
    /// Whether the auth token should be force-renewed before the command runs.
    pub renew: bool,
}

impl ConnectionRequirements {
    /// Default requirements for a normal, connected command: needs a live
    /// authenticated session, no forced renewal.
    pub fn connected() -> Self {
        Self {
            needs_auth: true,
            renew: false,
        }
    }
}

/// Provides pre-connect [`ConnectionRequirements`] for a parsed command.
///
/// This is implemented by hand for the top-level `Cli`/`TopLevelCommands` dispatch
/// and the `auth` subcommand (both hand-written, not codegenerated — see
/// `grep -r TopLevelCommands`). Should more commands need pre-connect behavior
/// (e.g. a hypothetical purely-local `osc api-version`), a template-level
/// `connection_requirements()` hook in the code generator would be the long-term
/// fix; for now the mechanism is only wired where it is needed.
pub trait ConnectionRequirementsProvider {
    /// Return the connection requirements for this parsed command.
    fn connection_requirements(&self) -> ConnectionRequirements {
        ConnectionRequirements::connected()
    }
}

#[cfg(test)]
mod context_precedence_tests {
    use clap::Command;

    use super::*;

    fn matches(args: &[&str]) -> ArgMatches {
        #[derive(Parser)]
        struct Wrapper {
            #[command(flatten)]
            conn: ConnectionOpts,
            #[command(subcommand)]
            cmd: Sub,
        }
        #[derive(clap::Subcommand)]
        enum Sub {
            Run,
        }
        let cmd: Command = <Wrapper as clap::CommandFactory>::command();
        cmd.try_get_matches_from(["t"].into_iter().chain(args.iter().copied()))
            .unwrap()
    }

    const OTHERS: [&str; 2] = ["os_cloud", "os_region_name"];

    #[test]
    fn command_line_conflict() {
        let m = matches(&["--os-context", "c", "--os-cloud", "x", "run"]);
        assert!(context_precedence(&m, "os_context", &OTHERS).is_err());
        // global option after the subcommand
        let m = matches(&["run", "--os-context", "c", "--os-region-name", "r"]);
        assert!(context_precedence(&m, "os_context", &OTHERS).is_err());
    }

    /// Command with the arguments reading the (test specific) environment variables.
    fn env_command() -> Command {
        use clap::Arg;
        let arg = |id: &'static str, long: &'static str, env: &'static str| {
            Arg::new(id).long(long).env(env).global(true)
        };
        Command::new("t")
            .arg(arg(
                "os_context",
                "os-context",
                "CTX_PRECEDENCE_TEST_CONTEXT",
            ))
            .arg(arg("os_cloud", "os-cloud", "CTX_PRECEDENCE_TEST_CLOUD"))
            .arg(arg(
                "os_region_name",
                "os-region-name",
                "CTX_PRECEDENCE_TEST_REGION",
            ))
            .subcommand(Command::new("run"))
    }

    // Environment variables are process wide: a single test (with its own variables) mutates them
    #[test]
    fn environment_versus_command_line() {
        let parse = |args: &[&str]| {
            env_command()
                .try_get_matches_from(["t"].into_iter().chain(args.iter().copied()))
                .unwrap()
        };
        let check = |args: &[&str]| context_precedence(&parse(args), "os_context", &OTHERS);
        // SAFETY: the variables are used by this test only
        unsafe {
            std::env::set_var("CTX_PRECEDENCE_TEST_CLOUD", "env-cloud");
            std::env::set_var("CTX_PRECEDENCE_TEST_REGION", "env-region");
        }
        // command line context wins over all env values
        assert_eq!(
            check(&["--os-context", "c", "run"]).unwrap(),
            ["os_cloud", "os_region_name"]
        );
        // ... but conflicts with the command line ones
        assert!(check(&["--os-context", "c", "--os-cloud", "x", "run"]).is_err());
        // command line cloud wins over env context
        unsafe {
            std::env::set_var("CTX_PRECEDENCE_TEST_CONTEXT", "env-ctx");
        }
        assert_eq!(check(&["--os-cloud", "x", "run"]).unwrap(), ["os_context"]);
        // env against env: the context loses
        assert_eq!(check(&["run"]).unwrap(), ["os_context"]);
        unsafe {
            std::env::remove_var("CTX_PRECEDENCE_TEST_CLOUD");
            std::env::remove_var("CTX_PRECEDENCE_TEST_REGION");
        }
        // only the env context
        assert!(check(&["run"]).unwrap().is_empty());
        unsafe {
            std::env::remove_var("CTX_PRECEDENCE_TEST_CONTEXT");
        }
    }

    #[test]
    fn region_conflict_hints_at_the_context() {
        let m = matches(&["--os-context", "c", "--os-region-name", "r", "run"]);
        let err = context_precedence(&m, "os_context", &OTHERS).unwrap_err();
        assert!(err.contains("region_name"), "{err}");
    }

    #[test]
    fn no_conflict() {
        let m = matches(&["--os-context", "c", "run"]);
        assert!(
            context_precedence(&m, "os_context", &OTHERS)
                .unwrap()
                .is_empty()
        );
        let m = matches(&["--os-cloud", "x", "run"]);
        assert!(
            context_precedence(&m, "os_context", &OTHERS)
                .unwrap()
                .is_empty()
        );
    }
}
