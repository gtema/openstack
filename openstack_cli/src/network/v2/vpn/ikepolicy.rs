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

//! Ikepolicy commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// The Internet Key Exchange (IKE) policy that identifies the authentication and encryption
/// algorithm to use during phase one and two negotiation of a VPN connection.
#[derive(Parser)]
pub struct IkepolicyCommand {
    /// subcommand
    #[command(subcommand)]
    command: IkepolicyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum IkepolicyCommands {
    Create(create::IkepolicyCommand),
    Delete(delete::IkepolicyCommand),
    List(list::IkepoliciesCommand),
    Set(set::IkepolicyCommand),
    Show(show::IkepolicyCommand),
}

impl IkepolicyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            IkepolicyCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            IkepolicyCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            IkepolicyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            IkepolicyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            IkepolicyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
