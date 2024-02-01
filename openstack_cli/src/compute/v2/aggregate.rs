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

pub mod add_host;
pub mod create_21;
pub mod delete;
pub mod list;
pub mod image {
    pub mod cache_281;
}
pub mod remove_host;
pub mod set_21;
pub mod set_metadata;
pub mod show;

/// Creates and manages host aggregates. An aggregate assigns
/// metadata to groups of compute nodes.
///
/// Policy defaults enable only users with the administrative role
/// to perform operations with aggregates. Cloud providers can
/// change these permissions through policy file configuration.
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AggregateArgs {
    #[command(subcommand)]
    command: AggregateCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AggregateCommands {
    /// Adds a host to an aggregate.
    #[command(about = "Add Host")]
    AddHost(add_host::AggregateArgs),
    Create(create_21::AggregateArgs),
    CacheImage(image::cache_281::ImageArgs),
    Delete(delete::AggregateArgs),
    List(list::AggregatesArgs),
    /// Removes a host from an aggregate.
    #[command(about = "Remove Host")]
    RemoveHost(remove_host::AggregateArgs),
    Show(show::AggregateArgs),
    Set(set_21::AggregateArgs),
    /// Creates or replaces metadata for an aggregate.
    #[command(about = "Create Or Update Aggregate Metadata")]
    SetMetadata(set_metadata::AggregateArgs),
}

pub struct AggregateCommand {
    pub args: AggregateArgs,
}

impl OSCCommand for AggregateCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AggregateCommands::AddHost(args) => {
                Ok(Box::new(add_host::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::CacheImage(args) => {
                Ok(Box::new(image::cache_281::ImageCmd { args: args.clone() }))
            }
            AggregateCommands::Create(args) => {
                Ok(Box::new(create_21::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::Delete(args) => {
                Ok(Box::new(delete::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::List(args) => {
                Ok(Box::new(list::AggregatesCmd { args: args.clone() }))
            }
            AggregateCommands::RemoveHost(args) => {
                Ok(Box::new(remove_host::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::Show(args) => {
                Ok(Box::new(show::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::Set(args) => {
                Ok(Box::new(set_21::AggregateCmd { args: args.clone() }))
            }
            AggregateCommands::SetMetadata(args) => {
                Ok(Box::new(set_metadata::AggregateCmd { args: args.clone() }))
            }
        }
    }
}
