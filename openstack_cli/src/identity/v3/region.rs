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

//! Identity Region commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Region commands
///
/// A region is a general division of an OpenStack deployment. You can associate zero or more
/// sub-regions with a region to create a tree- like structured hierarchy.
///
/// Although a region does not have a geographical connotation, a deployment can use a geographical
/// name for a region ID, such as us- east.
///
/// You can list, create, update, show details for, and delete regions.
#[derive(Parser)]
pub struct RegionCommand {
    #[command(subcommand)]
    command: RegionCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RegionCommands {
    Create(create::RegionCommand),
    Delete(delete::RegionCommand),
    List(list::RegionsCommand),
    Set(set::RegionCommand),
    Show(show::RegionCommand),
}

impl RegionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RegionCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            RegionCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            RegionCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RegionCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            RegionCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
