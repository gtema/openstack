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
//! TUI Cli parameters
use clap::{ArgMatches, Command, Parser, ValueHint};
use std::path::PathBuf;

use crate::utils::version;

/// TUI Cli parameters
#[derive(Parser, Debug)]
#[command(author, version = version(), about)]
pub struct Cli {
    /// Number of ticks per second. May be used by certain views to trigger certain actions, but
    /// most likely not used.
    #[arg(short, long, value_name = "FLOAT", default_value_t = 0.2)]
    pub tick_rate: f64,

    /// Refresh frame rate (number of frames per second)
    #[arg(short, long, value_name = "FLOAT", default_value_t = 1.0)]
    pub frame_rate: f64,

    /// Cloud name to connect to after the start
    ///
    /// Conflicts with the `--cloud-config-from-env` option.
    #[arg(long, env = "OS_CLOUD", conflicts_with = "cloud_config_from_env")]
    pub os_cloud: Option<String>,

    /// Name of the context from the `contexts.yaml` file to connect to after the start.
    ///
    /// The file is searched in `~/.config/openstack` and `/etc/openstack`. The `clouds.yaml` is
    /// not involved and, same as with `--cloud-config-from-env`, the TUI is limited to this
    /// single connection. Without `--os-context`, `--os-cloud`,
    /// `--cloud-config-from-env` and `--os-cloud-name` the `current-context` of the file is
    /// **not** used by the TUI: the context must be requested explicitly. Conflicts with
    /// `--os-cloud`, `--cloud-config-from-env` and `--os-cloud-name` given on the command line
    /// (the command line wins over the environment variables).
    #[arg(long, env = "OS_CONTEXT", value_name = "CONTEXT")]
    pub os_context: Option<String>,

    /// Custom path to the `contexts.yaml` file.
    ///
    /// Used instead of searching `~/.config/openstack` and `/etc/openstack`. The file must exist.
    /// Only has an effect together with `--os-context`.
    #[arg(long, env = "OS_CONTEXT_FILE", value_hint = ValueHint::FilePath)]
    pub os_context_file: Option<PathBuf>,

    /// Get the cloud config from environment variables.
    ///
    /// Conflicts with the `--os-cloud` option. No merging of environment variables with the
    /// options from the `clouds.yaml` file done. This effectively limits the TUI to a single
    /// cloud connection.
    #[arg(long)]
    pub cloud_config_from_env: bool,

    /// Cloud name used when configuration is retrieved from environment variables. When not
    /// specified the `envvars` would be used as a default.
    #[arg(long, env = "OS_CLOUD_NAME")]
    pub os_cloud_name: Option<String>,

    /// Custom path to the `clouds.yaml` config file
    #[arg(long, env = "OS_CLIENT_CONFIG_FILE", value_hint = ValueHint::FilePath)]
    pub os_client_config_file: Option<PathBuf>,

    /// Custom path to the `secure.yaml` config file
    #[arg(long, env = "OS_CLIENT_SECURE_FILE", value_hint = ValueHint::FilePath)]
    pub os_client_secure_file: Option<PathBuf>,
}

impl Cli {
    /// Apply the precedence between `--os-context` and the options selecting the cloud
    /// connection (command line wins over the environment). Exits on a command line conflict.
    pub fn apply_context_precedence(&mut self, matches: &ArgMatches, cmd: &mut Command) {
        let others = ["os_cloud", "cloud_config_from_env", "os_cloud_name"];
        match openstack_cli_core::cli::context_precedence(matches, "os_context", &others) {
            Ok(ignored) => {
                for id in ignored {
                    match id.as_str() {
                        "os_context" => self.os_context = None,
                        "os_cloud" => self.os_cloud = None,
                        "cloud_config_from_env" => self.cloud_config_from_env = false,
                        "os_cloud_name" => self.os_cloud_name = None,
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
