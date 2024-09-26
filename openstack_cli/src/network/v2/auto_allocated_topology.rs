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

//! AutoAllocatedTopology resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Auto Allocated Topologies
///
/// Show details and delete the auto allocated topology for a given project. This API is only
/// available when the auto-allocated-topology extension is enabled.
#[derive(Parser)]
pub struct AutoAllocatedTopologyCommand {
    /// subcommand
    #[command(subcommand)]
    command: AutoAllocatedTopologyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AutoAllocatedTopologyCommands {
    Create(Box<create::AutoAllocatedTopologyCommand>),
    Delete(Box<delete::AutoAllocatedTopologyCommand>),
    List(Box<list::AutoAllocatedTopologiesCommand>),
    Set(Box<set::AutoAllocatedTopologyCommand>),
    Show(Box<show::AutoAllocatedTopologyCommand>),
}

impl AutoAllocatedTopologyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AutoAllocatedTopologyCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AutoAllocatedTopologyCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            AutoAllocatedTopologyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AutoAllocatedTopologyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            AutoAllocatedTopologyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
