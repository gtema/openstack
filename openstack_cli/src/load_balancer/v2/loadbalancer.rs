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

//! Octavia `Loadbalancer` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod failover;
mod list;
mod set;
mod show;
mod stats;
mod status;

/// Loadbalancer (Octavia) commands
#[derive(Parser)]
pub struct LoadbalancerCommand {
    /// subcommand
    #[command(subcommand)]
    command: LoadbalancerCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LoadbalancerCommands {
    Create(Box<create::LoadbalancerCommand>),
    Delete(Box<delete::LoadbalancerCommand>),
    Failover(Box<failover::LoadbalancerCommand>),
    List(Box<list::LoadbalancersCommand>),
    Set(Box<set::LoadbalancerCommand>),
    Show(Box<show::LoadbalancerCommand>),
    Stats(Box<stats::LoadbalancerCommand>),
    Status(Box<status::LoadbalancerCommand>),
}

impl LoadbalancerCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            LoadbalancerCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Failover(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Stats(cmd) => cmd.take_action(parsed_args, session).await,
            LoadbalancerCommands::Status(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
