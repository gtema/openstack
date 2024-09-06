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

//! Identity Credential commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Identity Credential commands
///
/// In exchange for a set of authentication credentials that the user submits, the Identity service
/// generates and returns a token. A token represents the authenticated identity of a user and,
/// optionally, grants authorization on a specific project or domain.
///
/// You can list all credentials, and create, show details for, update, and delete a credential.

#[derive(Parser)]
pub struct CredentialCommand {
    /// subcommand
    #[command(subcommand)]
    command: CredentialCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum CredentialCommands {
    Create(create::CredentialCommand),
    Delete(delete::CredentialCommand),
    List(list::CredentialsCommand),
    Set(set::CredentialCommand),
    Show(show::CredentialCommand),
}

impl CredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            CredentialCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            CredentialCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            CredentialCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            CredentialCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            CredentialCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
