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

//! Block storage default VolumeType commands

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod delete;
mod list;
mod set_362;
mod show;

/// Default Volume Types (default-types)
///
/// Manage a default volume type for individual projects.
///
/// By default, a volume-create request that does not specify a volume-type will assign the
/// configured system default volume type to the volume. You can override this behavior on a
/// per-project basis by setting a different default volume type for any project.
///
/// Available in microversion 3.62 or higher.
///
/// NOTE: The default policy for list API is system admin so you would require a system scoped
/// token to access it.
#[derive(Parser)]
pub struct DefaultTypeCommand {
    /// subcommand
    #[command(subcommand)]
    command: DefaultTypeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DefaultTypeCommands {
    Delete(delete::DefaultTypeCommand),
    List(list::DefaultTypesCommand),
    #[command(visible_alias = "set")]
    Set362(set_362::DefaultTypeCommand),
    Show(show::DefaultTypeCommand),
}

impl DefaultTypeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            DefaultTypeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            DefaultTypeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            DefaultTypeCommands::Set362(cmd) => cmd.take_action(parsed_args, session).await,
            DefaultTypeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
