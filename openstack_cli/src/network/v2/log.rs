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

//! Log commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod log;
mod loggable_resource;

/// Logging
///
/// Log resource
///
/// The logging extension lists, creates, shows information for, and updates log resource.
#[derive(Parser)]
pub struct LogCommand {
    /// subcommand
    #[command(subcommand)]
    command: LogCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LogCommands {
    Log(log::LogCommand),
    LoggableResource(loggable_resource::LoggableResourceCommand),
}

impl LogCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            LogCommands::Log(cmd) => cmd.take_action(parsed_args, session).await,
            LogCommands::LoggableResource(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
