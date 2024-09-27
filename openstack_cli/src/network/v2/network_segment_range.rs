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

//! NetworkSegmentRange commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

mod create;
mod delete;
mod list;
mod set;
mod show;
mod tag;

/// Network Segment Ranges
///
/// The network segment range extension exposes the segment range management to be administered via
/// the Neutron API. It introduces the network-segment-range resource for tenant network segment
/// allocation. In addition, it introduces the ability for the administrator to control the segment
/// ranges globally or on a per-tenant basis.
///
/// Lists, shows details for, creates, updates, and deletes network segment ranges. The network
/// segment ranges API is admin-only.
#[derive(Parser)]
pub struct NetworkSegmentRangeCommand {
    /// subcommand
    #[command(subcommand)]
    command: NetworkSegmentRangeCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum NetworkSegmentRangeCommands {
    Create(Box<create::NetworkSegmentRangeCommand>),
    Delete(Box<delete::NetworkSegmentRangeCommand>),
    List(Box<list::NetworkSegmentRangesCommand>),
    Set(Box<set::NetworkSegmentRangeCommand>),
    Show(Box<show::NetworkSegmentRangeCommand>),
    Tag(Box<tag::TagCommand>),
}

impl NetworkSegmentRangeCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            NetworkSegmentRangeCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkSegmentRangeCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkSegmentRangeCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkSegmentRangeCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkSegmentRangeCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
            NetworkSegmentRangeCommands::Tag(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
