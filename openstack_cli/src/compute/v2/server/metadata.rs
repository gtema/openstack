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

//! Compute Server metadata commands
#![deny(missing_docs)]
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod replace;
mod set;
mod show;

/// Lists metadata, creates or replaces one or more metadata items, and updates
/// one or more metadata items for a server.
///
/// Shows details for, creates or replaces, and updates a metadata item, by
/// key, for a server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
#[command(about = "Server metadata")]
pub struct MetadataArgs {
    #[command(subcommand)]
    command: MetadataCommands,
}

#[derive(Subcommand, Clone)]
pub enum MetadataCommands {
    Create(Box<create::MetadataArgs>),
    Delete(Box<delete::MetadataArgs>),
    List(Box<list::MetadatasArgs>),
    Replace(Box<replace::MetadataArgs>),
    Set(Box<set::MetadataArgs>),
    Show(Box<show::MetadataArgs>),
}

pub struct MetadataCommand {
    pub args: MetadataArgs,
}

impl OSCCommand for MetadataCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MetadataCommands::Create(args) => Ok(Box::new(create::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Delete(args) => Ok(Box::new(delete::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::List(args) => Ok(Box::new(list::MetadatasCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Replace(args) => Ok(Box::new(replace::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Set(args) => Ok(Box::new(set::MetadataCmd {
                args: *args.clone(),
            })),
            MetadataCommands::Show(args) => Ok(Box::new(show::MetadataCmd {
                args: *args.clone(),
            })),
        }
    }
}
