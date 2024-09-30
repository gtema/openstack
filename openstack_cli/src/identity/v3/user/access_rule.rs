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

//! Identity User Access Rules commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod delete;
mod list;
mod show;

/// Application Credentials - Access Rules
///
/// Users have the option of delegating more fine-grained access control to their application
/// credentials by using access rules. For example, to create an application credential that is
/// constricted to creating servers in nova, the user can add the following access rules:
///
/// ```json
///
/// { "access_rules": [{ "path": "/v2.1/servers", "method": "POST", "service": "compute" }] }
///
/// ```
///
/// The "path" attribute of application credential access rules uses a wildcard syntax to make it
/// more flexible. For example, to create an application credential that is constricted to listing
/// server IP addresses, you could use either of the following access rules:
///
/// ```json
///
/// { "access_rules": [ { "path": "/v2.1/servers/*/ips", "method": "GET", "service": "compute" } ] }
///
/// ```
///
/// or equivalently:
///
/// ```json
///
/// { "access_rules": [ { "path": "/v2.1/servers/{server_id}/ips", "method": "GET", "service": "compute" } ] }
///
/// ```
///
/// In both cases, a request path containing any server ID will match the access rule. For even
/// more flexibility, the recursive wildcard ** indicates that request paths containing any number
/// of / will be matched. For example:
///
/// ```json
///
/// { "access_rules": [ { "path": "/v2.1/**", "method": "GET", "service": "compute" } ] }
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
/// { "access_rules": [ { "id": "abcdef" } ] }
///
///
/// ```
#[derive(Parser)]
pub struct AccessRuleCommand {
    /// subcommand
    #[command(subcommand)]
    command: AccessRuleCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AccessRuleCommands {
    Delete(delete::AccessRuleCommand),
    List(list::AccessRulesCommand),
    Show(show::AccessRuleCommand),
}

impl AccessRuleCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AccessRuleCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            AccessRuleCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AccessRuleCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
