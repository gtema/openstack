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

//! Compute API v2 command

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod aggregate;
mod availability_zone;
mod extension;
mod flavor;
mod hypervisor;
mod keypair;
mod server;

/// Compute service (Nova) arguments
#[derive(Parser)]
pub struct ComputeCommand {
    /// Compute service resource
    #[command(subcommand)]
    command: ComputeCommands,
}

/// Compute resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ComputeCommands {
    #[command(about = "Host Aggregates")]
    Aggregate(Box<aggregate::AggregateCommand>),
    /// Lists and gets detailed availability zone information.
    ///
    /// An availability zone is created or updated by setting the
    /// availability_zone parameter in the create, update, or
    /// create or update methods of the Host Aggregates API. See
    /// Host Aggregates for more details.
    #[command(about = "Availability zones")]
    AvailabilityZone(Box<availability_zone::AvailabilityZoneCommand>),
    Extension(Box<extension::ExtensionCommand>),
    Flavor(Box<flavor::FlavorCommand>),
    Hypervisor(Box<hypervisor::HypervisorCommand>),
    Keypair(Box<keypair::KeypairCommand>),
    Server(Box<server::ServerCommand>),
}

impl ComputeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ComputeCommands::Aggregate(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::AvailabilityZone(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Extension(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Hypervisor(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Flavor(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Keypair(cmd) => cmd.take_action(parsed_args, session).await,
            ComputeCommands::Server(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
