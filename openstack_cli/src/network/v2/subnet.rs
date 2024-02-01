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

/// Subnet commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SubnetArgs {
    #[command(subcommand)]
    command: SubnetCommands,
}

#[derive(Subcommand, Clone)]
pub enum SubnetCommands {
    List(Box<list::SubnetsArgs>),
    Show(Box<show::SubnetArgs>),
    Create(Box<create::SubnetArgs>),
    Delete(delete::SubnetArgs),
}

pub struct SubnetCommand {
    pub args: SubnetArgs,
}

impl OSCCommand for SubnetCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            SubnetCommands::List(args) => Ok(Box::new(list::SubnetsCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Show(args) => Ok(Box::new(show::SubnetCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Create(args) => Ok(Box::new(create::SubnetCmd {
                args: *args.clone(),
            })),
            SubnetCommands::Delete(args) => Ok(Box::new(delete::SubnetCmd { args: args.clone() })),
        }
    }
}
