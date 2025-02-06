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

//! Identity domain group role

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod delete;
pub mod list;
pub mod set;
pub mod show;

/// Domain group roles
///
/// OpenStack services typically determine whether a user’s API request should be allowed using
/// Role Based Access Control (RBAC). For OpenStack this means the service compares the roles that
/// user has on the project (as indicated by the roles in the token), against the roles required
/// for the API in question (as defined in the service’s policy file). A user obtains roles on a
/// project by having these assigned to them via the Identity service API.
///
/// Roles must initially be created as entities via the Identity services API and, once created,
/// can then be assigned. You can assign roles to a user or group on a project, including projects
/// owned by other domains. You can also assign roles to a user or group on a domain, although this
/// is only currently relevant for using a domain scoped token to execute domain-level Identity
/// service API requests.
///
#[derive(Parser)]
pub struct RoleCommand {
    /// subcommand
    #[command(subcommand)]
    command: RoleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RoleCommands {
    Delete(delete::RoleCommand),
    List(list::RolesCommand),
    Set(set::RoleCommand),
    Show(show::RoleCommand),
}

impl RoleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RoleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            RoleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
