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

//! Domain configuration group option

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod default;
pub mod delete;
pub mod set;
pub mod show;

/// Domain Group Option
///
/// A domain group option is a configuration option of the domain specific driver.
#[derive(Parser)]
pub struct OptionCommand {
    /// subcommand
    #[command(subcommand)]
    command: OptionCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum OptionCommands {
    Default(default::OptionCommand),
    Delete(delete::OptionCommand),
    Set(set::OptionCommand),
    Show(show::OptionCommand),
}

impl OptionCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            OptionCommands::Default(cmd) => cmd.take_action(parsed_args, session).await,
            OptionCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            OptionCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            OptionCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
