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

//! Placement `ResourceProviderAggregate` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod list;
mod set_11;
mod set_119;

/// Resource provider aggregates
///
/// Each resource provider can be associated with one or more other resource providers in groups
/// called aggregates. API calls in this section are used to list and update the aggregates that
/// are associated with one resource provider.
///
/// Provider aggregates are used for modeling relationships among providers. Examples may include:
///
///   - A shared storage pool providing DISK_GB resources to compute node providers that provide
///     VCPU and MEMORY_MB resources.
///
///   - Affinity/anti-affinity relationships such as physical location, power failure domains, or
///     other reliability/availability constructs.
///
///   - Groupings of compute host providers corresponding to Nova host aggregates or availability
///     zones.
///
/// Note: Placement aggregates are not the same as Nova host aggregates and should not be
/// considered equivalent.
///
/// The primary differences between Nova’s host aggregates and placement aggregates are the
/// following:
///
///   - In Nova, a host aggregate associates a nova-compute service with other nova-compute
///     services. Placement aggregates are not specific to a nova-compute service and are, in fact,
///     not compute-specific at all. A resource provider in the Placement API is generic, and
///     placement aggregates are simply groups of generic resource providers. This is an important
///     difference especially for Ironic, which when used with Nova, has many Ironic baremetal nodes
///     attached to a single nova-compute service. In the Placement API, each Ironic baremetal node
///     is its own resource provider and can therefore be associated to other Ironic baremetal nodes
///     via a placement aggregate association.
///
///   - In Nova, a host aggregate may have metadata key/value pairs attached to it. All
///     nova-compute services associated with a Nova host aggregate share the same metadata.
///     Placement aggregates have no such metadata because placement aggregates only represent the
///     grouping of resource providers. In the Placement API, resource providers are individually
///     decorated with traits that provide qualitative information about the resource provider.
///
///   - In Nova, a host aggregate dictates the availability zone within which one or more
///     nova-compute services reside. While placement aggregates may be used to model availability
///     zones, they have no inherent concept thereof.
#[derive(Parser)]
pub struct AggregateCommand {
    #[command(subcommand)]
    command: AggregateCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AggregateCommands {
    List(list::AggregateCommand),
    #[command(visible_alias = "set")]
    Set119(set_119::AggregateCommand),
    Set11(set_11::AggregateCommand),
}

impl AggregateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AggregateCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Set119(cmd) => cmd.take_action(parsed_args, session).await,
            AggregateCommands::Set11(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
