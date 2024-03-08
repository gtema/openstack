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

//! Router commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod add_external_gateways;
mod add_extraroutes;
mod add_router_interface;
mod conntrack_helper;
mod create;
mod delete;
mod l3_agent;
mod list;
mod remove_external_gateways;
mod remove_extraroutes;
mod remove_router_interface;
mod show;
mod tag;

/// Router commands
#[derive(Parser)]
pub struct RouterCommand {
    /// subcommand
    #[command(subcommand)]
    command: RouterCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RouterCommands {
    AddExternalGateway(Box<add_external_gateways::RouterCommand>),
    AddExtraroute(Box<add_extraroutes::RouterCommand>),
    AddRouterInterface(Box<add_router_interface::RouterCommand>),
    ConntrackHelper(Box<conntrack_helper::ConntrackHelperCommand>),
    Create(create::RouterCommand),
    Delete(delete::RouterCommand),
    L3Agent(l3_agent::L3AgentCommand),
    List(list::RoutersCommand),
    RemoveExternalGateway(Box<remove_external_gateways::RouterCommand>),
    RemoveExtraroute(Box<remove_extraroutes::RouterCommand>),
    RemoveRouterInterface(Box<remove_router_interface::RouterCommand>),
    Show(show::RouterCommand),
    Tag(tag::TagCommand),
}

impl RouterCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RouterCommands::AddExternalGateway(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::AddExtraroute(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::AddRouterInterface(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::ConntrackHelper(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::L3Agent(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::RemoveExternalGateway(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            RouterCommands::RemoveExtraroute(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::RemoveRouterInterface(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            RouterCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            RouterCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
