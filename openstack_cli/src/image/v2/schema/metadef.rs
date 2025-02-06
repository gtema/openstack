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

//! Image members schema

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod namespace;
pub mod namespaces;
pub mod object;
pub mod objects;
pub mod properties;
pub mod property;
pub mod resource_type;
pub mod resource_types;
pub mod tag;
pub mod tags;

/// Metadata definition schemas
///
/// Gets a JSON-schema document that represents a metadata definition entity.
#[derive(Parser)]
pub struct MetadefCommand {
    #[command(subcommand)]
    command: MetadefCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MetadefCommands {
    Namespace(namespace::NamespaceCommand),
    Namespaces(namespaces::NamespacesCommand),
    Object(object::ObjectCommand),
    Objects(objects::ObjectsCommand),
    Properties(properties::PropertiesCommand),
    Property(property::PropertyCommand),
    ResourceType(resource_type::ResourceTypeCommand),
    ResourceTypes(resource_types::ResourceTypesCommand),
    Tag(tag::TagCommand),
    Tags(tags::TagsCommand),
}

impl MetadefCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MetadefCommands::Namespace(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Namespaces(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Object(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Objects(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Properties(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Property(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::ResourceType(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::ResourceTypes(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
            MetadefCommands::Tags(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
