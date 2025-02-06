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

//! Identity Domain commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod config;
pub mod create;
pub mod delete;
pub mod group;
pub mod list;
pub mod set;
pub mod show;
pub mod user;

/// Identity Domain commands
///
/// A domain is a collection of users, groups, and projects. Each group and project is owned by exactly one domain.
///
/// Each domain defines a namespace where certain API-visible name attributes exist, which affects
/// whether those names must be globally unique or unique within that domain. In the Identity API,
/// the uniqueness of these attributes is as follows:
///
///   - Domain name. Globally unique across all domains.
///
///   - Role name. Unique within the owning domain.
///
///   - User name. Unique within the owning domain.
///
///   - Project name. Unique within the owning domain.
///
///   - Group name. Unique within the owning domain.

#[derive(Parser)]
pub struct DomainCommand {
    /// subcommand
    #[command(subcommand)]
    command: DomainCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DomainCommands {
    Create(create::DomainCommand),
    Config(config::ConfigCommand),
    Delete(delete::DomainCommand),
    Group(group::GroupCommand),
    List(list::DomainsCommand),
    Set(set::DomainCommand),
    Show(show::DomainCommand),
    User(user::UserCommand),
}

impl DomainCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            DomainCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::Config(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::Group(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            DomainCommands::User(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
