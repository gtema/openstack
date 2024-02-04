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

//! Compute Flavor commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod add_tenant_access;
mod create_20;
mod create_21;
mod create_255;
mod delete;
mod extra_spec;
mod flavor_access;
mod list;
mod remove_tenant_access;
mod set;
mod show;

/// Flavor commands
///
/// Flavors are a way to describe the basic dimensions of a server
/// to be created including how much cpu, ram, and disk space are
/// allocated to a server built with this flavor.
#[derive(Parser)]
pub struct FlavorCommand {
    /// subcommand
    #[command(subcommand)]
    command: FlavorCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum FlavorCommands {
    Access(Box<flavor_access::FlavorAccessCommand>),
    #[command(visible_alias = "create")]
    Create255(Box<create_255::FlavorCommand>),
    Create21(Box<create_21::FlavorCommand>),
    Create20(Box<create_20::FlavorCommand>),
    Delete(Box<delete::FlavorCommand>),
    Extraspecs(Box<extra_spec::ExtraSpecsCommand>),
    List(Box<list::FlavorsCommand>),
    Set(Box<set::FlavorCommand>),
    Show(Box<show::FlavorCommand>),
}

impl FlavorCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            FlavorCommands::Access(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Create255(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Create21(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Create20(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Extraspecs(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
