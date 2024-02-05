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

//! Glance `Image` resource commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod deactivate;
mod delete;
/// Image data operations
mod file {
    pub(super) mod download;
    pub(super) mod upload;
}
mod list;
mod patch;
mod reactivate;
mod show;

/// Image (Glance) commands
#[derive(Parser)]
pub struct ImageCommand {
    /// subcommand
    #[command(subcommand)]
    command: ImageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ImageCommands {
    Create(create::ImageCommand),
    /// Deactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only.
    #[command(about = "Deactivate image")]
    Deactivate(deactivate::ImageCommand),
    Delete(delete::ImageCommand),
    Download(Box<file::download::FileCommand>),
    List(Box<list::ImagesCommand>),
    /// Reactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only
    #[command(about = "Reactivate image")]
    Reactivate(reactivate::ImageCommand),
    Set(patch::ImageCommand),
    Show(show::ImageCommand),
    Upload(file::upload::FileCommand),
}

impl ImageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ImageCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Deactivate(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Download(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Reactivate(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            ImageCommands::Upload(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
