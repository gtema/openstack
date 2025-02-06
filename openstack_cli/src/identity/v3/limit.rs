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

//! Identity Unified Limits commands

use clap::{Parser, Subcommand};

use openstack_sdk::AsyncOpenStack;

use crate::{Cli, OpenStackCliError};

pub mod create;
pub mod delete;
pub mod list;
/// Limit model
pub mod model {
    pub mod get;
}
pub mod set;
pub mod show;

/// Unified Limits
///
/// In OpenStack, a quota system mainly contains two parts: limit and usage. The Unified limits in
/// Keystone is a replacement of the limit part. It contains two kinds of resources: Registered
/// Limit and Limit. A registered limit is a default limit. It is usually created by the services
/// which are registered in Keystone. A limit is the limit that override the registered limit for
/// each project.
#[derive(Parser)]
pub struct LimitCommand {
    #[command(subcommand)]
    command: LimitCommands,
}

/// Supported subcommands
#[allow(missing_docs)]
#[derive(Subcommand)]
pub enum LimitCommands {
    Create(create::LimitCommand),
    Delete(delete::LimitCommand),
    List(list::LimitsCommand),
    Model(model::get::ModelCommand),
    Set(set::LimitCommand),
    Show(show::LimitCommand),
}

impl LimitCommand {
    /// Perform command action
    pub async fn take_action(
        &self,
        parsed_args: &Cli,
        session: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            LimitCommands::Create(cmd) => cmd.take_action(parsed_args, session).await,
            LimitCommands::Delete(cmd) => cmd.take_action(parsed_args, session).await,
            LimitCommands::List(cmd) => cmd.take_action(parsed_args, session).await,
            LimitCommands::Model(cmd) => cmd.take_action(parsed_args, session).await,
            LimitCommands::Set(cmd) => cmd.take_action(parsed_args, session).await,
            LimitCommands::Show(cmd) => cmd.take_action(parsed_args, session).await,
        }
    }
}
