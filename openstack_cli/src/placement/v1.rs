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

//! Placement v1 commands

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

/// The placement API service was introduced in the 14.0.0 Newton release within the nova
/// repository and extracted to the placement repository in the 19.0.0 Stein release. This is a
/// REST API stack and data model used to track resource provider inventories and usages, along
/// with different classes of resources. For example, a resource provider can be a compute node, a
/// shared storage pool, or an IP allocation pool. The placement service tracks the inventory and
/// usage of each provider. For example, an instance created on a compute node may be a consumer of
/// resources such as RAM and CPU from a compute node resource provider, disk from an external
/// shared storage pool resource provider and IP addresses from an external IP pool resource
/// provider.
///
/// The types of resources consumed are tracked as classes. The service provides a set of standard
/// resource classes (for example DISK_GB, MEMORY_MB, and VCPU) and provides the ability to define
/// custom resource classes as needed.
///
/// Each resource provider may also have a set of traits which describe qualitative aspects of the
/// resource provider. Traits describe an aspect of a resource provider that cannot itself be
/// consumed but a workload may wish to specify. For example, available disk may be solid state
/// drives (SSD).
#[derive(Parser)]
pub struct PlacementCommand {
    /// Placement service resource
    #[command(subcommand)]
    command: PlacementCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum PlacementCommands {}

impl PlacementCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        _parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::Placement)
            .await?;

        todo!()
    }
}
