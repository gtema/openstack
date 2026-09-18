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

//! Show the effective local `osc` CLI configuration.

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::info;

use openstack_cli_core::config::Config;
use openstack_cli_core::output::{OutputFor, OutputProcessor};
use openstack_cli_core::{cli::CliArgs, error::OpenStackCliError};
use structable::{StructTable, StructTableOptions};

/// Show the effective local CLI configuration.
///
/// This is the `$XDG_CONFIG_HOME/osc/config.yaml` configuration merged with
/// the built-in defaults. It controls CLI-only behavior (output views,
/// hints) and is unrelated to `clouds.yaml`/`secure.yaml` cloud connection
/// credentials.
#[derive(Debug, Parser)]
pub struct ShowCommand {}

/// A displayable view of the effective CLI configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, StructTable)]
pub struct ConfigView {
    /// Configured output views, keyed by resource.
    #[structable(serialize)]
    pub views: Value,
    /// Configured per-resource command hints.
    #[structable(serialize)]
    pub command_hints: Value,
    /// General hints shown independent of the command.
    #[structable(serialize)]
    pub hints: Value,
    /// Whether hints are shown after a successful command.
    #[structable()]
    pub enable_hints: bool,
}

impl TryFrom<&Config> for ConfigView {
    type Error = eyre::Report;
    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        Ok(Self {
            views: serde_json::to_value(&value.views)?,
            command_hints: serde_json::to_value(&value.command_hints)?,
            hints: serde_json::to_value(&value.hints)?,
            enable_hints: value.enable_hints,
        })
    }
}

impl ShowCommand {
    /// Perform command action.
    pub async fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Show effective CLI configuration");

        let op = OutputProcessor::from_args(parsed_args, Some("config"), Some("show"));
        let config = parsed_args.config();

        match op.target {
            OutputFor::Human => {
                op.output_human(&ConfigView::try_from(config)?)?;
            }
            _ => {
                op.output_machine(serde_json::to_value(config)?)?;
            }
        }
        op.show_command_hint()?;
        Ok(())
    }
}
