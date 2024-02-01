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
mod set;
mod show;

#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ContainerArgs {
    #[command(subcommand)]
    command: ContainerCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ContainerCommands {
    List(list::ContainersArgs),
    Show(show::ContainerArgs),
    Set(set::ContainerArgs),
    Create(create::ContainerArgs),
    Delete(delete::ContainerArgs),
}

pub struct ContainerCommand {
    pub args: ContainerArgs,
}

impl OSCCommand for ContainerCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ContainerCommands::List(args) => {
                Ok(Box::new(list::ContainersCmd { args: args.clone() }))
            }
            ContainerCommands::Set(args) => Ok(Box::new(set::ContainerCmd { args: args.clone() })),
            ContainerCommands::Show(args) => {
                Ok(Box::new(show::ContainerCmd { args: args.clone() }))
            }
            ContainerCommands::Create(args) => {
                Ok(Box::new(create::ContainerCmd { args: args.clone() }))
            }
            ContainerCommands::Delete(args) => {
                Ok(Box::new(delete::ContainerCmd { args: args.clone() }))
            }
        }
    }
}
