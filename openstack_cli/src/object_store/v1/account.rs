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

mod set;
mod show;

#[derive(Args, Clone, Debug)]
pub struct AccountArgs {
    #[command(subcommand)]
    command: AccountCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum AccountCommands {
    Show(show::AccountArgs),
    Set(set::AccountArgs),
}

pub struct AccountCommand {
    pub args: AccountArgs,
}

impl OSCCommand for AccountCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AccountCommands::Show(args) => Ok(Box::new(show::AccountCmd { args: args.clone() })),
            AccountCommands::Set(args) => Ok(Box::new(set::AccountCmd { args: args.clone() })),
        }
    }
}
