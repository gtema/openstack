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

//! Server IP commands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod list;
mod show;

/// Servers IPs (servers, ips)
///
/// Lists the IP addresses for an instance and shows details for an IP address.
#[derive(Parser)]
pub struct IpCommand {
    /// subcommand
    #[command(subcommand)]
    command: IpCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum IpCommands {
    List(list::IpsCommand),
    Show(show::IpCommand),
}

impl IpCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            IpCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            IpCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
