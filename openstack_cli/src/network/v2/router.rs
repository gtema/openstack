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
pub struct RouterArgs {
    #[command(subcommand)]
    command: RouterCommands,
}

#[derive(Subcommand, Clone)]
pub enum RouterCommands {
    /// List Routers
    List(list::RoutersArgs),
    /// Show single Router
    Show(show::RouterArgs),
    /// Create single Router
    Create(create::RouterArgs),
    /// Delete single Router
    Delete(delete::RouterArgs),
}

pub struct RouterCommand {
    pub args: RouterArgs,
}

impl OSCCommand for RouterCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            RouterCommands::List(args) => Ok(Box::new(list::RoutersCmd { args: args.clone() })),
            RouterCommands::Show(args) => Ok(Box::new(show::RouterCmd { args: args.clone() })),
            RouterCommands::Create(args) => Ok(Box::new(create::RouterCmd { args: args.clone() })),
            RouterCommands::Delete(args) => Ok(Box::new(delete::RouterCmd { args: args.clone() })),
        }
    }
}
