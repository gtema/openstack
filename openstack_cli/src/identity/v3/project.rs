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

//! Identity Project commands
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Identity Project commands
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ProjectCommands {
    Create(create::ProjectArgs),
    Delete(delete::ProjectArgs),
    List(list::ProjectsArgs),
    Set(set::ProjectArgs),
    Show(show::ProjectArgs),
}

pub struct ProjectCommand {
    pub args: ProjectArgs,
}

impl OSCCommand for ProjectCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ProjectCommands::Create(args) => {
                Ok(Box::new(create::ProjectCmd { args: args.clone() }))
            }
            ProjectCommands::Delete(args) => {
                Ok(Box::new(delete::ProjectCmd { args: args.clone() }))
            }
            ProjectCommands::List(args) => Ok(Box::new(list::ProjectsCmd { args: args.clone() })),
            ProjectCommands::Set(args) => Ok(Box::new(set::ProjectCmd { args: args.clone() })),
            ProjectCommands::Show(args) => Ok(Box::new(show::ProjectCmd { args: args.clone() })),
        }
    }
}
