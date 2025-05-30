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

pub mod auth;
pub mod credential;
pub mod domain;
pub mod endpoint;
pub mod group;
pub mod limit;
pub mod os_ep_filter;
pub mod os_federation;
pub mod project;
pub mod region;
pub mod registered_limit;
pub mod role;
pub mod role_assignment;
pub mod role_inference;
pub mod service;
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
    Auth(auth::AuthCommand),
    AccessRule(user::access_rule::AccessRuleCommand),
    ApplicationCredential(user::application_credential::ApplicationCredentialCommand),
    Credential(credential::CredentialCommand),
    Domain(domain::DomainCommand),
    Endpoint(endpoint::EndpointCommand),
    EndpointFilter(os_ep_filter::EndpointFilterCommand),
    Federation(os_federation::FederationCommand),
    Group(group::GroupCommand),
    Limit(limit::LimitCommand),
    Project(project::ProjectCommand),
    Region(region::RegionCommand),
    RegisteredLimit(Box<registered_limit::RegisteredLimitCommand>),
    Role(role::RoleCommand),
    RoleAssignment(role_assignment::RoleAssignmentCommand),
    RoleInference(role_inference::RoleInferenceCommand),
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
            IdentityCommands::Auth(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::AccessRule(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::ApplicationCredential(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            IdentityCommands::Credential(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Domain(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Endpoint(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::EndpointFilter(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Federation(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Group(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Limit(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Project(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Region(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::RegisteredLimit(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::RoleAssignment(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::RoleInference(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Role(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::Service(cmd) => cmd.take_action(parsed_args, session).await,
            IdentityCommands::User(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
