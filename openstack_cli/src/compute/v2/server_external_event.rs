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

//! External event

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create_20;
mod create_251;
mod create_276;
mod create_282;
mod create_293;

/// Create external events (os-server-external-events)
///
/// Creates one or more external events. The API dispatches each event to a server instance.
///
/// **Warning**
///
/// This is an admin level service API only designed to be used by other OpenStack services. The
/// point of this API is to coordinate between Nova and Neutron, Nova and Cinder, Nova and Ironic
/// (and potentially future services) on activities they both need to be involved in, such as
/// network hotplugging.
///
/// Unless you are writing Neutron, Cinder or Ironic code you should not be using this API.
#[derive(Parser)]
pub struct ServerExternalEventCommand {
    /// subcommand
    #[command(subcommand)]
    command: ServerExternalEventCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ServerExternalEventCommands {
    Create20(create_20::ServerExternalEventCommand),
    Create251(create_251::ServerExternalEventCommand),
    Create276(create_276::ServerExternalEventCommand),
    Create282(create_282::ServerExternalEventCommand),
    #[command(visible_alias = "create")]
    Create293(create_293::ServerExternalEventCommand),
}

impl ServerExternalEventCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ServerExternalEventCommands::Create20(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ServerExternalEventCommands::Create251(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ServerExternalEventCommands::Create276(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ServerExternalEventCommands::Create282(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ServerExternalEventCommands::Create293(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
