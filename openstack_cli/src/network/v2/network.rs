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

/// Network commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct NetworkArgs {
    #[command(subcommand)]
    command: NetworkCommands,
}

#[derive(Subcommand, Clone)]
pub enum NetworkCommands {
    List(Box<list::NetworksArgs>),
    Show(Box<show::NetworkArgs>),
    Create(Box<create::NetworkArgs>),
    Delete(delete::NetworkArgs),
}

pub struct NetworkCommand {
    /// Command arguments
    pub args: NetworkArgs,
}

impl OSCCommand for NetworkCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            NetworkCommands::List(args) => Ok(Box::new(list::NetworksCmd {
                args: *args.clone(),
            })),
            NetworkCommands::Show(args) => Ok(Box::new(show::NetworkCmd {
                args: *args.clone(),
            })),
            NetworkCommands::Create(args) => Ok(Box::new(create::NetworkCmd {
                args: *args.clone(),
            })),
            NetworkCommands::Delete(args) => {
                Ok(Box::new(delete::NetworkCmd { args: args.clone() }))
            }
        }
    }
}
