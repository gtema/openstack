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

//! Clear the on-disk auth cache
use std::io::IsTerminal;

use clap::Parser;
use dialoguer::Confirm;
use eyre::eyre;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk::AsyncOpenStack;
use openstack_sdk_core::state::State;
use structable::{StructTable, StructTableOptions};

/// Clear the on-disk auth cache.
///
/// Without `--all`, only the cache file(s) for the *current* cloud are
/// removed (never contacts the network). Equivalent to `osc auth logout
/// --local`.
///
/// With `--all`, every cached credential under `~/.osc` is removed,
/// regardless of cloud/profile, and no cloud connection is required at all.
#[derive(Debug, Parser)]
#[command(alias = "drop")]
pub struct ClearCommand {
    /// Clear cached credentials for every cloud/profile, not just the
    /// current one. Destructive across profiles: prompts for confirmation
    /// unless `--yes` is given.
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub all: bool,

    /// Suppress the confirmation prompt for `--all`.
    #[arg(short = 'y', long, action=clap::ArgAction::SetTrue)]
    pub yes: bool,
}

/// Result of an `osc auth cache clear` invocation.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct CacheClearResult {
    /// Cache files that were removed.
    #[structable(serialize)]
    pub removed: Vec<String>,

    /// Entries under the cache directory that could not be removed.
    #[structable(serialize)]
    pub skipped: Vec<String>,
}

impl ClearCommand {
    /// Perform command action for the single-cloud (non-`--all`) case.
    pub async fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        // `--all` is handled entirely by `take_action_offline`, invoked by
        // the entry point before any cloud is resolved; this path only ever
        // runs for the current-cloud case.
        info!("Clear current auth cache");

        let op = OutputProcessor::from_args(parsed_args, Some("auth.cache"), Some("clear"));

        let removed = client
            .clear_auth_cache()
            .into_iter()
            .map(|p| p.display().to_string())
            .collect();

        let result = CacheClearResult {
            removed,
            skipped: Vec::new(),
        };

        output_result(&op, &result)?;
        op.show_command_hint()?;
        Ok(())
    }

    /// Client-free, cloud-independent path for `--all`. Called directly by
    /// `openstack_cli`'s entry point before any cloud config is resolved.
    pub fn take_action_offline<C: CliArgs>(
        &self,
        parsed_args: &C,
    ) -> Result<(), OpenStackCliError> {
        let op = OutputProcessor::from_args(parsed_args, Some("auth.cache"), Some("clear"));

        let base_dir = State::default_base_dir();
        let (preview_removed, _preview_skipped) = {
            // Peek at what's there without deleting yet, so the prompt can
            // report a count.
            let entries = std::fs::read_dir(&base_dir)
                .map(|rd| rd.flatten().count())
                .unwrap_or(0);
            (entries, 0)
        };

        if !self.yes {
            if std::io::stdin().is_terminal() {
                let confirmed = Confirm::new()
                    .with_prompt(format!(
                        "This will delete {preview_removed} cached credential file(s) in {}, \
                         affecting all clouds and profiles. Other open sessions will need to \
                         authenticate again. Continue?",
                        base_dir.display()
                    ))
                    .default(false)
                    .interact()
                    .map_err(|err| eyre!(err.to_string()))?;
                if !confirmed {
                    return Err(eyre!("aborted").into());
                }
            } else {
                return Err(eyre!(
                    "refusing to clear all cached credentials non-interactively without --yes"
                )
                .into());
            }
        }

        let (removed, skipped) = State::clear_all_cache_files(&base_dir);
        let result = CacheClearResult {
            removed: removed
                .into_iter()
                .map(|p| p.display().to_string())
                .collect(),
            skipped: skipped
                .into_iter()
                .map(|p| p.display().to_string())
                .collect(),
        };

        output_result(&op, &result)?;
        op.show_command_hint()?;
        Ok(())
    }
}

fn output_result(op: &OutputProcessor, result: &CacheClearResult) -> Result<(), OpenStackCliError> {
    match op.target {
        OutputFor::Human => {
            op.output_human(result)?;
        }
        _ => {
            op.output_machine(serde_json::to_value(result)?)?;
        }
    }
    Ok(())
}
