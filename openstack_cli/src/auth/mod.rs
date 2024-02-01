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

//! Authorization operations
//!
//!
pub mod login;
pub mod show;
use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{OSCCommand, OpenStackCliError};

/// Cloud Authentication operations
///
/// This command provides various authorization operations (login, show,
/// status, etc)
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct AuthArgs {
    /// Authentication commands
    #[command(subcommand)]
    pub(crate) command: AuthCommands,
}

#[derive(Clone, Subcommand)]
pub enum AuthCommands {
    Login(login::AuthArgs),
    Show(show::AuthArgs),
}

pub struct AuthCommand {
    pub args: AuthArgs,
}

impl OSCCommand for AuthCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            AuthCommands::Login(args) => Ok(Box::new(login::AuthCmd { args: args.clone() })),
            AuthCommands::Show(args) => Ok(Box::new(show::AuthCmd { args: args.clone() })),
        }
    }
}
