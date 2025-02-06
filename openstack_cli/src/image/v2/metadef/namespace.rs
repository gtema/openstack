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

//! Glance Metadef namespace

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod object;
pub mod property;
pub mod resource_type;
pub mod set;
pub mod show;
pub mod tag;

/// Metadata definition namespaces
///
/// Creates, lists, shows details for, updates, and deletes metadata definition namespaces. Defines
/// namespaces that can contain property definitions, object definitions, and resource type
/// associations.
#[derive(Parser)]
pub struct NamespaceCommand {
    /// subcommand
    #[command(subcommand)]
    command: NamespaceCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NamespaceCommands {
    Create(create::NamespaceCommand),
    Delete(delete::NamespaceCommand),
    List(list::NamespacesCommand),
    Object(object::ObjectCommand),
    Property(property::PropertyCommand),
    ResourceTypeAssociation(resource_type::ResourceTypeCommand),
    Set(set::NamespaceCommand),
    Show(show::NamespaceCommand),
    Tag(tag::TagCommand),
}

impl NamespaceCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            NamespaceCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::Object(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::Property(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::ResourceTypeAssociation(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            NamespaceCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            NamespaceCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
