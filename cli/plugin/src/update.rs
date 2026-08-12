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

//! Update installed wasm auth plugin(s) to the latest registry version.

use std::io::IsTerminal;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::OutputProcessor;
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk_plugin_wasm::registry::{ProvenanceOutcome, UpdateOutcome};
use structable::{StructTable, StructTableOptions};

use crate::confirm;

/// Update installed, non-pinned wasm auth plugin(s) to the latest version
/// available in the registry index.
///
/// Provenance is re-verified fresh for every plugin considered, never reused
/// from a previous install — the same GitHub attestation check `osc plugin
/// install` performs. A plugin installed with an explicit `@version` is
/// "pinned" and is skipped by `--all` (and refused by name) unless
/// reinstalled explicitly via `osc plugin install <name>@<version>`.
#[derive(Debug, Parser)]
pub struct UpdateCommand {
    /// Plugin name to update. Omit and pass `--all` to update every
    /// installed, non-pinned plugin instead.
    #[arg(conflicts_with = "all")]
    pub name: Option<String>,

    /// Update every installed, non-pinned plugin.
    #[arg(long)]
    pub all: bool,

    /// Registry index URL to resolve against. The pinned default is never
    /// silently overridden by anything but this explicit flag.
    #[arg(long)]
    pub registry_url: Option<String>,

    /// Proceed without an interactive confirmation prompt for each update.
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Allow updating to a version whose provenance could not be verified.
    /// Loudly logged. A plugin whose new version fails verification is
    /// skipped when this is not given.
    #[arg(long)]
    pub allow_unsigned: bool,
}

/// The outcome of one plugin's update attempt.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct PluginUpdateResult {
    /// Plugin name.
    #[structable()]
    pub name: String,

    /// What happened: `updated`, `up-to-date`, `declined`, `skipped
    /// (pinned)`, or `not in registry index`.
    #[structable()]
    pub status: String,

    /// The version that was active before this update attempt.
    #[structable()]
    pub from_version: String,

    /// The version now active (equal to `from_version` unless `status` is
    /// `updated`).
    #[structable()]
    pub to_version: String,
}

impl UpdateCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Update installed wasm auth plugin(s)");

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("update"));

        if self.name.is_none() && !self.all {
            return Err(eyre::eyre!("update requires either a plugin name or --all").into());
        }
        if !self.yes && !std::io::stdin().is_terminal() {
            return Err(eyre::eyre!(
                "refusing to update without confirmation in a non-interactive context; pass --yes to proceed"
            )
            .into());
        }

        let registry_url = self
            .registry_url
            .as_deref()
            .unwrap_or(openstack_sdk_plugin_wasm::index::DEFAULT_REGISTRY_URL);
        let client = openstack_sdk_plugin_wasm::index::http_client().map_err(eyre::Report::from)?;

        let yes = self.yes;
        let allow_unsigned = self.allow_unsigned;
        let outcomes = openstack_sdk_plugin_wasm::registry::update(
            self.name.as_deref(),
            self.all,
            registry_url,
            &client,
            allow_unsigned,
            |pending| {
                if matches!(pending.provenance, ProvenanceOutcome::Unverified { .. }) && !allow_unsigned {
                    eprintln!(
                        "refusing to update {}@{} without provenance verification (pass --allow-unsigned to override); skipping",
                        pending.name, pending.version
                    );
                    return false;
                }
                let proceed = confirm::confirm_pending(pending, yes).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    false
                });
                if proceed && matches!(pending.provenance, ProvenanceOutcome::Unverified { .. }) {
                    confirm::warn_allow_unsigned(&pending.name, &pending.version);
                }
                proceed
            },
        )
        .await
        .map_err(eyre::Report::from)?;

        let data: Vec<serde_json::Value> = outcomes
            .into_iter()
            .map(|outcome| {
                let entry = match outcome {
                    UpdateOutcome::UpToDate { name, version } => PluginUpdateResult {
                        name,
                        status: "up-to-date".into(),
                        from_version: version.clone(),
                        to_version: version,
                    },
                    UpdateOutcome::Updated { name, from, to } => PluginUpdateResult {
                        name,
                        status: "updated".into(),
                        from_version: from,
                        to_version: to,
                    },
                    UpdateOutcome::Declined { name, version } => PluginUpdateResult {
                        name,
                        status: "declined".into(),
                        from_version: version.clone(),
                        to_version: version,
                    },
                    UpdateOutcome::SkippedPinned { name, version } => PluginUpdateResult {
                        name,
                        status: "skipped (pinned)".into(),
                        from_version: version.clone(),
                        to_version: version,
                    },
                    UpdateOutcome::NotInIndex { name } => PluginUpdateResult {
                        name,
                        status: "not in registry index".into(),
                        from_version: String::new(),
                        to_version: String::new(),
                    },
                };
                serde_json::to_value(entry)
            })
            .collect::<Result<_, _>>()?;

        op.output_list::<PluginUpdateResult>(data)
    }
}
