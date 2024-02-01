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

mod list;
mod show;

/// Servers actions
///
/// List actions and action details for a server.
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct InstanceActionArgs {
    #[command(subcommand)]
    command: InstanceActionCommands,
}

#[derive(Subcommand, Clone)]
pub enum InstanceActionCommands {
    List(Box<list::InstanceActionsArgs>),
    Show(Box<show::InstanceActionArgs>),
}

pub struct InstanceActionCommand {
    pub args: InstanceActionArgs,
}

impl OSCCommand for InstanceActionCommand {
    fn get_subcommand(
        &self,
        _session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            InstanceActionCommands::List(args) => Ok(Box::new(list::InstanceActionsCmd {
                args: *args.clone(),
            })),
            InstanceActionCommands::Show(args) => Ok(Box::new(show::InstanceActionCmd {
                args: *args.clone(),
            })),
        }
    }
}
