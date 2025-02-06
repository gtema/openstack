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

//! Metering commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod metering_label;
pub mod metering_label_rule;

/// Metering labels and rules  (metering-labels, metering-label-rules)
///
/// Creates, modifies, and deletes OpenStack Layer3 metering labels and rules.
#[derive(Parser)]
pub struct MeteringCommand {
    /// subcommand
    #[command(subcommand)]
    command: MeteringCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum MeteringCommands {
    #[command(visible_alias = "label")]
    MeteringLabel(metering_label::MeteringLabelCommand),
    #[command(visible_alias = "label-rule")]
    MeteringLabelRule(metering_label_rule::MeteringLabelRuleCommand),
}

impl MeteringCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            MeteringCommands::MeteringLabel(cmd) => cmd.take_action(parsed_args, session).await,
            MeteringCommands::MeteringLabelRule(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
