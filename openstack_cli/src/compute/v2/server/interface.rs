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

//! Server interfaces

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create_249;
mod delete;
mod list;
mod show;

/// Port interfaces (servers, os-interface)
///
/// List port interfaces, show port interface details of the given server.
/// Create a port interface and uses it to attach a port to the given server,
/// detach a port interface from the given server.
#[derive(Parser)]
pub struct InterfaceCommand {
    /// subcommand
    #[command(subcommand)]
    command: InterfaceCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum InterfaceCommands {
    #[command(visible_alias = "create")]
    Create249(create_249::InterfaceCommand),
    Delete(delete::InterfaceCommand),
    List(list::InterfacesCommand),
    Show(show::InterfaceCommand),
}

impl InterfaceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            InterfaceCommands::Create249(cmd) => cmd.take_action(parsed_args, session).await,
            InterfaceCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            InterfaceCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            InterfaceCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
