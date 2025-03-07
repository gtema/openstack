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

//! Block storage Volume QOS-SPEC commands

use clap::{Parser, Subcommand};

use crate::{Cli, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

pub mod associate;
pub mod association;
pub mod create;
pub mod delete;
pub mod delete_keys;
pub mod disassociate;
pub mod disassociate_all;
pub mod list;
pub mod set;
pub mod show;

/// Quality of service (QoS) specifications (qos-specs)
///
/// Administrators only, depending on policy settings.
///
/// Creates, lists, shows details for, associates, disassociates, sets keys, unsets keys, and
/// deletes quality of service (QoS) specifications.
#[derive(Parser)]
pub struct QosSpecCommand {
    /// subcommand
    #[command(subcommand)]
    command: QosSpecCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QosSpecCommands {
    Association(Box<association::AssociationCommand>),
    Associate(Box<associate::QosSpecCommand>),
    Create(Box<create::QosSpecCommand>),
    Delete(Box<delete::QosSpecCommand>),
    DeleteKeys(Box<delete_keys::QosSpecCommand>),
    Disassociate(Box<disassociate::QosSpecCommand>),
    DisassociateAll(Box<disassociate_all::QosSpecCommand>),
    List(Box<list::QosSpecsCommand>),
    Set(Box<set::QosSpecCommand>),
    Show(Box<show::QosSpecCommand>),
}

impl QosSpecCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QosSpecCommands::Association(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Associate(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::DeleteKeys(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Disassociate(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::DisassociateAll(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            QosSpecCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
