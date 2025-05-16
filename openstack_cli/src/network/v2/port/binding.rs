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

//! Binding binding commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod activate;
pub mod create;
pub mod delete;
pub mod list;
//pub mod set;
//pub mod show;

/// Binding commands
#[derive(Parser)]
pub struct BindingCommand {
    /// subcommand
    #[command(subcommand)]
    command: BindingCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum BindingCommands {
    Activate(Box<activate::BindingCommand>),
    Create(Box<create::BindingCommand>),
    Delete(delete::BindingCommand),
    List(Box<list::BindingsCommand>),
    // Set(Box<set::BindingCommand>),
    // Show(Box<show::BindingCommand>),
}

impl BindingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            BindingCommands::Activate(cmd) => cmd.take_action(parsed_args, session).await,
            BindingCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            BindingCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            BindingCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            // BindingCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            // BindingCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
