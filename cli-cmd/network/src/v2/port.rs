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

//! Port commands

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;

pub mod binding;
pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;
pub mod tag;

/// Port commands
#[derive(Parser)]
pub struct PortCommand {
    /// subcommand
    #[command(subcommand)]
    command: PortCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PortCommands {
    Binding(Box<binding::BindingCommand>),
    Create(Box<create::PortCommand>),
    Delete(delete::PortCommand),
    List(Box<list::PortsCommand>),
    Set(Box<set::PortCommand>),
    Show(Box<show::PortCommand>),
    Tag(Box<tag::TagCommand>),
}

impl PortCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            PortCommands::Binding(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            PortCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
