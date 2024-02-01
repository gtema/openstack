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

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod list;
pub mod show;

/// Lists all hypervisors, shows summary statistics for all
/// hypervisors over all compute nodes, shows details for a
/// hypervisor, shows the uptime for a hypervisor, lists all
/// servers on hypervisors that match the given
/// hypervisor_hostname_pattern or searches for hypervisors by the
/// given hypervisor_hostname_pattern.
#[derive(Args, Clone, Debug)]
#[command(about = "Hypervisors")]
// #[command(args_conflicts_with_subcommands = true)]
pub struct HypervisorArgs {
    #[command(subcommand)]
    command: HypervisorCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum HypervisorCommands {
    List(list::HypervisorsArgs),
    Show(show::HypervisorArgs),
}

pub struct HypervisorCommand {
    /// Command arguments
    pub args: HypervisorArgs,
}

impl OSCCommand for HypervisorCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            HypervisorCommands::List(args) => {
                Ok(Box::new(list::HypervisorsCmd { args: args.clone() }))
            }
            HypervisorCommands::Show(args) => {
                Ok(Box::new(show::HypervisorCmd { args: args.clone() }))
            }
        }
    }
}
