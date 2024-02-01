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

mod get;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ImagesArgs {
    #[command(subcommand)]
    command: ImagesCommands,
}

#[derive(Subcommand, Clone)]
pub enum ImagesCommands {
    /// Show Images Schema
    Show(get::ImagesArgs),
}

pub struct ImagesCommand {
    pub args: ImagesArgs,
}

impl OSCCommand for ImagesCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ImagesCommands::Show(args) => Ok(Box::new(get::ImagesCmd { args: args.clone() })),
        }
    }
}
