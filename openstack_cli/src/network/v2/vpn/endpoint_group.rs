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

//! EndpointGroup commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// VPN Endpoint Groups
///
/// The endpoint-groups extension adds support for defining one or more endpoints of a specific
/// type, and can be used to specify both local and peer endpoints for IPsec connections.
#[derive(Parser)]
pub struct EndpointGroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: EndpointGroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum EndpointGroupCommands {
    Create(create::EndpointGroupCommand),
    Delete(delete::EndpointGroupCommand),
    List(list::EndpointGroupsCommand),
    Set(set::EndpointGroupCommand),
    Show(show::EndpointGroupCommand),
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
            EndpointGroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            EndpointGroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
