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

//! Identity EndpointGroup Filter commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod endpoint;
mod list;
mod project;
mod set;
mod show;

/// EndpointGroup project API
#[derive(Parser)]
pub struct EndpointGroupCommand {
    #[command(subcommand)]
    command: EndpointGroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum EndpointGroupCommands {
    Create(Box<create::EndpointGroupCommand>),
    Delete(Box<delete::EndpointGroupCommand>),
    Endpoint(Box<endpoint::EndpointCommand>),
    List(Box<list::EndpointGroupsCommand>),
    Project(Box<project::ProjectCommand>),
    Set(Box<set::EndpointGroupCommand>),
    Show(Box<show::EndpointGroupCommand>),
}

impl EndpointGroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            EndpointGroupCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Endpoint(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Project(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
