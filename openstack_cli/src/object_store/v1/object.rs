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

mod delete;
mod download;
mod list;
mod show;
mod upload;

/// Object commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ObjectArgs {
    #[command(subcommand)]
    command: ObjectCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ObjectCommands {
    List(list::ObjectsArgs),
    Download(download::ObjectArgs),
    Upload(upload::ObjectArgs),
    Show(show::ObjectArgs),
    Delete(delete::ObjectArgs),
}

pub struct ObjectCommand {
    pub args: ObjectArgs,
}

impl OSCCommand for ObjectCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ObjectCommands::List(args) => Ok(Box::new(list::ObjectsCmd { args: args.clone() })),
            ObjectCommands::Download(args) => {
                Ok(Box::new(download::ObjectCmd { args: args.clone() }))
            }
            ObjectCommands::Upload(args) => Ok(Box::new(upload::ObjectCmd { args: args.clone() })),
            ObjectCommands::Show(args) => Ok(Box::new(show::ObjectCmd { args: args.clone() })),
            ObjectCommands::Delete(args) => Ok(Box::new(delete::ObjectCmd { args: args.clone() })),
        }
    }
}
