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

//! Keypairs (keypairs)
//!
//! Generates, imports, and deletes SSH keys.
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create_20;
mod create_21;
mod create_210;
mod create_22;
mod create_292;
mod delete;
mod list;
mod show;

/// Keypairs commands
///
/// Generates, imports, and deletes SSH keys.
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct KeypairArgs {
    #[command(subcommand)]
    command: KeypairCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum KeypairCommands {
    List(list::KeypairsArgs),
    Show(show::KeypairArgs),
    #[command(visible_alias = "create")]
    Create292(create_292::KeypairArgs),
    Create210(create_210::KeypairArgs),
    Create22(create_22::KeypairArgs),
    Create21(create_21::KeypairArgs),
    Create20(create_20::KeypairArgs),
    Delete(delete::KeypairArgs),
}

pub struct KeypairCommand {
    /// Command arguments
    pub args: KeypairArgs,
}

impl OSCCommand for KeypairCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            KeypairCommands::List(args) => Ok(Box::new(list::KeypairsCmd { args: args.clone() })),
            KeypairCommands::Show(args) => Ok(Box::new(show::KeypairCmd { args: args.clone() })),
            KeypairCommands::Create292(args) => {
                Ok(Box::new(create_292::KeypairCmd { args: args.clone() }))
            }
            KeypairCommands::Create210(args) => {
                Ok(Box::new(create_210::KeypairCmd { args: args.clone() }))
            }
            KeypairCommands::Create22(args) => {
                Ok(Box::new(create_22::KeypairCmd { args: args.clone() }))
            }
            KeypairCommands::Create21(args) => {
                Ok(Box::new(create_21::KeypairCmd { args: args.clone() }))
            }
            KeypairCommands::Create20(args) => {
                Ok(Box::new(create_20::KeypairCmd { args: args.clone() }))
            }
            KeypairCommands::Delete(args) => {
                Ok(Box::new(delete::KeypairCmd { args: args.clone() }))
            }
        }
    }
}
