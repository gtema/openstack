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

//! DNS API v2 command

use clap::{Parser, Subcommand};

use openstack_sdk::{types::ServiceType, AsyncOpenStack};

use crate::{Cli, OpenStackCliError};

pub mod limit;
pub mod quota;
pub mod recordset;
pub mod reverse;
pub mod zone;

/// DNS service (Designate) operations
#[derive(Parser)]
pub struct DnsCommand {
    /// Dns service resource
    #[command(subcommand)]
    command: DnsCommands,
}

/// Dns resources commands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum DnsCommands {
    Limit(Box<limit::LimitCommand>),
    Quota(Box<quota::QuotaCommand>),
    Recordset(Box<recordset::RecordsetCommand>),
    Reverse(Box<reverse::ReverseCommand>),
    Zone(Box<zone::ZoneCommand>),
}

impl DnsCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        session.discover_service_endpoint(&ServiceType::Dns).await?;

        match &self.command {
            DnsCommands::Limit(cmd) => cmd.take_action(parsed_args, session).await,
            DnsCommands::Quota(cmd) => cmd.take_action(parsed_args, session).await,
            DnsCommands::Recordset(cmd) => cmd.take_action(parsed_args, session).await,
            DnsCommands::Reverse(cmd) => cmd.take_action(parsed_args, session).await,
            DnsCommands::Zone(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
