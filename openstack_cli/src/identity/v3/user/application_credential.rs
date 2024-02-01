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

//! Identity User Access Credentials commands
//!

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod create;
mod delete;
mod list;
mod show;

/// Identity Application Credentials
#[derive(Args, Clone, Debug)]
// #[command(args_conflicts_with_subcommands = true)]
pub struct ApplicationCredentialArgs {
    #[command(subcommand)]
    command: ApplicationCredentialCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum ApplicationCredentialCommands {
    /// Creates an application credential for a user on
    /// the project to which the current token is scoped.
    #[command(about = "Create application credential")]
    Create(create::ApplicationCredentialArgs),
    /// Delete an application credential.
    #[command(about = "Delete application credential")]
    Delete(delete::ApplicationCredentialArgs),
    /// List all application credentials for a user.
    #[command(about = "List application credentials")]
    List(list::ApplicationCredentialsArgs),
    /// Show details of an application credential.
    #[command(about = "Show application credential details")]
    Show(show::ApplicationCredentialArgs),
}

pub struct ApplicationCredentialCommand {
    pub args: ApplicationCredentialArgs,
}

impl OSCCommand for ApplicationCredentialCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            ApplicationCredentialCommands::Create(args) => {
                Ok(Box::new(create::ApplicationCredentialCmd {
                    args: args.clone(),
                }))
            }
            ApplicationCredentialCommands::Delete(args) => {
                Ok(Box::new(delete::ApplicationCredentialCmd {
                    args: args.clone(),
                }))
            }
            ApplicationCredentialCommands::List(args) => {
                Ok(Box::new(list::ApplicationCredentialsCmd {
                    args: args.clone(),
                }))
            }
            ApplicationCredentialCommands::Show(args) => {
                Ok(Box::new(show::ApplicationCredentialCmd {
                    args: args.clone(),
                }))
            }
        }
    }
}
