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

//! Identity Service commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Service commands
///
/// A service is an OpenStack web service that you can access through a URL, i.e. an endpoint.
///
/// You can create, list, show details for, update, and delete services. When you create or update
/// a service, you can enable the service, which causes it and its endpoints to appear in the
/// service catalog.
#[derive(Parser)]
pub struct ServiceCommand {
    #[command(subcommand)]
    command: ServiceCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ServiceCommands {
    Create(create::ServiceCommand),
    Delete(delete::ServiceCommand),
    List(list::ServicesCommand),
    Set(set::ServiceCommand),
    Show(show::ServiceCommand),
}

impl ServiceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ServiceCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ServiceCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
