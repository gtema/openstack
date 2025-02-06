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

//! Identity Federation commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod identity_provider;
pub mod mapping;
pub mod service_provider;
/// IdP SAML2
pub mod saml2 {
    pub mod metadata;
}

/// OS-Federation
///
/// Provide the ability for users to manage Identity Providers (IdPs) and
/// establish a set of rules to map federation protocol attributes to Identity
/// API attributes.
#[derive(Parser)]
pub struct FederationCommand {
    #[command(subcommand)]
    command: FederationCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum FederationCommands {
    IdentityProvider(identity_provider::IdentityProviderCommand),
    Mapping(mapping::MappingCommand),
    ServiceProvider(service_provider::ServiceProviderCommand),
    Saml2Metadata(saml2::metadata::MetadataCommand),
}

impl FederationCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            FederationCommands::IdentityProvider(cmd) => {
                cmd.take_action(parsed_args, session).await
            }
            FederationCommands::Mapping(cmd) => cmd.take_action(parsed_args, session).await,
            FederationCommands::Saml2Metadata(cmd) => cmd.take_action(parsed_args, session).await,
            FederationCommands::ServiceProvider(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
