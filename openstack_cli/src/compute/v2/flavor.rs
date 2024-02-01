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

//! Compute Flavor commands

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod add_tenant_access;
mod create_20;
mod create_21;
mod create_255;
mod delete;
mod extra_spec;
mod flavor_access;
mod list;
mod remove_tenant_access;
mod set;
mod show;

/// Flavor commands
///
/// Flavors are a way to describe the basic dimensions of a server
/// to be created including how much cpu, ram, and disk space are
/// allocated to a server built with this flavor.
#[derive(Args, Clone, Debug)]
pub struct FlavorArgs {
    #[command(subcommand)]
    command: FlavorCommands,
}

#[derive(Subcommand, Clone, Debug)]
pub enum FlavorCommands {
    Access(Box<flavor_access::FlavorAccessArgs>),
    #[command(visible_alias = "create")]
    Create255(Box<create_255::FlavorArgs>),
    Create21(Box<create_21::FlavorArgs>),
    Create20(Box<create_20::FlavorArgs>),
    Delete(Box<delete::FlavorArgs>),
    Extraspecs(Box<extra_spec::ExtraSpecsArgs>),
    List(Box<list::FlavorsArgs>),
    Set(Box<set::FlavorArgs>),
    Show(Box<show::FlavorArgs>),
}

pub struct FlavorCommand {
    /// Command arguments
    pub args: FlavorArgs,
}

impl OSCCommand for FlavorCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            FlavorCommands::Access(args) => flavor_access::FlavorAccessCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            FlavorCommands::Create20(args) => Ok(Box::new(create_20::FlavorCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Create21(args) => Ok(Box::new(create_21::FlavorCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Create255(args) => Ok(Box::new(create_255::FlavorCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Delete(args) => Ok(Box::new(delete::FlavorCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Extraspecs(args) => extra_spec::ExtraSpecsCommand {
                args: *args.clone(),
            }
            .get_subcommand(session),
            FlavorCommands::List(args) => Ok(Box::new(list::FlavorsCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Set(args) => Ok(Box::new(set::FlavorCmd {
                args: *args.clone(),
            })),
            FlavorCommands::Show(args) => Ok(Box::new(show::FlavorCmd {
                args: *args.clone(),
            })),
        }
    }
}
