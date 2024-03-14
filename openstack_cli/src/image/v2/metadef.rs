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

//! Glance Metadefs

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod namespace;
mod resource_type;

/// Metadefs
///
/// The Metadata Definitions Service (“metadefs”, for short) provides a common API for vendors,
/// operators, administrators, services, and users to meaningfully define available key:value pairs
/// that can be used on different types of cloud resources (for example, images, artifacts,
/// volumes, flavors, aggregates, and other resources).
///
/// To get you started, Glance contains a default catalog of metadefs that may be installed at your
/// site; see the README in the code repository for details.
///
/// Once a common catalog of metadata definitions has been created, the catalog is available for
/// querying through the API. Note that this service stores only the catalog, because metadefs are
/// meta-metadata. Metadefs provide information about resource metadata, but do not themselves
/// serve as actual metadata.
///
/// Actual key:value pairs are stored on the resources to which they apply using the metadata
/// facilities provided by the appropriate API. (For example, the Images API would be used to put
/// specific key:value pairs on a virtual machine image.)
///
/// A metadefs definition includes a property’s key, its description, its constraints, and the
/// resource types to which it can be associated. See [Metadata Definition
/// Concepts](https://docs.openstack.org/glance/latest/user/metadefs-concepts.html) in the Glance
/// Developer documentation for more information.
///
/// **Note**: By default, only admins can manipulate the data exposed by this API, but all users
/// may list and show public resources. This changed from a default of “open to all” in the Wallaby
/// release.
#[derive(Parser)]
pub struct MetadefCommand {
    /// subcommand
    #[command(subcommand)]
    command: MetadefCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MetadefCommands {
    Namespace(namespace::NamespaceCommand),
    ResourceType(resource_type::ResourceTypeCommand),
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
            MetadefCommands::ResourceType(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
