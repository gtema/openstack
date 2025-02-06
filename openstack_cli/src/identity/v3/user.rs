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

//! Identity User commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod access_rule;
pub mod application_credential;
pub mod create;
pub mod delete;
pub mod list;
pub mod password;
pub mod set;
pub mod show;
/// User project commands
pub mod project {
    pub mod list;
}
/// User group commands
pub mod group {
    pub mod list;
}

/// User commands
///
/// A user is an individual API consumer that is owned by a domain. A role
/// explicitly associates a user with projects or domains. A user with no
/// assigned roles has no access to OpenStack resources.
///
/// You can list, create, show details for, update, delete, and change the
/// password for users.
///
/// You can also list groups, projects, and role assignments for a specified
/// user.
#[derive(Parser)]
pub struct UserCommand {
    #[command(subcommand)]
    command: UserCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum UserCommands {
    Create(create::UserCommand),
    Delete(delete::UserCommand),
    Groups(group::list::GroupsCommand),
    List(list::UsersCommand),
    Password(password::PasswordCommand),
    Projects(project::list::ProjectsCommand),
    Set(set::UserCommand),
    Show(show::UserCommand),
}

impl UserCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            UserCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Groups(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Password(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Projects(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            UserCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
