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

//! Octavia `Provider` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod availability_zone_capability;
mod flavor_capability;
mod list;

/// Provider (Octavia) commands
#[derive(Parser)]
pub struct ProviderCommand {
    /// subcommand
    #[command(subcommand)]
    command: ProviderCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ProviderCommands {
    AvailabilityZoneCapability(availability_zone_capability::AvailabilityZoneCapabilityCommand),
    FlavorCapability(flavor_capability::FlavorCapabilityCommand),
    List(list::ProvidersCommand),
}

impl ProviderCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ProviderCommands::AvailabilityZoneCapability(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            ProviderCommands::FlavorCapability(cmd) => cmd.take_action(parsed_args, session).await,
            ProviderCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
