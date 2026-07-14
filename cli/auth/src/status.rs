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

//! Show current authentication status without requiring a fresh authorization
use chrono::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;
use openstack_sdk::auth::AuthState as SdkAuthState;
use structable::{StructTable, StructTableOptions};

/// Show current authentication status.
///
/// This command reports the currently known (locally cached) authentication
/// state for the connection: scope, token validity/expiry, and the on-disk
/// cache file used, without forcing a token renewal.
///
/// This command does not require a fully valid/unexpired authorization to run.
#[derive(Debug, Parser)]
pub struct StatusCommand {}

/// Authentication status information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct Status {
    /// Whether a token is present at all.
    #[structable()]
    pub authenticated: bool,

    /// Human readable auth scope (unscoped/project/domain/system) with name/id.
    #[structable(optional)]
    pub scope: Option<String>,

    /// Token validity state.
    #[structable(optional)]
    pub state: Option<String>,

    /// Token expiration time.
    #[structable(optional)]
    pub expires_at: Option<DateTime<Utc>>,

    /// On-disk auth cache file in use, if the cache is enabled.
    #[structable(optional)]
    pub cache_file: Option<String>,
}

impl StatusCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show auth status");

        let op = OutputProcessor::from_args(parsed_args, Some("auth"), Some("status"));

        let auth_info = client.get_auth_info();
        let auth_state = client.get_auth_state(None);
        let cache_file = client
            .get_auth_cache_file()
            .map(|p| p.display().to_string());

        let scope = auth_info.as_ref().map(|info| {
            if let Some(project) = &info.token.project {
                format!(
                    "project: {}",
                    project
                        .name
                        .clone()
                        .or_else(|| project.id.clone())
                        .unwrap_or_default()
                )
            } else if let Some(domain) = &info.token.domain {
                format!(
                    "domain: {}",
                    domain
                        .name
                        .clone()
                        .or_else(|| domain.id.clone())
                        .unwrap_or_default()
                )
            } else if let Some(system) = &info.token.system {
                format!("system: {:?}", system)
            } else {
                "unscoped".to_string()
            }
        });

        let status = Status {
            authenticated: auth_info.is_some(),
            scope,
            state: auth_state.map(|s| match s {
                SdkAuthState::Valid => "valid".to_string(),
                SdkAuthState::Expired => "expired".to_string(),
                SdkAuthState::AboutToExpire => "about to expire".to_string(),
                SdkAuthState::Unset => "unset".to_string(),
            }),
            expires_at: auth_info.as_ref().map(|info| info.token.expires_at),
            cache_file,
        };

        match op.target {
            OutputFor::Human => {
                op.output_human(&status)?;
            }
            _ => {
                op.output_machine(serde_json::to_value(&status)?)?;
            }
        }
        op.show_command_hint()?;
        Ok(())
    }
}
