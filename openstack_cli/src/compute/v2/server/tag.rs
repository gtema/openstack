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

use openstack_sdk::AsyncOpenStack;

mod delete;
mod delete_all;
mod list;
mod replace_226;
mod set;
mod show;

/// Lists tags, creates, replaces or deletes one or more tags for a
/// server, checks the existence of a tag for a server.
///
/// Available since version 2.26
///
/// Tags have the following restrictions:
///
///  - Tag is a Unicode bytestring no longer than 60 characters.
///
///  - Tag is a non-empty string.
///
///  - ‘/’ is not allowed to be in a tag name
///
///  - Comma is not allowed to be in a tag name in order to
///  simplify requests that specify lists of tags
///
///  - All other characters are allowed to be in a tag name
///
///  - Each server can have up to 50 tags.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TagArgs {
    #[command(subcommand)]
    command: TagCommands,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    Add(set::TagArgs),
    Check(show::TagArgs),
    Delete(delete::TagArgs),
    List(list::TagsArgs),
    Purge(delete_all::TagArgs),
    Replace(replace_226::TagArgs),
}

pub struct TagCommand {
    pub args: TagArgs,
}

impl OSCCommand for TagCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            TagCommands::Add(args) => Ok(Box::new(set::TagCmd { args: args.clone() })),
            TagCommands::Check(args) => Ok(Box::new(show::TagCmd { args: args.clone() })),
            TagCommands::Delete(args) => Ok(Box::new(delete::TagCmd { args: args.clone() })),
            TagCommands::List(args) => Ok(Box::new(list::TagsCmd { args: args.clone() })),
            TagCommands::Purge(args) => Ok(Box::new(delete_all::TagCmd { args: args.clone() })),
            TagCommands::Replace(args) => Ok(Box::new(replace_226::TagCmd { args: args.clone() })),
        }
    }
}
