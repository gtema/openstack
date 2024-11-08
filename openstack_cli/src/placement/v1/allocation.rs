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

//! Placement `allocation` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

//mod create_113;
//mod create_128;
//mod create_134;
//mod create_138;
mod delete;
mod set_10;
//mod set_112;
//mod set_128;
//mod set_138;
mod show;

/// Allocations
///
/// Allocations are records representing resources that have been assigned and used by some
/// consumer of that resource. They indicate the amount of a particular resource that has been
/// allocated to a given consumer of that resource from a particular resource provider.
#[derive(Parser)]
pub struct AllocationCommand {
    #[command(subcommand)]
    command: AllocationCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AllocationCommands {
    //    #[command(visible_alias = "create")]
    //    Create138(create_138::AllocationCommand),
    //    Create134(create_134::AllocationCommand),
    //    Create128(create_128::AllocationCommand),
    //    Create113(create_113::AllocationCommand),
    Delete(delete::AllocationCommand),
    //    #[command(visible_alias = "set")]
    //    Set138(set_138::AllocationCommand),
    //    Set128(set_128::AllocationCommand),
    //    Set112(set_112::AllocationCommand),
    Set10(set_10::AllocationCommand),
    Show(show::AllocationCommand),
}

impl AllocationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            //            AllocationCommands::Create138(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Create134(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Create128(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Create113(cmd) => cmd.take_action(parsed_args, session).await,
            AllocationCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Set138(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Set128(cmd) => cmd.take_action(parsed_args, session).await,
            //            AllocationCommands::Set112(cmd) => cmd.take_action(parsed_args, session).await,
            AllocationCommands::Set10(cmd) => cmd.take_action(parsed_args, session).await,
            AllocationCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
