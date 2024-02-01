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

pub mod image;
pub mod schema;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::image::v2::image::{ImageArgs, ImageCommand};
use crate::image::v2::schema::{SchemaArgs, SchemaCommand};
use crate::{OSCCommand, OpenStackCliError};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImageSrvArgs {
    /// Image service resource
    #[command(subcommand)]
    command: ImageSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum ImageSrvCommands {
    /// Image commands
    Image(ImageArgs),
    /// Schema commands
    Schema(SchemaArgs),
}

pub struct ImageSrvCommand {
    pub args: ImageSrvArgs,
}

impl OSCCommand for ImageSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImageSrvCommands::Image(args) => {
                ImageCommand { args: args.clone() }.get_subcommand(session)
            }
            ImageSrvCommands::Schema(args) => {
                SchemaCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
