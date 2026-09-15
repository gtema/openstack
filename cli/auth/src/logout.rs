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

//! Revoke the current token and clear the local auth cache
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;
use structable::{StructTable, StructTableOptions};

/// Revoke the current token at the cloud and clear the local auth cache.
///
/// This command never forces a fresh login: it operates on whatever token is
/// already cached for the current cloud (never prompting for credentials).
/// If nothing is cached, it is a harmless no-op.
#[derive(Debug, Parser)]
pub struct LogoutCommand {
    /// Only clear the local auth cache; do not contact the cloud to revoke
    /// the token. Useful when the cloud is unreachable or the token is
    /// already known to be dead. Equivalent to `osc auth cache clear`.
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub local: bool,
}

/// Result of an `osc auth logout` invocation.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct LogoutResult {
    /// Whether a token was revoked at the cloud (`false` if `--local` was
    /// used, or if there was nothing to revoke).
    #[structable()]
    pub revoked: bool,

    /// Whether the local auth cache was cleared.
    #[structable()]
    pub cache_cleared: bool,

    /// On-disk auth cache file that was cleared, if the cache was enabled.
    #[structable(optional)]
    pub cache_file: Option<String>,
}

impl LogoutCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Logout");

        let op = OutputProcessor::from_args(parsed_args, Some("auth"), Some("logout"));

        let cache_file = client
            .get_auth_cache_file()
            .map(|p| p.display().to_string());

        let revoked = if self.local {
            client.clear_auth_cache();
            false
        } else {
            client.revoke_current_token().await?
        };

        let result = LogoutResult {
            revoked,
            cache_cleared: true,
            cache_file,
        };

        match op.target {
            OutputFor::Human => {
                op.output_human(&result)?;
            }
            _ => {
                op.output_machine(serde_json::to_value(&result)?)?;
            }
        }
        op.show_command_hint()?;
        Ok(())
    }
}
