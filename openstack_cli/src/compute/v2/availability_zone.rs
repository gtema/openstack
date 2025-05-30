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

//! Availability zone management

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod list;
pub mod list_detail;

/// Availability zones
///
/// Lists and gets detailed availability zone information.
///
/// An availability zone is created or updated by setting the
/// availability_zone parameter in the create, update, or create or update
/// methods of the Host Aggregates API. See Host Aggregates for more details.
#[derive(Parser)]
pub struct AvailabilityZoneCommand {
    /// subcommand
    #[command(subcommand)]
    command: AvailabilityZoneCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AvailabilityZoneCommands {
    List(list::AvailabilityZonesCommand),
    ListDetail(list_detail::AvailabilityZonesCommand),
}

impl AvailabilityZoneCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AvailabilityZoneCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            AvailabilityZoneCommands::ListDetail(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
        }
    }
}
