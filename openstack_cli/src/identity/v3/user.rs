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
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod access_rule;
pub mod application_credential;
mod create;
mod delete;
mod list;
mod password;
mod set;
mod show;
mod project {
    pub(super) mod list;
}
mod group {
    pub(super) mod list;
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
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct UserArgs {
    #[command(subcommand)]
    command: UserCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum UserCommands {
    Create(create::UserArgs),
    Delete(delete::UserArgs),
    List(list::UsersArgs),
    Set(set::UserArgs),
    Show(show::UserArgs),
    Password(password::PasswordArgs),
    Projects(project::list::ProjectsArgs),
    Groups(group::list::GroupsArgs),
}

pub struct UserCommand {
    pub args: UserArgs,
}

impl OSCCommand for UserCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            UserCommands::Create(args) => Ok(Box::new(create::UserCmd { args: args.clone() })),
            UserCommands::Delete(args) => Ok(Box::new(delete::UserCmd { args: args.clone() })),
            UserCommands::List(args) => Ok(Box::new(list::UsersCmd { args: args.clone() })),
            UserCommands::Set(args) => Ok(Box::new(set::UserCmd { args: args.clone() })),
            UserCommands::Show(args) => Ok(Box::new(show::UserCmd { args: args.clone() })),
            UserCommands::Password(args) => {
                password::PasswordCommand { args: args.clone() }.get_subcommand(session)
            }
            UserCommands::Projects(args) => {
                Ok(Box::new(project::list::ProjectsCmd { args: args.clone() }))
            }
            UserCommands::Groups(args) => {
                Ok(Box::new(group::list::GroupsCmd { args: args.clone() }))
            }
        }
    }
}
