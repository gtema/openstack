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

//! Authorization operations

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod login;
mod show;

/// Cloud Authentication operations
///
/// This command provides various authorization
/// operations (login, show, status, etc)
#[derive(Parser)]
pub struct AuthCommand {
    /// Authentication commands
    #[command(subcommand)]
    pub(crate) command: AuthCommands,
}

#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AuthCommands {
    Login(login::LoginCommand),
    Show(show::ShowCommand),
}

impl AuthCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AuthCommands::Show(cmd) => cmd.take_action(parsed_args, client).await,
            AuthCommands::Login(cmd) => cmd.take_action(parsed_args, client).await,
        }
    }
}
