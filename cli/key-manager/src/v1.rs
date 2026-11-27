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

//! Key-manager (Barbican) API v1 command

use clap::{Parser, Subcommand};

use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::{AsyncOpenStack, types::ServiceType};

// pub mod container;
// pub mod order;
// pub mod project_quota;
// pub mod quota;
// pub mod secret;
// pub mod secret_store;
// pub mod transport_key;
// pub mod version;

/// Key-manager (Barbican) service operations
#[derive(Parser)]
pub struct KeyManagerCommand {
    /// Key-manager service resource
    #[command(subcommand)]
    command: KeyManagerCommands,
}

/// Key-manager resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum KeyManagerCommands {}

impl KeyManagerCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        _parsed_args: &C,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session
            .discover_service_endpoint(&ServiceType::KeyManager)
            .await?;

        // match &self.command {
        //     KeyManagerCommands::Container(cmd) => cmd.take_action(parsed_args, session).await,
        //     KeyManagerCommands::Order(cmd) => cmd.take_action(parsed_args, session).await,
        //     KeyManagerCommands::ProjectQuota(cmd) => {
        //         cmd.take_action(parsed_args, session).await
        //     }
        //     KeyManagerCommands::Quota(cmd) => cmd.take_action(parsed_args, session).await,
        //     KeyManagerCommands::Secret(cmd) => cmd.take_action(parsed_args, session).await,
        //     KeyManagerCommands::SecretStore(cmd) => {
        //         cmd.take_action(parsed_args, session).await
        //     }
        //     KeyManagerCommands::TransportKey(cmd) => {
        //         cmd.take_action(parsed_args, session).await
        //     }
        //     KeyManagerCommands::Version(cmd) => cmd.take_action(parsed_args, session).await,
        // }
        Ok(())
    }
}
