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

pub mod create_249;
pub mod delete;
pub mod list;
pub mod show;

/// Port interfaces (servers, os-interface)
///
/// List port interfaces, show port interface details of the given server.
/// Create a port interface and uses it to attach a port to the given server,
/// detach a port interface from the given server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InterfaceArgs {
    #[command(subcommand)]
    command: InterfaceCommands,
}

#[derive(Subcommand, Clone)]
pub enum InterfaceCommands {
    Create(create_249::InterfaceArgs),
    Delete(delete::InterfaceArgs),
    List(list::InterfacesArgs),
    Show(show::InterfaceArgs),
}

pub struct InterfaceCommand {
    pub args: InterfaceArgs,
}

impl OSCCommand for InterfaceCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            InterfaceCommands::Create(args) => {
                Ok(Box::new(create_249::InterfaceCmd { args: args.clone() }))
            }
            InterfaceCommands::Delete(args) => {
                Ok(Box::new(delete::InterfaceCmd { args: args.clone() }))
            }
            InterfaceCommands::List(args) => {
                Ok(Box::new(list::InterfacesCmd { args: args.clone() }))
            }
            InterfaceCommands::Show(args) => {
                Ok(Box::new(show::InterfaceCmd { args: args.clone() }))
            }
        }
    }
}
