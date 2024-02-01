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

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod deactivate;
mod delete;
mod file {
    pub(super) mod download;
    pub(super) mod upload;
}
mod list;
mod patch;
mod reactivate;
mod show;

/// Image (Glance) commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageArgs {
    #[command(subcommand)]
    command: ImageCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImageCommands {
    List(Box<list::ImagesArgs>),
    Show(show::ImageArgs),
    Create(create::ImageArgs),
    Set(patch::ImageArgs),
    Download(Box<file::download::FileArgs>),
    Upload(file::upload::FileArgs),
    Delete(delete::ImageArgs),
    /// Deactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only.
    #[command(about = "Deactivate image")]
    Deactivate(deactivate::ImageArgs),
    /// Reactivates an image. (Since Image API v2.3)
    ///
    /// By default, this operation is restricted to administrators only
    #[command(about = "Reactivate image")]
    Reactivate(reactivate::ImageArgs),
}

pub struct ImageCommand {
    pub args: ImageArgs,
}

impl OSCCommand for ImageCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImageCommands::List(args) => Ok(Box::new(list::ImagesCmd {
                args: *args.clone(),
            })),
            ImageCommands::Show(args) => Ok(Box::new(show::ImageCmd { args: args.clone() })),
            ImageCommands::Set(args) => Ok(Box::new(patch::ImageCmd { args: args.clone() })),
            ImageCommands::Download(args) => Ok(Box::new(file::download::FileCmd {
                args: *args.clone(),
            })),
            ImageCommands::Upload(args) => {
                Ok(Box::new(file::upload::FileCmd { args: args.clone() }))
            }
            ImageCommands::Create(args) => Ok(Box::new(create::ImageCmd { args: args.clone() })),
            ImageCommands::Delete(args) => Ok(Box::new(delete::ImageCmd { args: args.clone() })),
            ImageCommands::Deactivate(args) => {
                Ok(Box::new(deactivate::ImageCmd { args: args.clone() }))
            }
            ImageCommands::Reactivate(args) => {
                Ok(Box::new(reactivate::ImageCmd { args: args.clone() }))
            }
        }
    }
}
