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

//! Output processing module

use comfy_table::{Cell, Color, ContentArrangement, Table, presets::UTF8_FULL_CONDENSED};
use openstack_sdk::types::EntryStatus;
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::io::{self, Write};

use crate::OpenStackCliError;
use crate::cli::{Cli, OutputFormat, TableArrangement};
use crate::config::{FieldConfig, ViewConfig};
use structable::{OutputConfig, StructTable, StructTableOptions};

/// Output Processor
#[derive(Clone)]
pub(crate) struct OutputProcessor {
    /// Resource output configuration
    pub(crate) config: Option<ViewConfig>,
    /// Whether output is for human or for machine
    pub(crate) target: OutputFor,
    /// Table arrangement
    pub(crate) table_arrangement: TableArrangement,
    /// Fields requested
    pub(crate) fields: BTreeSet<String>,
    /// Wide mode
    pub(crate) wide: bool,
    /// Pretty mode
    pub(crate) pretty: bool,
}

impl StructTableOptions for OutputProcessor {
    fn wide_mode(&self) -> bool {
        self.wide
    }

    fn pretty_mode(&self) -> bool {
        self.pretty
    }

    fn should_return_field<S: AsRef<str>>(&self, field: S, is_wide_field: bool) -> bool {
        let is_requested = self
            .fields
            .iter()
            .any(|x| x.to_lowercase() == field.as_ref().to_lowercase())
            || (self.fields.is_empty()
                && self
                    .config
                    .as_ref()
                    .map(|cfg| {
                        cfg.fields.iter().any(|x| {
                            match x {
                                FieldConfig::Simple(name) => name,
                                FieldConfig::Extended { name, .. } => name,
                            }
                            .to_lowercase()
                                == field.as_ref().to_lowercase()
                        })
                    })
                    .is_some_and(|x| x));

        if !is_wide_field {
            // Return non wide field when no field filters passed or explicitly requested the field
            is_requested || (self.fields.is_empty() && self.config.is_none())
        } else {
            // The wide field is returned in wide mode when no filters passed or explicitly
            // requested the field
            (self.fields.is_empty() && self.wide_mode()) || is_requested
        }
    }
}

/// Output target (human or machine) enum
#[derive(Clone)]
pub(crate) enum OutputFor {
    Human,
    Machine,
}

impl From<TableArrangement> for ContentArrangement {
    fn from(value: TableArrangement) -> Self {
        match value {
            TableArrangement::Dynamic => Self::Dynamic,
            TableArrangement::DynamicFullWidth => Self::DynamicFullWidth,
            TableArrangement::Disabled => Self::Disabled,
        }
    }
}

impl OutputProcessor {
    /// Get OutputConfig from passed arguments
    pub fn from_args(args: &Cli) -> Self {
        let target = match args.global_opts.output {
            None => OutputFor::Human,
            Some(OutputFormat::Wide) => OutputFor::Human,
            _ => OutputFor::Machine,
        };
        Self {
            config: None,
            target,
            table_arrangement: args.global_opts.table_arrangement,
            fields: BTreeSet::from_iter(args.global_opts.fields.iter().cloned()),
            wide: matches!(args.global_opts.output, Some(OutputFormat::Wide)),
            pretty: args.global_opts.pretty,
        }
    }

    /// Get OutputConfig from passed arguments
    pub fn from_args_with_resource_key<S: AsRef<str>>(args: &Cli, resource_key: S) -> Self {
        let target = match args.global_opts.output {
            None => OutputFor::Human,
            Some(OutputFormat::Wide) => OutputFor::Human,
            _ => OutputFor::Machine,
        };

        Self {
            config: args.config.views.get(resource_key.as_ref()).cloned(),
            target,
            table_arrangement: args.global_opts.table_arrangement,
            fields: BTreeSet::from_iter(args.global_opts.fields.iter().cloned()),
            wide: matches!(args.global_opts.output, Some(OutputFormat::Wide)),
            pretty: args.global_opts.pretty,
        }
    }

    /// Validate command arguments with respect to the output producing
    pub fn validate_args(&self, _args: &Cli) -> Result<(), OpenStackCliError> {
        Ok(())
    }

    /// Output List of resources
    pub fn output_list<T>(&self, data: Vec<serde_json::Value>) -> Result<(), OpenStackCliError>
    where
        T: StructTable,
        T: DeserializeOwned,
        for<'a> &'a T: StructTable,
    {
        match self.target {
            OutputFor::Human => {
                let table: Vec<T> = serde_json::from_value(serde_json::Value::Array(data.clone()))
                    .map_err(|err| {
                        OpenStackCliError::deserialize(
                            err,
                            serde_json::to_string(&serde_json::Value::Array(
                                data.into_iter()
                                    .filter(|item| {
                                        serde_json::from_value::<T>(item.clone()).is_err()
                                    })
                                    .collect(),
                            ))
                            .unwrap_or_else(|v| format!("{:?}", v)),
                        )
                    })?;

                let (headers, table_rows) = structable::build_list_table(table.iter(), self);
                let mut statuses: Vec<Option<String>> =
                    table.iter().map(|item| item.status()).collect();

                // Ensure we have as many statuses as rows to zip them properly
                statuses.resize_with(table_rows.len(), Default::default);

                let rows = table_rows
                    .iter()
                    .zip(statuses.iter())
                    .map(|(data, status)| {
                        let color = match EntryStatus::from(status.as_ref()) {
                            EntryStatus::Error => Some(Color::Red),
                            EntryStatus::Pending => Some(Color::Yellow),
                            EntryStatus::Inactive => Some(Color::Cyan),
                            _ => None,
                        };
                        data.iter().map(move |cell| {
                            if let Some(color) = color {
                                Cell::new(cell).fg(color)
                            } else {
                                Cell::new(cell)
                            }
                        })
                    });
                let mut table = Table::new();
                table
                    .load_preset(UTF8_FULL_CONDENSED)
                    .set_content_arrangement(ContentArrangement::from(self.table_arrangement))
                    .set_header(headers)
                    .add_rows(rows);
                println!("{table}");
                Ok(())
            }
            _ => self.output_machine(serde_json::from_value(serde_json::Value::Array(data))?),
        }
    }

    /// Output List of resources
    pub fn output_single<T>(&self, data: serde_json::Value) -> Result<(), OpenStackCliError>
    where
        T: StructTable,
        T: DeserializeOwned,
    {
        match self.target {
            OutputFor::Human => {
                let table: T = serde_json::from_value(data.clone()).map_err(|err| {
                    OpenStackCliError::deserialize(
                        err,
                        serde_json::to_string(&data.clone()).unwrap_or_default(),
                    )
                })?;

                self.output_human(&table)
            }
            _ => self.output_machine(serde_json::from_value(data)?),
        }
    }

    /// Produce output for humans (table) for a single resource
    pub fn output_human<T: StructTable>(&self, data: &T) -> Result<(), OpenStackCliError> {
        let (headers, table_rows) = structable::build_table(data, &OutputConfig::default());

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL_CONDENSED)
            .set_content_arrangement(ContentArrangement::from(self.table_arrangement))
            .set_header(headers)
            .add_rows(table_rows);
        println!("{table}");
        Ok(())
    }

    /// Produce output for machine
    /// Return machine readable output with the API side names
    pub fn output_machine(&self, data: serde_json::Value) -> Result<(), OpenStackCliError> {
        if self.pretty {
            serde_json::to_writer_pretty(io::stdout(), &data)?;
        } else {
            serde_json::to_writer(io::stdout(), &data)?;
        }
        io::stdout().write_all(b"\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_returned_no_selection() {
        let out = OutputProcessor {
            config: None,
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::new(),
            wide: false,
            pretty: false,
        };

        assert!(
            out.should_return_field("dummy", false),
            "default field returned in non-wide mode with empty fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with empty fields selector"
        );

        let out = OutputProcessor {
            config: None,
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::new(),
            wide: true,
            pretty: false,
        };

        assert!(
            out.should_return_field("dummy", false),
            "default field returned in wide mode with empty fields selector"
        );
        assert!(
            out.should_return_field("dummy", true),
            "wide field returned in wide mode with empty fields selector"
        );
    }

    #[test]
    fn test_field_returned_selection_no_config() {
        let out = OutputProcessor {
            config: None,
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::from(["foo".to_string()]),
            wide: false,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            out.should_return_field("foo", false),
            "default field returned in non-wide mode with matching fields selector"
        );
        assert!(
            out.should_return_field("foo", true),
            "wide field returned in non-wide mode with matching fields selector"
        );

        let out = OutputProcessor {
            config: None,
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::from(["foo".to_string()]),
            wide: true,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in wide mode with mismatching fields selector"
        );
    }

    #[test]
    fn test_field_returned_selection_with_config_with_filters() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                fields: Vec::from([FieldConfig::Simple("foo".to_string())]),
            }),
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::from(["bar".to_string()]),
            wide: false,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("foo", false),
            "default field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("foo", true),
            "wide field not returned in non-wide mode with mismatching fields selector"
        );
        assert!(
            out.should_return_field("bar", false),
            "default field returned in non-wide mode with matching fields selector"
        );
        assert!(
            out.should_return_field("bar", true),
            "wide field returned in non-wide mode with matching fields selector"
        );

        let out = OutputProcessor {
            config: Some(ViewConfig {
                fields: Vec::from([FieldConfig::Simple("foo".to_string())]),
            }),
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::from(["bar".to_string()]),
            wide: true,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("foo", false),
            "config field not returned in wide mode with mismatching fields selector"
        );
        assert!(
            !out.should_return_field("foo", true),
            "wide config field not returned in wide mode with mismatching fields selector"
        );
        assert!(
            out.should_return_field("bar", false),
            "default field returned in wide mode with matching fields selector"
        );
        assert!(
            out.should_return_field("bar", true),
            "wide field returned in wide mode with matching fields selector"
        );
    }

    #[test]
    fn test_field_returned_selection_with_config_no_filters() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                fields: Vec::from([FieldConfig::Simple("foo".to_string())]),
            }),
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::new(),
            wide: false,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in non-wide mode with empty fields selector and not in config"
        );
        assert!(
            out.should_return_field("foo", false),
            "default field not returned in non-wide mode with empty fields selector, but in config"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with empty fields selector and not in config"
        );
        assert!(
            out.should_return_field("foo", true),
            "wide field returned in non-wide mode with empty fields selector, but in config"
        );

        let out = OutputProcessor {
            config: Some(ViewConfig {
                fields: Vec::from([FieldConfig::Simple("foo".to_string())]),
            }),
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::new(),
            wide: true,
            pretty: false,
        };

        assert!(
            !out.should_return_field("dummy", false),
            "default field not returned in wide mode with empty fields selector and not in config"
        );
        assert!(
            out.should_return_field("foo", false),
            "default field returned in wide mode with empty fields selector, but in config"
        );
        assert!(
            out.should_return_field("dummy", true),
            "wide field returned in wide mode with empty fields selector and not in config"
        );
        assert!(
            out.should_return_field("foo", true),
            "wide field returned in wide mode with empty fields selector, but in config"
        );
    }
}
