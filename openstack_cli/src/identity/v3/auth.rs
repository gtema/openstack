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

//! Identity Auth commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod catalog;
mod domain;
mod os_federation;
mod project;
mod system;
mod token;

/// Identity Auth commands
///
/// The Identity service generates tokens in exchange for authentication credentials. A token
/// represents the authenticated identity of a user and, optionally, grants authorization on a
/// specific project, domain, or the deployment system.
///
/// The body of an authentication request must include a payload that specifies the authentication
/// methods, which are normally just password or token, the credentials, and, optionally, the
/// authorization scope. You can scope a token to a project, domain, the deployment system, or the
/// token can be unscoped. You cannot scope a token to multiple scope targets.
///
/// Tokens have IDs, which the Identity API returns in the X-Subject-Token response header.
///
/// In the case of multi-factor authentication (MFA) more than one authentication method needs to
/// be supplied to authenticate. As of v3.12 a failure due to MFA rules only partially being met
/// will result in an auth receipt ID being returned in the response header Openstack-Auth-Receipt,
/// and a response body that details the receipt itself and the missing authentication methods.
/// Supplying the auth receipt ID in the Openstack-Auth-Receipt header in a follow-up
/// authentication request, with the missing authentication methods, will result in a valid token
/// by reusing the successful methods from the first request. This allows MFA authentication to be
/// a multi-step process.
///
/// After you obtain an authentication token, you can:
///
/// - Make REST API requests to other OpenStack services. You supply the ID of your
///   authentication token in the X-Auth-Token request header.
///
/// - Validate your authentication token and list the domains, projects, roles, and endpoints
///   that your token gives you access to.
///
/// - Use your token to request another token scoped for a different domain and project.
///
/// - Force the immediate revocation of a token.
///
/// - List revoked public key infrastructure (PKI) tokens.
///

#[derive(Parser)]
pub struct AuthCommand {
    /// subcommand
    #[command(subcommand)]
    command: AuthCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum AuthCommands {
    Catalog(catalog::CatalogCommand),
    Domain(domain::DomainCommand),
    Federation(os_federation::OsFederationCommand),
    Project(project::ProjectCommand),
    System(system::SystemCommand),
    Token(token::TokenCommand),
}

impl AuthCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            AuthCommands::Catalog(cmd) => cmd.take_action(parsed_args, session).await,
            AuthCommands::Domain(cmd) => cmd.take_action(parsed_args, session).await,
            AuthCommands::Federation(cmd) => cmd.take_action(parsed_args, session).await,
            AuthCommands::Project(cmd) => cmd.take_action(parsed_args, session).await,
            AuthCommands::System(cmd) => cmd.take_action(parsed_args, session).await,
            AuthCommands::Token(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
