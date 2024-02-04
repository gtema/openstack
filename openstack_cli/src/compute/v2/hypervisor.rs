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

//! Hypervisor commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod list;
mod show;

/// Lists all hypervisors, shows summary statistics for all hypervisors over
/// all compute nodes, shows details for a hypervisor, shows the uptime for a
/// hypervisor, lists all servers on hypervisors that match the given
/// hypervisor_hostname_pattern or searches for hypervisors by the given
/// hypervisor_hostname_pattern.
#[derive(Parser)]
#[command(about = "Hypervisors")]
pub struct HypervisorCommand {
    /// subcommand
    #[command(subcommand)]
    command: HypervisorCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum HypervisorCommands {
    List(list::HypervisorsCommand),
    Show(show::HypervisorCommand),
}

impl HypervisorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            HypervisorCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            HypervisorCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
