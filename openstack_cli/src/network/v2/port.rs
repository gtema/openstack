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

mod create;
mod delete;
mod list;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct PortArgs {
    #[command(subcommand)]
    command: PortCommands,
}

#[derive(Subcommand, Clone)]
pub enum PortCommands {
    /// List Ports
    List(Box<list::PortsArgs>),
    /// Show single Port
    Show(Box<show::PortArgs>),
    /// Create single Port
    Create(Box<create::PortArgs>),
    /// Delete single Port
    Delete(delete::PortArgs),
}

pub struct PortCommand {
    pub args: PortArgs,
}

impl OSCCommand for PortCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            PortCommands::List(args) => Ok(Box::new(list::PortsCmd {
                args: *args.clone(),
            })),
            PortCommands::Show(args) => Ok(Box::new(show::PortCmd {
                args: *args.clone(),
            })),
            PortCommands::Create(args) => Ok(Box::new(create::PortCmd {
                args: *args.clone(),
            })),
            PortCommands::Delete(args) => Ok(Box::new(delete::PortCmd { args: args.clone() })),
        }
    }
}
