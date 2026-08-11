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

//! Re-check installed wasm auth plugin file(s) against the lockfile

use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;

use openstack_cli_core::output::OutputProcessor;
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// Re-check installed wasm auth plugin file(s) against the SHA-256 recorded
/// in the lockfile at install time.
///
/// Fails on the first version whose on-disk content no longer matches, or
/// whose file is missing.
#[derive(Debug, Parser)]
pub struct VerifyCommand {
    /// Plugin name to verify.
    pub name: String,

    /// Verify only this specific version. Defaults to every installed
    /// version of `name`.
    #[arg(long)]
    pub version: Option<String>,
}

/// A single version that passed verification.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct VerifiedPluginVersion {
    /// Plugin name.
    #[structable()]
    pub name: String,

    /// Verified version.
    #[structable()]
    pub version: String,

    /// The SHA-256 that matched between the lockfile and the file on disk.
    #[structable()]
    pub sha256: String,
}

impl VerifyCommand {
    /// Perform command action
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Verify wasm auth plugin {}", self.name);

        let op = OutputProcessor::from_args(parsed_args, Some("plugin"), Some("verify"));

        let verified =
            openstack_sdk_plugin_wasm::registry::verify(&self.name, self.version.as_deref())
                .map_err(eyre::Report::from)?;

        let data: Vec<serde_json::Value> = verified
            .into_iter()
            .map(|entry| {
                serde_json::to_value(VerifiedPluginVersion {
                    name: entry.name,
                    version: entry.version,
                    sha256: entry.sha256,
                })
            })
            .collect::<Result<_, _>>()?;

        op.output_list::<VerifiedPluginVersion>(data)
    }
}
