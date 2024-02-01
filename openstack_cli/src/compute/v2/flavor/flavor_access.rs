// Copyright 2024
//
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
use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

use crate::compute::v2::flavor::add_tenant_access;
use crate::compute::v2::flavor::remove_tenant_access;

mod list;

#[derive(Args, Clone, Debug)]
pub struct FlavorAccessArgs {
    #[command(subcommand)]
    command: FlavorAccessCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlavorAccessCommands {
    /// Adds flavor access to a tenant and flavor.
    #[command(about = "Add Flavor Access To Tenant (addTenantAccess Action)")]
    Add(add_tenant_access::FlavorArgs),
    /// Lists flavor access information.
    #[command(about = "List Flavor Access Information For Given Flavor")]
    List(list::FlavorAccesesArgs),
    /// Removes flavor access from a tenant and flavor.
    #[command(about = "Remove Flavor Access From Tenant (removeTenantAccess Action)")]
    Remove(remove_tenant_access::FlavorArgs),
}

pub struct FlavorAccessCommand {
    pub args: FlavorAccessArgs,
}

impl OSCCommand for FlavorAccessCommand {
    fn get_subcommand(
        &self,
        _: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            FlavorAccessCommands::Add(args) => Ok(Box::new(add_tenant_access::FlavorCmd {
                args: args.clone(),
            })),
            FlavorAccessCommands::List(args) => {
                Ok(Box::new(list::FlavorAccesesCmd { args: args.clone() }))
            }
            FlavorAccessCommands::Remove(args) => Ok(Box::new(remove_tenant_access::FlavorCmd {
                args: args.clone(),
            })),
        }
    }
}
