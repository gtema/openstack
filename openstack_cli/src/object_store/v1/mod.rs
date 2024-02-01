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

pub mod account;
pub mod container;
pub mod object;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::object_store::v1::account::{AccountArgs, AccountCommand};
use crate::object_store::v1::container::{ContainerArgs, ContainerCommand};
use crate::object_store::v1::object::{ObjectArgs, ObjectCommand};
use crate::{OSCCommand, OpenStackCliError};

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ObjectStoreSrvArgs {
    /// Object store service resource
    #[command(subcommand)]
    command: ObjectStoreSrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum ObjectStoreSrvCommands {
    /// Account commands
    Account(AccountArgs),
    /// Container commands
    Container(ContainerArgs),
    /// Object commands
    Object(ObjectArgs),
}

pub struct ObjectStoreSrvCommand {
    pub args: ObjectStoreSrvArgs,
}

impl OSCCommand for ObjectStoreSrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ObjectStoreSrvCommands::Account(args) => {
                AccountCommand { args: args.clone() }.get_subcommand(session)
            }
            ObjectStoreSrvCommands::Container(args) => {
                ContainerCommand { args: args.clone() }.get_subcommand(session)
            }
            ObjectStoreSrvCommands::Object(args) => {
                ObjectCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
