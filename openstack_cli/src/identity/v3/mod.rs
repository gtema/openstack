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

pub mod project;
pub mod user;

use clap::{Args, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::identity::v3::project::{ProjectArgs, ProjectCommand};
use crate::identity::v3::user::access_rule::{AccessRuleArgs, AccessRuleCommand};
use crate::identity::v3::user::application_credential::{
    ApplicationCredentialArgs, ApplicationCredentialCommand,
};
use crate::identity::v3::user::{UserArgs, UserCommand};
use crate::{OSCCommand, OpenStackCliError};

/// Identity (Keystone) commands
#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct IdentitySrvArgs {
    /// Identity service resource
    #[command(subcommand)]
    command: IdentitySrvCommands,
}

#[derive(Clone, Subcommand)]
pub enum IdentitySrvCommands {
    AccessRule(AccessRuleArgs),
    ApplicationCredential(ApplicationCredentialArgs),
    Project(ProjectArgs),
    User(UserArgs),
}

pub struct IdentitySrvCommand {
    /// Command arguments
    pub args: IdentitySrvArgs,
}

impl OSCCommand for IdentitySrvCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            IdentitySrvCommands::AccessRule(args) => {
                AccessRuleCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::ApplicationCredential(args) => {
                ApplicationCredentialCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::Project(args) => {
                ProjectCommand { args: args.clone() }.get_subcommand(session)
            }
            IdentitySrvCommands::User(args) => {
                UserCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
