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

//! Container infrastructure cluster management
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
pub mod show;

/// Manage Cluster Templates
///
/// Lists, creates, shows details for, updates, and deletes Cluster Templates.
#[derive(Parser)]
pub struct ClustertemplateCommand {
    /// subcommand
    #[command(subcommand)]
    command: ClustertemplateCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ClustertemplateCommands {
    Create(Box<create::ClustertemplateCommand>),
    Delete(Box<delete::ClustertemplateCommand>),
    List(Box<list::ClustertemplatesCommand>),
    Show(Box<show::ClustertemplateCommand>),
}

impl ClustertemplateCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ClustertemplateCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ClustertemplateCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ClustertemplateCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ClustertemplateCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
