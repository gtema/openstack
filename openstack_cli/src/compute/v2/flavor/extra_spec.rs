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

//! Compute Flavor Extra Specs commands
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod set;
mod show;

#[derive(Args, Clone, Debug)]
pub struct ExtraSpecsArgs {
    #[command(subcommand)]
    command: ExtraSpecsCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ExtraSpecsCommands {
    /// Creates extra specs for a flavor, by ID.
    #[command(about = "Create Extra Specs For A Flavor")]
    Create(create::ExtraSpecArgs),
    /// Deletes an extra spec, by key, for a flavor, by ID.
    #[command(about = "Delete An Extra Spec For A Flavor")]
    Delete(delete::ExtraSpecArgs),
    /// Lists all extra specs for a flavor, by ID.
    #[command(about = "List Extra Specs For A Flavor")]
    List(list::ExtraSpecsArgs),
    /// Shows an extra spec, by key, for a flavor, by ID.
    #[command(about = "Show An Extra Spec For A Flavor")]
    Show(show::ExtraSpecArgs),
    /// Updates an extra spec, by key, for a flavor, by ID.
    #[command(about = "Update An Extra Spec For A Flavor
")]
    Set(set::ExtraSpecArgs),
}

pub struct ExtraSpecsCommand {
    pub args: ExtraSpecsArgs,
}

impl OSCCommand for ExtraSpecsCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ExtraSpecsCommands::Create(args) => {
                Ok(Box::new(create::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Delete(args) => {
                Ok(Box::new(delete::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::List(args) => {
                Ok(Box::new(list::ExtraSpecsCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Show(args) => {
                Ok(Box::new(show::ExtraSpecCmd { args: args.clone() }))
            }
            ExtraSpecsCommands::Set(args) => Ok(Box::new(set::ExtraSpecCmd { args: args.clone() })),
        }
    }
}
