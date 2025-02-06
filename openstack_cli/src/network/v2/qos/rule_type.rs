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

//! QoS Rule Type commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod list;
pub mod show;

/// QoS rule types
///
/// Lists and shows information for QoS rule types available in current deployment.
///
/// Rule type details
///
/// The qos-rule-type-details extension adds the drivers attribute to QoS rule types. The drivers
/// attribute’s value is a list of driver objects. Each driver object represents a loaded backend
/// QoS driver and includes the driver’s name as well as a list of its supported_parameters and
/// acceptable values.
#[derive(Parser)]
pub struct RuleTypeCommand {
    /// subcommand
    #[command(subcommand)]
    command: RuleTypeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum RuleTypeCommands {
    List(Box<list::RuleTypesCommand>),
    Show(Box<show::RuleTypeCommand>),
}

impl RuleTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            RuleTypeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            RuleTypeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
