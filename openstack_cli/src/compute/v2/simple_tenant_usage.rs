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

//! Tenant usage

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod list;
pub mod show;

/// Usage reports (os-simple-tenant-usage)
///
/// Reports usage statistics of compute and storage resources periodically for an individual tenant
/// or all tenants. The usage statistics will include all instances’ CPU, memory and local disk
/// during a specific period.
///
/// **Warning**
///
/// The os-simple-tenant-usage will report usage statistics based on the latest flavor that is
/// configured in the virtual machine (VM), and ignoring stop, pause, and other events that might
/// have happened with the VM. Therefore, it uses the time the VM existed in the cloud environment
/// to execute the usage accounting.
///
/// More information can be found at
/// http://eavesdrop.openstack.org/meetings/nova/2020/nova.2020-12-03-16.00.log.txt, and
/// https://review.opendev.org/c/openstack/nova/+/711113
///
/// Microversion 2.40 added pagination (and next links) to the usage statistics via optional limit
/// and marker query parameters. If limit isn’t provided, the configurable max_limit will be used
/// which currently defaults to 1000. Older microversions will not accept these new paging query
/// parameters, but they will start to silently limit by max_limit.
///
/// ```text
///   /os-simple-tenant-usage?limit={limit}&marker={instance_uuid}
///   /os-simple-tenant-usage/{tenant_id}?limit={limit}&marker={instance_uuid}
/// ```
///
/// **Note**
///
/// A tenant’s usage statistics may span multiple pages when the number of instances exceeds limit,
/// and API consumers will need to stitch together the aggregate results if they still want totals
/// for all instances in a specific time window, grouped by tenant.
#[derive(Parser)]
pub struct SimpleTenantUsageCommand {
    /// subcommand
    #[command(subcommand)]
    command: SimpleTenantUsageCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum SimpleTenantUsageCommands {
    List(list::SimpleTenantUsagesCommand),
    Show(show::SimpleTenantUsageCommand),
}

impl SimpleTenantUsageCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            SimpleTenantUsageCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            SimpleTenantUsageCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
