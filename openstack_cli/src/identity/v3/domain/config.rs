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

//! Identity Config configuration

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod default;
mod delete_all;
mod group;
mod list;
mod replace;
mod set;

/// Config configuration
///
/// You can manage domain-specific configuration options.
///
/// Config-specific configuration options are structured within their group objects. The API
/// supports only the identity and ldap groups. These groups override the default configuration
/// settings for the storage of users and groups by the Identity server.
///
/// You can create, update, and delete domain-specific configuration options by using the HTTP PUT
/// , PATCH , and DELETE methods. When updating, it is only necessary to include those options that
/// are being updated.
///
/// To create an option, use the PUT method. The Identity API does not return options that are
/// considered sensitive, although you can create and update these options. The only option
/// currently considered sensitive is the password option within the ldap group.
///
/// The API enables you to include sensitive options as part of non- sensitive options. For
/// example, you can include the password as part of the url option.
///
/// If you try to create or update configuration options for groups other than whitelisted on the
/// server side, the Forbidden (403) response code is returned.
///
/// For information about how to integrate the Identity service with LDAP, see Integrate Identity
/// with LDAP.
///
/// A domain is a collection of users, groups, and projects. Each group and project is owned by
/// exactly one domain.
#[derive(Parser)]
pub struct ConfigCommand {
    /// subcommand
    #[command(subcommand)]
    command: ConfigCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ConfigCommands {
    Default(default::ConfigCommand),
    Group(group::GroupCommand),
    Purge(delete_all::ConfigCommand),
    List(list::ConfigsCommand),
    Replace(replace::ConfigCommand),
    Set(set::ConfigCommand),
}

impl ConfigCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ConfigCommands::Default(cmd) => cmd.take_action(parsed_args, session).await,
            ConfigCommands::Group(cmd) => cmd.take_action(parsed_args, session).await,
            ConfigCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ConfigCommands::Purge(cmd) => cmd.take_action(parsed_args, session).await,
            ConfigCommands::Replace(cmd) => cmd.take_action(parsed_args, session).await,
            ConfigCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
