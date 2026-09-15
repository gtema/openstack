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

//! On-disk auth cache management (`osc auth cache ...`)
use clap::{Parser, Subcommand};

use openstack_cli_core::{
    cli::{CliArgs, ConnectionRequirements, ConnectionRequirementsProvider},
    error::OpenStackCliError,
};
use openstack_sdk::AsyncOpenStack;

pub mod clear;

/// On-disk auth cache management
#[derive(Debug, Parser)]
pub struct CacheCommand {
    /// Cache commands
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[allow(missing_docs)]
#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    Clear(clear::ClearCommand),
}

impl ConnectionRequirementsProvider for CacheCommands {
    fn connection_requirements(&self) -> ConnectionRequirements {
        // Every `cache` operation is purely local (memory/disk) and never
        // needs a live/valid session.
        ConnectionRequirements {
            needs_auth: false,
            renew: false,
        }
    }
}

impl CacheCommand {
    /// Returns `Some(&ClearCommand)` only when this is `cache clear --all`,
    /// which needs to run before any cloud is resolved (it is
    /// profile-independent). See `openstack_cli`'s entry point.
    pub fn as_offline_cache_clear_all(&self) -> Option<&clear::ClearCommand> {
        match &self.command {
            CacheCommands::Clear(cmd) if cmd.all => Some(cmd),
            CacheCommands::Clear(_) => None,
        }
    }

    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        match &self.command {
            CacheCommands::Clear(cmd) => cmd.take_action(parsed_args, client).await,
        }
    }
}
