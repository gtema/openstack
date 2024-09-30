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

//! Services

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod list;
mod set_20;
mod set_211;
mod set_253;

/// Server groups (os-server-groups)
///
/// Lists, shows information for, creates, and deletes server groups.
#[derive(Parser)]
pub struct ServiceCommand {
    /// subcommand
    #[command(subcommand)]
    command: ServiceCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ServiceCommands {
    Delete(delete::ServiceCommand),
    List(list::ServicesCommand),
    #[command(visible_alias = "set")]
    Set253(set_253::ServiceCommand),
    Set211(set_211::ServiceCommand),
    Set20(set_20::ServiceCommand),
}

impl ServiceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ServiceCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Set253(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Set211(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Set20(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
