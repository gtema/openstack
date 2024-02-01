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

use crate::*;
use anyhow::Context;

use serde::de::DeserializeOwned;

/// Output Processor
pub(crate) struct OutputProcessor {
    /// Output configuration
    config: OutputConfig,
    /// Whether output is for human or for machine
    pub(crate) target: OutputFor,
}

/// Output target (human or machine) enum
pub(crate) enum OutputFor {
    Human,
    Machine,
}

impl OutputProcessor {
    /// Get OutputConfig from passed arguments
    pub(crate) fn from_args(args: &Cli) -> Self {
        let target = match args.global_opts.output {
            None => OutputFor::Human,
            Some(OutputFormat::Wide) => OutputFor::Human,
            _ => OutputFor::Machine,
        };
        Self {
            config: OutputConfig {
                fields: BTreeSet::from_iter(args.global_opts.fields.iter().cloned()),
                wide: matches!(args.global_opts.output, Some(OutputFormat::Wide)),
            },
            target,
        }
    }

    /// Validate command arguments with respect to the output producing
    pub(crate) fn validate_args(&self, _args: &Cli) -> Result<(), OpenStackCliError> {
        Ok(())
    }

    /// Output List of resources
    pub(crate) fn output_list<T>(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<(), OpenStackCliError>
    where
        Vec<T>: StructTable,
        T: DeserializeOwned,
    {
        match self.target {
            OutputFor::Human => {
                let table: Vec<T> = serde_json::from_value(serde_json::Value::Array(data.clone()))
                    .with_context(|| "Serializing Json data list into the table failed. Try using `-o json` to still see the raw data.".to_string())?;
                self.output_human(&table)
            }
            _ => self.output_machine(serde_json::from_value(serde_json::Value::Array(data))?),
        }
    }

    /// Output List of resources
    pub(crate) fn output_single<T>(&self, data: serde_json::Value) -> Result<(), OpenStackCliError>
    where
        T: StructTable,
        T: DeserializeOwned,
    {
        match self.target {
            OutputFor::Human => {
                let table: T = serde_json::from_value(data.clone())
                    .with_context(|| "Serializing Json data list into the table failed. Try using `-o json` to still see the raw data.".to_string())?;
                self.output_human(&table)
            }
            _ => self.output_machine(serde_json::from_value(data)?),
        }
    }

    /// Produce output for humans (table)
    pub(crate) fn output_human<T: StructTable>(&self, data: &T) -> Result<(), OpenStackCliError> {
        let (headers, table_data) = data.build(&self.config);
        print_stdout(
            table_data.table().title(headers).separator(
                cli_table::format::Separator::builder()
                    .column(Some(cli_table::format::VerticalLine::default()))
                    .title(Some(cli_table::format::HorizontalLine::default()))
                    .build(),
            ),
        )
        .map_err(OpenStackCliError::from)
    }

    /// Produce output for machine
    /// Return machine readable output with the API side names
    pub(crate) fn output_machine(&self, data: serde_json::Value) -> Result<(), OpenStackCliError> {
        serde_json::to_writer(io::stdout(), &data)?;
        io::stdout().write_all(b"\n")?;
        Ok(())
    }
}
