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

//! Placement `reshaper` command with subcommands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create_134;
pub mod create_138;

/// Reshaper
#[derive(Parser)]
pub struct ReshaperCommand {
    #[command(subcommand)]
    command: ReshaperCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ReshaperCommands {
    #[command(visible_alias = "create")]
    Create138(create_138::ReshaperCommand),
    Create134(create_134::ReshaperCommand),
}

impl ReshaperCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ReshaperCommands::Create138(cmd) => cmd.take_action(parsed_args, session).await,
            ReshaperCommands::Create134(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
