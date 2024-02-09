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

//! Identity v3 API commands
use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod endpoint;
mod os_federation;
mod project;
mod region;
mod service;
mod user;

/// Identity (Keystone) commands
#[derive(Parser)]
pub struct IdentityCommand {
    /// subcommand
    #[command(subcommand)]
    command: IdentityCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum IdentityCommands {
    AccessRule(user::access_rule::AccessRuleCommand),
    ApplicationCredential(user::application_credential::ApplicationCredentialCommand),
    Endpoint(endpoint::EndpointCommand),
    Federation(os_federation::FederationCommand),
    Project(project::ProjectCommand),
    Region(region::RegionCommand),
    Service(service::ServiceCommand),
    User(user::UserCommand),
}

impl IdentityCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            IdentityCommands::AccessRule(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::ApplicationCredential(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            IdentityCommands::Endpoint(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Federation(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Project(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Region(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Service(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::User(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
