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

pub mod delete;
pub mod force_complete_222;
pub mod list;
pub mod show;

/// Server migrations (servers, migrations)
///
/// List, show, perform actions on and delete server migrations.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct MigrationArgs {
    #[command(subcommand)]
    command: MigrationCommands,
}

#[derive(Subcommand, Clone)]
pub enum MigrationCommands {
    Delete(delete::MigrationArgs),
    ForceComplete(force_complete_222::MigrationArgs),
    List(list::MigrationsArgs),
    Show(show::MigrationArgs),
}

pub struct MigrationCommand {
    pub args: MigrationArgs,
}

impl OSCCommand for MigrationCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            MigrationCommands::Delete(args) => {
                Ok(Box::new(delete::MigrationCmd { args: args.clone() }))
            }
            MigrationCommands::ForceComplete(args) => {
                Ok(Box::new(force_complete_222::MigrationCmd {
                    args: args.clone(),
                }))
            }
            MigrationCommands::List(args) => {
                Ok(Box::new(list::MigrationsCmd { args: args.clone() }))
            }
            MigrationCommands::Show(args) => {
                Ok(Box::new(show::MigrationCmd { args: args.clone() }))
            }
        }
    }
}
