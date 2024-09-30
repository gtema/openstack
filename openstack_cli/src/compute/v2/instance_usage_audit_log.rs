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

//! Server usage audit log commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod list;
mod show;

/// Server usage audit log (os-instance-usage-audit-log)
///
/// Audit server usage of the cloud. This API is dependent on the instance_usage_audit
/// configuration option being set on all compute hosts where usage auditing is required.
///
/// Policy defaults enable only users with the administrative role to perform all
/// os-instance-usage-audit-log related operations. Cloud providers can change these permissions
/// through the policy.json file.
#[derive(Parser)]
pub struct InstanceUsageAuditLogCommand {
    /// subcommand
    #[command(subcommand)]
    command: InstanceUsageAuditLogCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum InstanceUsageAuditLogCommands {
    List(list::InstanceUsageAuditLogsCommand),
    Show(show::InstanceUsageAuditLogCommand),
}

impl InstanceUsageAuditLogCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            InstanceUsageAuditLogCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            InstanceUsageAuditLogCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
