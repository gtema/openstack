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

pub mod volume;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::block_storage::v3::volume::{VolumeArgs, VolumeCommand};
use crate::{OSCCommand, OpenStackCliError};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct BlockStorageSrvArgs {
    /// BlockStorage API microversion
    #[arg(long, env = "OS_VOLUME_API_VERSION")]
    os_volume_api_version: Option<String>,
    /// BlockStorage service resource
    #[command(subcommand)]
    command: BlockStorageSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum BlockStorageSrvCommands {
    /// Volume commands
    Volume(VolumeArgs),
}

pub struct BlockStorageSrvCommand {
    pub args: BlockStorageSrvArgs,
}

impl OSCCommand for BlockStorageSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            BlockStorageSrvCommands::Volume(args) => {
                VolumeCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
