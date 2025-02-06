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

//! Identity Group commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod set;
pub mod show;
pub mod user;

/// Identity Group commands
///
/// A group is a collection of users. Each group is owned by a domain.
///
/// You can use groups to ease the task of managing role assignments for users. Assigning a role to
/// a group on a project or domain is equivalent to assigning the role to each group member on that
/// project or domain.
///
/// When you unassign a role from a group, that role is automatically unassigned from any user that
/// is a member of the group. Any tokens that authenticates those users to the relevant project or
/// domain are revoked.
///
/// As with users, a group without any role assignments is useless from the perspective of an
/// OpenStack service and has no access to resources. However, a group without role assignments is
/// permitted as a way of acquiring or loading users and groups from external sources before
/// mapping them to projects and domains.
#[derive(Parser)]
pub struct GroupCommand {
    /// subcommand
    #[command(subcommand)]
    command: GroupCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum GroupCommands {
    Create(Box<create::GroupCommand>),
    Delete(Box<delete::GroupCommand>),
    List(Box<list::GroupsCommand>),
    Set(Box<set::GroupCommand>),
    Show(Box<show::GroupCommand>),
    User(Box<user::UserCommand>),
}

impl GroupCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            GroupCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            GroupCommands::User(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
