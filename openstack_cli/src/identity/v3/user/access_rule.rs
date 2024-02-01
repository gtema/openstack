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
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod list;
mod show;

/// **Application Credentials - Access Rules**
///
/// Users also have the option of delegating more
/// fine-grained access control to their application
/// credentials by using access rules. For example, to
/// create an application credential that is constricted
/// to creating servers in nova, the user can add the
/// following access rules:
///
/// ```json { "access_rules": [{ "path": "/v2.1/servers",
/// "method": "POST", "service": "compute" }] } ```
///
/// The "path" attribute of application credential access
/// rules uses a wildcard syntax to make it more flexible.
/// For example, to create an application credential that
/// is constricted to listing server IP addresses, you
/// could use either of the following access rules:
///
/// ```json { "access_rules": [ { "path":
/// "/v2.1/servers/*/ips", "method": "GET", "service":
/// "compute" } ] } ```
///
/// or equivalently:
///
/// ```json { "access_rules": [ { "path":
/// "/v2.1/servers/{server_id}/ips", "method": "GET",
/// "service": "compute" } ] } ```
///
/// In both cases, a request path containing any server ID
/// will match the access rule. For even more flexibility,
/// the recursive wildcard ** indicates that request paths
/// containing any number of / will be matched. For
/// example:
///
/// ```json { "access_rules": [ { "path": "/v2.1/**",
/// "method": "GET", "service": "compute" } ] } ```
///
/// will match any nova API for version 2.1.
///
/// An access rule created for one application credential
/// can be re-used by providing its ID to another
/// application credential, for example:
///
/// ```json { "access_rules": [ { "id": "abcdef" } ] } ```
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct AccessRuleArgs {
    #[command(subcommand)]
    command: AccessRuleCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AccessRuleCommands {
    Delete(delete::AccessRuleArgs),
    List(list::AccessRulesArgs),
    Show(show::AccessRuleArgs),
}

pub struct AccessRuleCommand {
    pub args: AccessRuleArgs,
}

impl OSCCommand for AccessRuleCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AccessRuleCommands::Delete(args) => {
                Ok(Box::new(delete::AccessRuleCmd { args: args.clone() }))
            }
            AccessRuleCommands::List(args) => {
                Ok(Box::new(list::AccessRulesCmd { args: args.clone() }))
            }
            AccessRuleCommands::Show(args) => {
                Ok(Box::new(show::AccessRuleCmd { args: args.clone() }))
            }
        }
    }
}
