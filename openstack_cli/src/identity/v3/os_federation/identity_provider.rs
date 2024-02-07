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

//! Identity Federation IdentityProvider commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod protocol;
mod set;
mod show;

/// Identity Providers
///
/// An Identity Provider (IdP) is a third party service that is trusted by the
/// Identity API to authenticate identities.
#[derive(Parser)]
pub struct IdentityProviderCommand {
    #[command(subcommand)]
    command: IdentityProviderCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum IdentityProviderCommands {
    Create(create::IdentityProviderCommand),
    Delete(delete::IdentityProviderCommand),
    List(list::IdentityProvidersCommand),
    Protocol(protocol::ProtocolCommand),
    Set(set::IdentityProviderCommand),
    Show(show::IdentityProviderCommand),
}

impl IdentityProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            IdentityProviderCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityProviderCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityProviderCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityProviderCommands::Protocol(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityProviderCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityProviderCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
