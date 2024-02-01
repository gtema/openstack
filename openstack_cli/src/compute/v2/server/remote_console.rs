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

pub mod create_26;
pub mod create_28;

/// Server Consoles
///
/// Manage server consoles.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct RemoteConsoleArgs {
    #[command(subcommand)]
    command: RemoteConsoleCommands,
}

#[derive(Subcommand, Clone)]
pub enum RemoteConsoleCommands {
    Create26(create_26::RemoteConsoleArgs),
    #[command(visible_alias = "create")]
    Create28(create_28::RemoteConsoleArgs),
}

pub struct RemoteConsoleCommand {
    pub args: RemoteConsoleArgs,
}

impl OSCCommand for RemoteConsoleCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            RemoteConsoleCommands::Create26(args) => {
                Ok(Box::new(create_26::RemoteConsoleCmd { args: args.clone() }))
            }
            RemoteConsoleCommands::Create28(args) => {
                Ok(Box::new(create_28::RemoteConsoleCmd { args: args.clone() }))
            }
        }
    }
}
