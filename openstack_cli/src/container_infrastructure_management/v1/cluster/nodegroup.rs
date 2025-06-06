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

//! Container infrastructure cluster nodegroup management
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod delete_all;
pub mod list;
//pub mod set;
pub mod show;

/// Manage Nodegroup
///
/// Lists, creates, shows details for, updates, and deletes Nodegroup.
#[derive(Parser)]
pub struct NodegroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: NodegroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NodegroupCommands {
    //Create(Box<create::NodegroupCommand>),
    Delete(Box<delete::NodegroupCommand>),
    List(Box<list::NodegroupsCommand>),
    Purge(Box<delete_all::NodegroupCommand>),
    Show(Box<show::NodegroupCommand>),
    //Set(set::NodegroupCommand),
}

impl NodegroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            //NodegroupCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            NodegroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            NodegroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            NodegroupCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            NodegroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            //NodegroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
