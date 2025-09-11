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

pub mod user;

/// Identity (Keystone) commands
///
/// The Identity service generates authentication tokens that permit access to the OpenStack
/// services REST APIs. Clients obtain this token and the URL endpoints for other service APIs by
/// supplying their valid credentials to the authentication service.
///
/// Each time you make a REST API request to an OpenStack service, you supply your authentication
/// token in the X-Auth-Token request header.
///
/// Like most OpenStack projects, OpenStack Identity protects its APIs by defining policy rules
/// based on a role-based access control (RBAC) approach.
///
/// The Identity service configuration file sets the name and location of a JSON policy file that
/// stores these rules.
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
            IdentityCommands::User(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
