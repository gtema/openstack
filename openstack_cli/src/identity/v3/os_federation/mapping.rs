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

//! Identity Federation Mapping commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Mappings
///
/// A mapping is a set of rules to map federation protocol attributes to
/// Identity API objects. An Identity Provider can have a single mapping
/// specified per protocol. A mapping is simply a list of rules.
#[derive(Parser)]
pub struct MappingCommand {
    #[command(subcommand)]
    command: MappingCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MappingCommands {
    Create(create::MappingCommand),
    Delete(delete::MappingCommand),
    List(list::MappingsCommand),
    Set(set::MappingCommand),
    Show(show::MappingCommand),
}

impl MappingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MappingCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            MappingCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            MappingCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            MappingCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            MappingCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
