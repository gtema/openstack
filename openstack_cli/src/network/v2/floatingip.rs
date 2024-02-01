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
mod tag;

/// Floating IP commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct FloatingIPArgs {
    #[command(subcommand)]
    command: FloatingIPCommands,
}

#[derive(Subcommand, Clone)]
pub enum FloatingIPCommands {
    Create(create::FloatingipArgs),
    Delete(delete::FloatingipArgs),
    List(list::FloatingipsArgs),
    Set(set::FloatingipArgs),
    Show(show::FloatingipArgs),
    Tag(tag::TagArgs),
}

pub struct FloatingIPCommand {
    pub args: FloatingIPArgs,
}

impl OSCCommand for FloatingIPCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            FloatingIPCommands::Create(args) => {
                Ok(Box::new(create::FloatingipCmd { args: args.clone() }))
            }
            FloatingIPCommands::Delete(args) => {
                Ok(Box::new(delete::FloatingipCmd { args: args.clone() }))
            }
            FloatingIPCommands::List(args) => {
                Ok(Box::new(list::FloatingipsCmd { args: args.clone() }))
            }
            FloatingIPCommands::Set(args) => {
                Ok(Box::new(set::FloatingipCmd { args: args.clone() }))
            }
            FloatingIPCommands::Show(args) => {
                Ok(Box::new(show::FloatingipCmd { args: args.clone() }))
            }
            FloatingIPCommands::Tag(args) => {
                tag::TagCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
