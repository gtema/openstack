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

use clap::{Args, Subcommand};

use crate::{OSCCommand, OpenStackCliError};

use openstack_sdk::AsyncOpenStack;

mod image;
mod images;
mod member;
mod members;

#[derive(Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct SchemaArgs {
    #[command(subcommand)]
    command: SchemaCommands,
}

#[derive(Subcommand, Clone)]
pub enum SchemaCommands {
    /// Show Image Schema
    Image(image::ImageArgs),
    /// Show Images Schema
    Images(images::ImagesArgs),
    /// Show Member Schema
    Member(member::MemberArgs),
    /// Show Members Schema
    Members(members::MembersArgs),
}

pub struct SchemaCommand {
    pub args: SchemaArgs,
}

impl OSCCommand for SchemaCommand {
    fn get_subcommand(
        &self,
        session: &mut AsyncOpenStack,
    ) -> Result<Box<dyn OSCCommand + Send + Sync>, OpenStackCliError> {
        match &self.args.command {
            SchemaCommands::Image(args) => {
                image::ImageCommand { args: args.clone() }.get_subcommand(session)
            }
            SchemaCommands::Images(args) => {
                images::ImagesCommand { args: args.clone() }.get_subcommand(session)
            }
            SchemaCommands::Member(args) => {
                member::MemberCommand { args: args.clone() }.get_subcommand(session)
            }
            SchemaCommands::Members(args) => {
                members::MembersCommand { args: args.clone() }.get_subcommand(session)
            }
        }
    }
}
