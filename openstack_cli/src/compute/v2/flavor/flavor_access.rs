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

//! Compute Flavor Access commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

use crate::compute::v2::flavor::add_tenant_access;
use crate::compute::v2::flavor::remove_tenant_access;

mod list;

/// Flavor access command
#[derive(Parser)]
pub struct FlavorAccessCommand {
    #[command(subcommand)]
    command: FlavorAccessCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum FlavorAccessCommands {
    /// Adds flavor access to a tenant and flavor.
    #[command(about = "Add Flavor Access To Tenant (addTenantAccess Action)")]
    Add(add_tenant_access::FlavorCommand),
    /// Lists flavor access information.
    #[command(about = "List Flavor Access Information For Given Flavor")]
    List(list::FlavorAccessesCommand),
    /// Removes flavor access from a tenant and flavor.
    #[command(about = "Remove Flavor Access From Tenant (removeTenantAccess Action)")]
    Remove(remove_tenant_access::FlavorCommand),
}

impl FlavorAccessCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            FlavorAccessCommands::Add(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorAccessCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            FlavorAccessCommands::Remove(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
