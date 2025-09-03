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

//! Block storage Volume `quota-class-set` commands

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod set;
pub mod show;

/// Quota class set extension (os-quota-class-sets)
///
/// Administrators only, depending on policy settings.
///
/// Shows and updates quota classes for a project.
#[derive(Parser)]
pub struct QuotaClassSetCommand {
    /// subcommand
    #[command(subcommand)]
    command: QuotaClassSetCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QuotaClassSetCommands {
    Set(Box<set::QuotaClassSetCommand>),
    Show(Box<show::QuotaClassSetCommand>),
}

impl QuotaClassSetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QuotaClassSetCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaClassSetCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
