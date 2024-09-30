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

//! Identity User Access Credentials commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod show;

/// Application Credentials
///
/// Application credentials provide a way to delegate a user’s authorization to an application
/// without sharing the user’s password authentication. This is a useful security measure,
/// especially for situations where the user’s identification is provided by an external source,
/// such as LDAP or a single-sign-on service. Instead of storing user passwords in config files, a
/// user creates an application credential for a specific project, with all or a subset of the role
/// assignments they have on that project, and then stores the application credential identifier
/// and secret in the config file.
///
/// Multiple application credentials may be active at once, so you can easily rotate application
/// credentials by creating a second one, converting your applications to use it one by one, and
/// finally deleting the first one.
///
/// Application credentials are limited by the lifespan of the user that created them. If the user
/// is deleted, disabled, or loses a role assignment on a project, the application credential is
/// deleted.
///
/// Application credentials can have their privileges limited in two ways. First, the owner may
/// specify a subset of their own roles that the application credential may assume when getting a
/// token for a project. For example, if a user has the member role on a project, they also have
/// the implied role reader and can grant the application credential only the reader role for the
/// project:
///
/// ```json
///
/// "roles": [ {"name": "reader"} ]
///
/// ```
///
/// Users also have the option of delegating more fine-grained access control to their application
/// credentials by using access rules. For example, to create an application credential that is
/// constricted to creating servers in nova, the user can add the following access rules:
///
/// ```json
///
/// "access_rules": [ { "path": "/v2.1/servers", "method": "POST", "service": "compute" } ]
///
/// ```
///
/// The "path" attribute of application credential access rules uses a wildcard syntax to make it
/// more flexible. For example, to create an application credential that is constricted to listing
/// server IP addresses, you could use either of the following access rules:
///
/// ```json
///
/// "access_rules": [ { "path": "/v2.1/servers/*/ips", "method": "GET", "service": "compute" } ]
///
/// ```
///
/// or equivalently:
///
/// ```json
///
/// "access_rules": [ { "path": "/v2.1/servers/{server_id}/ips", "method": "GET", "service": "compute" } ]
///
/// ```
///
/// In both cases, a request path containing any server ID will match the access rule. For even
/// more flexibility, the recursive wildcard ** indicates that request paths containing any number
/// of / will be matched. For example:
///
/// ```json
///
/// "access_rules": [ { "path": "/v2.1/**", "method": "GET", "service": "compute" } ]
///
/// ```
///
/// will match any nova API for version 2.1.
///
/// An access rule created for one application credential can be re-used by providing its ID to
/// another application credential, for example:
///
/// ```json
///
/// "access_rules": [ { "id": "abcdef" } ]
///
/// ```
#[derive(Parser)]
pub struct ApplicationCredentialCommand {
    /// subcommand
    #[command(subcommand)]
    command: ApplicationCredentialCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ApplicationCredentialCommands {
    Create(create::ApplicationCredentialCommand),
    Delete(delete::ApplicationCredentialCommand),
    List(list::ApplicationCredentialsCommand),
    Show(show::ApplicationCredentialCommand),
}

impl ApplicationCredentialCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ApplicationCredentialCommands::Create(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ApplicationCredentialCommands::Delete(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ApplicationCredentialCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ApplicationCredentialCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
