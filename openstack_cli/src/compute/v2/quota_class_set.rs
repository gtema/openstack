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

//! Quota Class Set

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod set_21;
mod show;

/// Quota class sets (os-quota-class-sets)
///
/// Show, Create or Update the quotas for a Quota Class. Nova supports implicit ‘default’ Quota
/// Class only.
///
/// **Note**
///
/// Once a default limit is set via the default quota class via the API, that takes precedence over
/// any changes to that resource limit in the configuration options. In other words, once you’ve
/// changed things via the API, you either have to keep those synchronized with the configuration
/// values or remove the default limit from the database manually as there is no REST API for
/// removing quota class values from the database.
///
/// For Example: If you updated default quotas for instances, to 20, but didn’t change
/// quota_instances in your nova.conf, you’d now have default quota for instances as 20 for all
/// projects. If you then change quota_instances=5 in nova.conf, but didn’t update the default
/// quota class via the API, you’ll still have a default quota of 20 for instances regardless of
/// nova.conf. Refer: Quotas for more details.
///
/// **Warning**
///
/// There is a bug in the v2.1 API until microversion 2.49 and the legacy v2 compatible API which
/// does not return the server_groups and server_group_members quotas in GET and PUT
/// os-quota-class-sets API response, whereas the v2 API used to return those keys in the API
/// response. There is workaround to get the server_groups and server_group_members quotas using
/// “List Default Quotas For Tenant” API in Quota sets (os-quota-sets) but that is per project
/// quota. This issue is fixed in microversion 2.50, here onwards server_groups and
/// server_group_members keys are returned in API response body.
#[derive(Parser)]
pub struct QuotaClassSetCommand {
    /// subcommand
    #[command(subcommand)]
    command: QuotaClassSetCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum QuotaClassSetCommands {
    #[command(visible_alias = "set")]
    Set21(set_21::QuotaClassSetCommand),
    Show(show::QuotaClassSetCommand),
}

impl QuotaClassSetCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            QuotaClassSetCommands::Set21(cmd) => cmd.take_action(parsed_args, session).await,
            QuotaClassSetCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
