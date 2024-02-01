// Copyright 2024
//
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

mod delete;
mod list;
mod set;
//mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TagArgs {
    #[command(subcommand)]
    command: TagCommands,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    Delete(delete::TagArgs),
    List(list::TagsArgs),
    Set(set::TagArgs),
    //    Show(show::TagArgs),
}

pub struct TagCommand {
    pub args: TagArgs,
}

impl OSCCommand for TagCommand {
    fn get_subcommand(&self) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            TagCommands::Delete(args) => Box::new(delete::TagCmd { args: args.clone() }),
            TagCommands::List(args) => Box::new(list::TagsCmd { args: args.clone() }),
            TagCommands::Set(args) => Box::new(set::TagCmd { args: args.clone() }),
            //            TagCommands::Show(args) => Box::new(show::TagCmd { args: args.clone() }),
        }
    }
}
