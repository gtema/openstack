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
mod replace;
mod set;
mod show;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct TagArgs {
    #[command(subcommand)]
    command: TagCommands,
}

#[derive(Subcommand, Clone)]
pub enum TagCommands {
    /// Add a tag
    ///
    /// Adds a tag on the resource.
    Add(set::TagArgs),
    /// Confirm tag presence
    ///
    /// Confirms a given tag is set on the resource.
    /// This method does not return any reasonable
    /// response, but fails with "not found" when tag is
    /// not present.
    Check(show::TagArgs),
    /// Remove a single tag
    ///
    /// Removes a tag on the resource.
    Delete(delete::TagArgs),
    /// List all tags
    ///
    /// Obtains the tags for a resource.
    List(list::TagsArgs),
    /// Remove all tags
    ///
    /// Removes all tags on the resource.
    Purge(delete_all::TagArgs),
    /// Replace all tags
    ///
    /// Replaces all tags on the resource.
    Replace(replace::TagArgs),
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
            TagCommands::Replace(args) => Ok(Box::new(replace::TagCmd { args: args.clone() })),
        }
    }
}
