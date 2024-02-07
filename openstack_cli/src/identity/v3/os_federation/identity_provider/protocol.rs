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

//! Identity Protocol Protocol commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;

/// Identity provider protocols
///
/// A protocol entry contains information that dictates which mapping rules to
/// use for a given incoming request. An IdP may have multiple supported
/// protocols.
///
/// Required attributes:
///
///  - mapping_id (string): Indicates which mapping should be used to process
///  federated authentication requests.
///
/// Optional attributes:
///
///  - remote_id_attribute (string): Key to obtain the entity ID of the
///  Identity Provider from the HTTPD environment. For mod_shib, this would be
///  Shib-Identity-Provider. For mod_auth_openidc, this could be HTTP_OIDC_ISS.
///  For mod_auth_mellon, this could be MELLON_IDP. This overrides the default
///  value provided in keystone.conf.
#[derive(Parser)]
pub struct ProtocolCommand {
    #[command(subcommand)]
    command: ProtocolCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum ProtocolCommands {
    Create(create::ProtocolCommand),
    Delete(delete::ProtocolCommand),
    List(list::ProtocolsCommand),
    Set(set::ProtocolCommand),
    Show(show::ProtocolCommand),
}

impl ProtocolCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            ProtocolCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            ProtocolCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            ProtocolCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            ProtocolCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            ProtocolCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
