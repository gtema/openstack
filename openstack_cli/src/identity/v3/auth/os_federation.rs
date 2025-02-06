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

//! Identity Federated auth commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod identity_provider;
//mod saml2;
pub mod websso;

/// Identity Federated auch commands
///
/// authorization.
#[derive(Parser)]
pub struct OsFederationCommand {
    /// subcommand
    #[command(subcommand)]
    command: OsFederationCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum OsFederationCommands {
    IdentityProvider(identity_provider::IdentityProviderCommand),
    //    Saml2(saml2::Saml2Command),
    Websso(websso::WebssoCommand),
}

impl OsFederationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            OsFederationCommands::IdentityProvider(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            //            OsFederationCommands::Saml2(cmd) => cmd.take_action(parsed_args, session).await,
            OsFederationCommands::Websso(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
