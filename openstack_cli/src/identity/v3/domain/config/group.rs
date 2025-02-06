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

//! Identity Group configuration

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod default;
pub mod delete;
pub mod option;
pub mod set;
pub mod show;

/// Domain Group group
///
/// Group group is a driver specific configuration for the domain. It contains individual
/// configuration options.
#[derive(Parser)]
pub struct GroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupCommands {
    Default(default::GroupCommand),
    Delete(delete::GroupCommand),
    Option(option::OptionCommand),
    Set(set::GroupCommand),
    Show(show::GroupCommand),
}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupCommands::Default(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Option(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
