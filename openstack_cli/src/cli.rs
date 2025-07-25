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
use clap::{Args, Parser, ValueEnum, ValueHint};
use clap_complete::Shell;

use openstack_sdk::AsyncOpenStack;

use crate::error::OpenStackCliError;

use crate::api;
use crate::auth;
use crate::block_storage::v3 as block_storage;
use crate::catalog;
use crate::compute::v2 as compute;
use crate::config::{Config, ConfigError};
use crate::container_infrastructure_management::v1 as container_infra;
use crate::dns::v2 as dns;
use crate::identity::v3 as identity;
use crate::image::v2 as image;
use crate::load_balancer::v2 as load_balancer;
use crate::network::v2 as network;
use crate::object_store::v1 as object_store;
use crate::placement::v1 as placement;

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::White.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Cyan.on_default())
}

/// OpenStack command line interface.
///
/// ## Configuration
///
/// As all OpenStack tools it fully supports `clouds.yaml`.
///
/// Certain aspects of the CLI itself (not the cloud connection) can be configured using
/// `$XDG_CONFIG_HOME/osc/config.yaml` file. With it it is possible, for example, to configure
/// which resource fields are returned when no other output controlling parameters has be passed.
///
/// Example:
///
/// ```yaml
/// views:
///   compute.server:
///     # Listing compute servers will only return ID, NAME and IMAGE columns unless `-o wide` or
///     `-f XXX` parameters are being passed
///     fields: [id, name, image]
///   dns.zone/recordset:
///     # DNS zone recordsets are listed in the wide mode by default.
///     wide: true
/// ```
///
/// ## Features
///
///  * `osc api` as an API wrapper allowing user to perform any direct API call specifying service
///    type, url, method and payload. This can be used for example when certain resource is not
///    currently implemented natively.
///
///  * `osc auth` with subcommands for dealing explicitly with authentication (showing current auth
///    info, renewing auth, MFA/SSO support)
///
///  * Every resource is having a service type in the command solving confusions like user groups
///    vs volume groups
///
///  * Every multi-word resource name is "-" separated (i.e. floating-ip, access-rule)
///
/// ## Output
///
///  * `osc ... -o json` as an explicit machine readable format output. It allows seeing raw
///    resource json representation as send by the API without any processing on the client side.
///
///    **Note:** the result is not the raw json response, but the raw json resource information found
///    underneath expected resource key. This mode can be used i.e. to see fields that are not
///    expected by the `osc` and allows further easy machine processing with tools like `jq`
///
///  * `osc ... -o wide` for list operations to return all known fields. By default list operation
///    will only return a subset of known generic resource fields to prevent multiline tables. This
///    mode (together with not specifying `-o` parameter at all) is considered as an output for
///    humans. Field names are not generally renamed and are names as the API returns them.
///
/// ## Shell autocompletion
///
/// `osc` supports generation of the completion file for diverse shells. This can be enabled i.e.
/// by executing
///
/// ```bash
/// echo 'source <(osc completion bash)' >>~/.bashrc
/// ```
#[derive(Parser)]
#[command(name="osc", author, version, about, long_about, styles = styles())]
#[command(propagate_version = true)]
pub struct Cli {
    /// Global cli arguments and options
    #[command(flatten)]
    pub global_opts: GlobalOpts,

    /// subcommand
    #[command(subcommand)]
    pub command: TopLevelCommands,

    /// CLI configuration
    ///
    /// This does not accept parameters at the moment and will always get config from default
    /// location.
    #[arg(hide = true, long("cli-config"), value_parser = parse_config, default_value_t = Config::new().expect("invalid config"))]
    pub config: Config,
}

/// Parse config file
pub fn parse_config(s: &str) -> Result<Config, OpenStackCliError> {
    let mut builder = Config::builder();
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

/// Supported Top Level commands (services).
#[allow(missing_docs)]
#[derive(Parser)]
pub enum TopLevelCommands {
    Api(api::ApiCommand),
    Auth(auth::AuthCommand),
    BlockStorage(block_storage::BlockStorageCommand),
    Catalog(catalog::CatalogCommand),
    Compute(compute::ComputeCommand),
    #[command(aliases = ["container-infrastructure-management", "container"])]
    ContainerInfrastructure(container_infra::ContainerInfrastructureCommand),
    Dns(dns::DnsCommand),
    Identity(identity::IdentityCommand),
    Image(image::ImageCommand),
    LoadBalancer(load_balancer::LoadBalancerCommand),
    Network(network::NetworkCommand),
    ObjectStore(object_store::ObjectStoreCommand),
    Placement(placement::PlacementCommand),
    Completion(CompletionCommand),
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

impl Cli {
    /// Perform command action.
    pub async fn take_action(&self, client: &mut AsyncOpenStack) -> Result<(), OpenStackCliError> {
        match &self.command {
            TopLevelCommands::Api(args) => args.take_action(self, client).await,
            TopLevelCommands::Auth(args) => args.take_action(self, client).await,
            TopLevelCommands::BlockStorage(args) => args.take_action(self, client).await,
            TopLevelCommands::Catalog(args) => args.take_action(self, client).await,
            TopLevelCommands::Compute(args) => args.take_action(self, client).await,
            TopLevelCommands::ContainerInfrastructure(args) => args.take_action(self, client).await,
            TopLevelCommands::Dns(args) => args.take_action(self, client).await,
            TopLevelCommands::Identity(args) => args.take_action(self, client).await,
            TopLevelCommands::Image(args) => args.take_action(self, client).await,
            TopLevelCommands::LoadBalancer(args) => args.take_action(self, client).await,
            TopLevelCommands::Network(args) => args.take_action(self, client).await,
            TopLevelCommands::ObjectStore(args) => args.take_action(self, client).await,
            TopLevelCommands::Placement(args) => args.take_action(self, client).await,
            TopLevelCommands::Completion(_) => unimplemented!(),
        }
    }
}
