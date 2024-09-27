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

//! NDP Proxy commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Router NDP proxy (ndp_proxies)
///
/// A ndp_proxy is a logical entity for annunciate a unique IPv6 address to external network. It
/// depends on a router entity on which external gateway is enabled.
#[derive(Parser)]
pub struct NdpProxyCommand {
    /// subcommand
    #[command(subcommand)]
    command: NdpProxyCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NdpProxyCommands {
    Create(Box<create::NdpProxyCommand>),
    Delete(Box<delete::NdpProxyCommand>),
    List(Box<list::NdpProxiesCommand>),
    Set(Box<set::NdpProxyCommand>),
    Show(Box<show::NdpProxyCommand>),
}

impl NdpProxyCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            NdpProxyCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            NdpProxyCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            NdpProxyCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            NdpProxyCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            NdpProxyCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
