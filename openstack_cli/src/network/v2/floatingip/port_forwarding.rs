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

//! FloatingIP Port Forwarding

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Floating IPs port forwarding
///
/// Lists, creates, shows details for, updates, and deletes floating IPs port forwardings.
///
/// ## Port forwarding with port ranges
///
/// The floating-ip-port-forwarding-port-ranges extension adds the new attributes
/// internal_port_range and external_port_range to the floating IP port forwardings. The value of
/// these new attributes should be a string that represents a colon separated port range. You can
/// not use the attributes internal_port_range and external_port_range with the attributes
/// internal_port and external_port in the same request.
///
/// ## Port forwarding rule description
///
/// The floating-ip-port-forwarding-description extension adds the description attribute to the
/// floating IP port forwardings. The value of the description attribute contains a text describing
/// the rule, which helps users to manage/find easily theirs rules.
#[derive(Parser)]
pub struct PortForwardingCommand {
    /// subcommand
    #[command(subcommand)]
    command: PortForwardingCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PortForwardingCommands {
    Create(create::PortForwardingCommand),
    Delete(delete::PortForwardingCommand),
    List(list::PortForwardingsCommand),
    Set(set::PortForwardingCommand),
    Show(show::PortForwardingCommand),
}

impl PortForwardingCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            PortForwardingCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            PortForwardingCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            PortForwardingCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            PortForwardingCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            PortForwardingCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
