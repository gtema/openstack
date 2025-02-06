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

//! Vpnservice commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// VPN Service - A parent object that associates VPN with a specific subnet and router.
#[derive(Parser)]
pub struct VpnserviceCommand {
    /// subcommand
    #[command(subcommand)]
    command: VpnserviceCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum VpnserviceCommands {
    Create(create::VpnserviceCommand),
    Delete(delete::VpnserviceCommand),
    List(list::VpnservicesCommand),
    Set(set::VpnserviceCommand),
    Show(show::VpnserviceCommand),
}

impl VpnserviceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            VpnserviceCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            VpnserviceCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            VpnserviceCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            VpnserviceCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            VpnserviceCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
