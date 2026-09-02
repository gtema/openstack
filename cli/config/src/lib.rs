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

//! Local client configuration file operations

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk_core::config::CloudConfig;

pub mod clouds;

/// Manage local OpenStack client configuration files
///
/// This command provides operations editing the local `clouds.yaml` and
/// `secure.yaml` files. The commands operate on local files only and never
/// authenticate against the cloud.
#[derive(Parser)]
pub struct ConfigCommand {
    /// Configuration commands
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ConfigCommands {
    Clouds(clouds::CloudsCommand),
}

impl ConfigCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        cloud_config: &CloudConfig,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ConfigCommands::Clouds(cmd) => cmd.take_action(parsed_args, cloud_config),
        }
    }
}
