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

//! Install a wasm auth plugin, either from the registry or a local file.

use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use openstack_sdk_plugin_wasm::registry::ProvenanceOutcome;
use structable::{StructTable, StructTableOptions};

use crate::confirm;

/// Install a wasm auth plugin, from the registry by name or from a local
/// `.wasm` file.
///
/// `osc plugin install <name>` resolves `<name>` against the registry index
/// (latest version, or `<name>@<version>` to pin a specific one), downloads
/// it, verifies its checksum and — unless `--allow-unsigned` is given —
/// its GitHub artifact attestation, shows what was found, and asks for
/// confirmation (skippable with `--yes`).
///
/// `osc plugin install --file <path>` installs a local `.wasm` file instead.
/// A local file has no provenance to verify, so this always requires
/// `--allow-unsigned`. In both cases the plugin is loaded and its ABI is
/// validated before anything is written to the plugin directory, and
/// installing over an already-installed `name@version` fails unless
/// `--force` is given.
#[derive(Debug, Parser)]
pub struct InstallCommand {
    /// Plugin to install: `<name>` (latest) or `<name>@<version>` (pinned),
    /// resolved against the registry index. Omit when using `--file`.
    #[arg(required_unless_present = "file")]
    pub spec: Option<String>,

    /// Install from a local `.wasm` file instead of the registry. Local
    /// files have no provenance to verify, so this always requires
    /// `--allow-unsigned`.
    #[arg(long, conflicts_with = "spec")]
    pub file: Option<PathBuf>,

    /// Version to record a `--file` install under. Defaults to `0.0.0` when
    /// not given. Ignored for registry installs — use `<name>@<version>` in
    /// the positional argument instead.
    #[arg(long)]
    pub version: Option<String>,

    /// Replace an existing installation of the same `name@version`.
    #[arg(long)]
    pub force: bool,

    /// Registry index URL to resolve against. The pinned default is never
    /// silently overridden by anything but this explicit flag.
    #[arg(long)]
    pub registry_url: Option<String>,

    /// Proceed without an interactive confirmation prompt.
    #[arg(short = 'y', long)]
    pub yes: bool,

    /// Install even though the plugin's provenance could not be verified
    /// (required for `--file`, since a local file has no provenance to
    /// check). Loudly logged.
    #[arg(long)]
    pub allow_unsigned: bool,
}

/// Information about an installed plugin.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct InstalledPlugin {
    /// Plugin name (its `.wasm` file stem).
    #[structable()]
    pub name: String,

    /// Path the plugin was installed to.
    #[structable()]
    pub source: String,

    /// Auth method names this plugin supports.
    #[structable()]
    pub supported_methods: String,

    /// Guest ABI's declared Identity API version (major.minor).
    #[structable()]
    pub api_version: String,
}

impl InstallCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("install"));

        let plugin = if let Some(file) = &self.file {
            info!("Install wasm auth plugin from {}", file.display());
            if !self.allow_unsigned {
                return Err(eyre::eyre!(
                    "local --file installs have no provenance to verify; pass --allow-unsigned to install anyway"
                )
                .into());
            }
            let plugin = openstack_sdk_plugin_wasm::registry::install(
                file,
                self.version.as_deref(),
                self.force,
            )
            .map_err(eyre::Report::from)?;
            confirm::warn_allow_unsigned(
                plugin.name(),
                self.version
                    .as_deref()
                    .unwrap_or(openstack_sdk_plugin_wasm::registry::DEFAULT_VERSION),
            );
            plugin
        } else {
            let spec = self
                .spec
                .as_deref()
                .ok_or_else(|| eyre::eyre!("either a plugin name or --file is required"))?;
            let (name, version) = match spec.split_once('@') {
                Some((n, v)) => (n, Some(v)),
                None => (spec, None),
            };
            info!("Install wasm auth plugin {spec} from the registry");

            let registry_url = self
                .registry_url
                .as_deref()
                .unwrap_or(openstack_sdk_plugin_wasm::index::DEFAULT_REGISTRY_URL);
            let client =
                openstack_sdk_plugin_wasm::index::http_client().map_err(eyre::Report::from)?;
            let pending = openstack_sdk_plugin_wasm::registry::plan_remote_install(
                name,
                version,
                registry_url,
                &client,
            )
            .await
            .map_err(eyre::Report::from)?;

            if matches!(pending.provenance, ProvenanceOutcome::Unverified { .. })
                && !self.allow_unsigned
            {
                return Err(eyre::eyre!(
                    "refusing to install {}@{} without provenance verification (pass --allow-unsigned to override)",
                    pending.name,
                    pending.version
                )
                .into());
            }

            if !confirm::confirm_pending(&pending, self.yes)? {
                return Err(eyre::eyre!("installation cancelled").into());
            }
            if matches!(pending.provenance, ProvenanceOutcome::Unverified { .. }) {
                confirm::warn_allow_unsigned(&pending.name, &pending.version);
            }

            let pinned = version.is_some();
            openstack_sdk_plugin_wasm::registry::finalize_install(
                pending,
                self.allow_unsigned,
                pinned,
                self.force,
            )
            .map_err(eyre::Report::from)?
        };

        let (major, minor) = plugin.api_version();
        let info = InstalledPlugin {
            name: plugin.name().to_string(),
            source: plugin.source().display().to_string(),
            supported_methods: plugin.supported_methods().join(", "),
            api_version: format!("{major}.{minor}"),
        };

        match op.target {
            OutputFor::Human => {
                op.output_human(&info)?;
            }
            _ => {
                op.output_machine(serde_json::to_value(&info)?)?;
            }
        }
        Ok(())
    }
}
