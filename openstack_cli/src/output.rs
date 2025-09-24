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

use comfy_table::{
    Cell, Color, ColumnConstraint, ContentArrangement, Table, Width, presets::UTF8_FULL_CONDENSED,
};
use itertools::Itertools;
use openstack_sdk::types::EntryStatus;
use owo_colors::{OwoColorize, Stream::Stderr};
use rand::prelude::*;
use serde::de::DeserializeOwned;
use std::collections::BTreeSet;
use std::io::{self, Write};

use crate::OpenStackCliError;
use crate::cli::{Cli, OutputFormat, TableArrangement};
use crate::config::ViewConfig;
use structable::{OutputConfig, StructTable, StructTableOptions};

/// Output Processor
#[derive(Default, Clone)]
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
    /// Command hints
    hints: Option<Vec<String>>,
}

impl StructTableOptions for OutputProcessor {
    fn wide_mode(&self) -> bool {
        self.wide
            || self
                .config
                .as_ref()
                .is_some_and(|cfg| cfg.wide.is_some_and(|w| w))
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
                        cfg.default_fields
                            .iter()
                            .any(|x| x.to_lowercase() == field.as_ref().to_lowercase())
                    })
                    .is_some_and(|x| x));

        if !is_wide_field {
            // Return non wide field when no field filters passed or explicitly requested the field
            is_requested
                || (self.fields.is_empty()
                    && self
                        .config
                        .as_ref()
                        .is_none_or(|cfg| cfg.default_fields.is_empty()))
        } else {
            // The wide field is returned in wide mode when no filters passed or explicitly
            // requested the field
            (self.fields.is_empty() && self.wide_mode()) || is_requested
        }
    }

    fn field_data_json_pointer<S: AsRef<str>>(&self, field: S) -> Option<String> {
        if !self.wide_mode() {
            self.config.as_ref().and_then(|config| {
                config
                    .fields
                    .iter()
                    .find(|x| x.name.to_lowercase() == field.as_ref().to_lowercase())
                    .and_then(|field_config| field_config.json_pointer.clone())
            })
        } else {
            None
        }
    }
}

/// Output target (human or machine) enum
#[derive(Default, Clone)]
pub(crate) enum OutputFor {
    #[default]
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
    pub fn from_args<R: AsRef<str>, A: AsRef<str>>(
        args: &Cli,
        resource_key: Option<R>,
        action: Option<A>,
    ) -> Self {
        let target = match args.global_opts.output.output {
            None => OutputFor::Human,
            Some(OutputFormat::Wide) => OutputFor::Human,
            _ => OutputFor::Machine,
        };
        let mut hints: Vec<String> = args.config.hints.clone();

        if let (Some(resource_key), Some(action)) = (&resource_key, &action) {
            args.config
                .command_hints
                .get(resource_key.as_ref())
                .and_then(|cmd_hints| {
                    cmd_hints.get(action.as_ref()).map(|val| {
                        hints.extend(val.clone());
                    })
                });
        }

        Self {
            config: resource_key
                .as_ref()
                .and_then(|val| args.config.views.get(val.as_ref()).cloned()),
            target,
            table_arrangement: args.global_opts.output.table_arrangement,
            fields: BTreeSet::from_iter(args.global_opts.output.fields.iter().cloned()),
            wide: matches!(args.global_opts.output.output, Some(OutputFormat::Wide)),
            pretty: args.global_opts.output.pretty,
            hints: Some(hints),
        }
    }

    /// Validate command arguments with respect to the output producing
    pub fn validate_args(&self, _args: &Cli) -> Result<(), OpenStackCliError> {
        Ok(())
    }

    /// Re-sort table according to the configuration and determine column constraints
    fn prepare_table(
        &self,
        headers: Vec<String>,
        data: Vec<Vec<String>>,
    ) -> (Vec<String>, Vec<Vec<String>>, Vec<Option<ColumnConstraint>>) {
        let mut headers = headers;
        let mut rows = data;
        let mut column_constrains: Vec<Option<ColumnConstraint>> = vec![None; headers.len()];

        if let Some(cfg) = &self.config {
            // Offset from the current iteration pointer
            if headers.len() > 1 {
                let mut idx_offset: usize = 0;
                for (default_idx, field) in cfg.default_fields.iter().unique().enumerate() {
                    if let Some(curr_idx) = headers
                        .iter()
                        .position(|x| x.to_lowercase() == field.to_lowercase())
                    {
                        // Swap headers between current and should pos
                        if default_idx - idx_offset < headers.len() {
                            headers.swap(default_idx - idx_offset, curr_idx);
                            for row in rows.iter_mut() {
                                // Swap also data columns
                                row.swap(default_idx - idx_offset, curr_idx);
                            }
                        }
                    } else {
                        // This column is not found in the data. Perhars structable returned some
                        // other name. Move the column to the very end
                        if default_idx - idx_offset < headers.len() {
                            let curr_hdr = headers.remove(default_idx - idx_offset);
                            headers.push(curr_hdr);
                            for row in rows.iter_mut() {
                                let curr_cell = row.remove(default_idx - idx_offset);
                                row.push(curr_cell);
                            }
                            // Some unmatched field moved to the end. Our "current" index should respect
                            // the offset
                            idx_offset += 1;
                        }
                    }
                }
            }
            // Find field configuration
            for (idx, field) in headers.iter().enumerate() {
                if let Some(field_config) = cfg
                    .fields
                    .iter()
                    .find(|x| x.name.to_lowercase() == field.to_lowercase())
                {
                    let constraint = match (
                        field_config.width,
                        field_config.min_width,
                        field_config.max_width,
                    ) {
                        (Some(fixed), _, _) => {
                            Some(ColumnConstraint::Absolute(Width::Fixed(fixed as u16)))
                        }
                        (None, Some(lower), Some(upper)) => Some(ColumnConstraint::Boundaries {
                            lower: Width::Fixed(lower as u16),
                            upper: Width::Fixed(upper as u16),
                        }),
                        (None, Some(lower), None) => {
                            Some(ColumnConstraint::LowerBoundary(Width::Fixed(lower as u16)))
                        }
                        (None, None, Some(upper)) => {
                            Some(ColumnConstraint::UpperBoundary(Width::Fixed(upper as u16)))
                        }
                        _ => None,
                    };
                    column_constrains[idx] = constraint;
                }
            }
        }
        (headers, rows, column_constrains)
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
                            .unwrap_or_else(|v| format!("{v:?}")),
                        )
                    })?;

                let data = structable::build_list_table(table.iter(), self);
                let (headers, table_rows, table_constraints) = self.prepare_table(data.0, data.1);
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

                for (idx, constraint) in table_constraints.iter().enumerate() {
                    if let Some(constraint) = constraint {
                        if let Some(col) = table.column_mut(idx) {
                            col.set_constraint(*constraint);
                        }
                    }
                }

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

    /// Show hints
    pub fn show_command_hint(&self) -> Result<(), OpenStackCliError> {
        if rand::random_bool(1.0 / 2.0) {
            self.hints.as_ref().and_then(|hints| {
                hints.choose(&mut rand::rng()).map(|hint| {
                    eprintln!(
                        "\n{} {}",
                        "Hint:".if_supports_color(Stderr, |text| text.green()),
                        hint.if_supports_color(Stderr, |text| text.blue())
                    );
                })
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FieldConfig;
    use clap::Parser;
    use std::io::Write;
    use tempfile::Builder;

    #[test]
    fn test_wide_mode() {
        assert!(!OutputProcessor::default().wide_mode());
        assert!(
            OutputProcessor {
                wide: true,
                ..Default::default()
            }
            .wide_mode()
        );
        assert!(
            OutputProcessor {
                config: Some(ViewConfig {
                    wide: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            }
            .wide_mode()
        );
    }

    #[test]
    fn test_field_returned_no_selection() {
        let out = OutputProcessor::default();

        assert!(
            out.should_return_field("dummy", false),
            "default field returned in non-wide mode with empty fields selector"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with empty fields selector"
        );

        let out = OutputProcessor {
            wide: true,
            ..Default::default()
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
            fields: BTreeSet::from(["foo".to_string()]),
            ..Default::default()
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
            fields: BTreeSet::from(["foo".to_string()]),
            wide: true,
            ..Default::default()
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
    fn test_field_returned_selection_empty_config() {
        let out = OutputProcessor {
            config: Some(ViewConfig::default()),
            target: OutputFor::Human,
            table_arrangement: TableArrangement::Disabled,
            fields: BTreeSet::new(),
            wide: false,
            pretty: false,
            ..Default::default()
        };

        assert!(
            out.should_return_field("dummy", false),
            "default field returned in non-wide mode with mismatching fields selector and empty config"
        );
        assert!(
            !out.should_return_field("dummy", true),
            "wide field not returned in non-wide mode with mismatching fields selector and empty config"
        );
    }

    #[test]
    fn test_field_returned_selection_with_config_with_filters() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                default_fields: vec!["foo".to_string()],
                ..Default::default()
            }),
            fields: BTreeSet::from(["bar".to_string()]),
            ..Default::default()
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
                default_fields: vec!["foo".to_string()],
                ..Default::default()
            }),
            fields: BTreeSet::from(["bar".to_string()]),
            wide: true,
            ..Default::default()
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
                default_fields: vec!["foo".to_string()],
                ..Default::default()
            }),
            ..Default::default()
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
                default_fields: vec!["foo".to_string()],
                ..Default::default()
            }),
            wide: true,
            ..Default::default()
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

    #[test]
    fn test_prepare_table() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                default_fields: vec![
                    "foo".to_string(),
                    "bar".to_string(),
                    "baz".to_string(),
                    "dummy".to_string(),
                ],
                fields: vec![FieldConfig {
                    name: "bar".to_string(),
                    min_width: Some(15),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let (hdr, rows, constraints) = out.prepare_table(
            vec![
                "dummy".to_string(),
                "bar".to_string(),
                "foo".to_string(),
                "baz".to_string(),
            ],
            vec![
                vec![
                    "11".to_string(),
                    "12".to_string(),
                    "13".to_string(),
                    "14".to_string(),
                ],
                vec![
                    "21".to_string(),
                    "22".to_string(),
                    "23".to_string(),
                    "24".to_string(),
                ],
            ],
        );
        assert_eq!(
            vec![
                "foo".to_string(),
                "bar".to_string(),
                "baz".to_string(),
                "dummy".to_string()
            ],
            hdr,
            "headers in the correct sort order"
        );
        assert_eq!(
            vec![
                vec![
                    "13".to_string(),
                    "12".to_string(),
                    "14".to_string(),
                    "11".to_string(),
                ],
                vec![
                    "23".to_string(),
                    "22".to_string(),
                    "24".to_string(),
                    "21".to_string(),
                ],
            ],
            rows,
            "row columns sorted properly"
        );
        assert_eq![
            vec![
                None,
                Some(ColumnConstraint::LowerBoundary(Width::Fixed(15))),
                None,
                None
            ],
            constraints
        ];

        let (hdr, rows, _constraints) = out.prepare_table(
            vec![
                "dummy".to_string(),
                "bar2".to_string(),
                "foo".to_string(),
                "baz2".to_string(),
            ],
            vec![
                vec![
                    "11".to_string(),
                    "12".to_string(),
                    "13".to_string(),
                    "14".to_string(),
                ],
                vec![
                    "21".to_string(),
                    "22".to_string(),
                    "23".to_string(),
                    "24".to_string(),
                ],
            ],
        );
        assert_eq!(
            vec![
                "foo".to_string(),
                "dummy".to_string(),
                "bar2".to_string(),
                "baz2".to_string(),
            ],
            hdr,
            "headers with unknown fields in the correct sort order"
        );
        assert_eq!(
            vec![
                vec![
                    "13".to_string(),
                    "11".to_string(),
                    "12".to_string(),
                    "14".to_string(),
                ],
                vec![
                    "23".to_string(),
                    "21".to_string(),
                    "22".to_string(),
                    "24".to_string(),
                ],
            ],
            rows,
            "row columns sorted properly"
        );
    }

    #[test]
    fn test_prepare_table_duplicated_values() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                default_fields: vec![
                    "foo".to_string(),
                    "bar".to_string(),
                    "foo".to_string(),
                    "baz".to_string(),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        let (hdr, rows, _constraints) = out.prepare_table(
            vec!["bar".to_string(), "foo".to_string(), "baz".to_string()],
            vec![
                vec!["11".to_string(), "12".to_string(), "13".to_string()],
                vec!["21".to_string(), "22".to_string(), "23".to_string()],
            ],
        );
        assert_eq!(
            vec!["foo".to_string(), "bar".to_string(), "baz".to_string(),],
            hdr,
            "headers in the correct sort order"
        );
        assert_eq!(
            vec![
                vec!["12".to_string(), "11".to_string(), "13".to_string(),],
                vec!["22".to_string(), "21".to_string(), "23".to_string(),],
            ],
            rows,
            "row columns sorted properly"
        );
    }

    #[test]
    fn test_prepare_table_missing_default_fields() {
        let out = OutputProcessor {
            config: Some(ViewConfig {
                default_fields: vec![
                    "foo".to_string(),
                    "bar1".to_string(),
                    "foo1".to_string(),
                    "baz1".to_string(),
                ],
                ..Default::default()
            }),
            ..Default::default()
        };
        let (hdr, rows, _constraints) = out.prepare_table(
            vec!["bar".to_string(), "foo".to_string(), "baz".to_string()],
            vec![
                vec!["11".to_string(), "12".to_string(), "13".to_string()],
                vec!["21".to_string(), "22".to_string(), "23".to_string()],
            ],
        );
        assert_eq!(
            vec!["foo".to_string(), "baz".to_string(), "bar".to_string(),],
            hdr,
            "headers in the correct sort order"
        );
        assert_eq!(
            vec![
                vec!["12".to_string(), "13".to_string(), "11".to_string(),],
                vec!["22".to_string(), "23".to_string(), "21".to_string(),],
            ],
            rows,
            "row columns sorted properly"
        );
    }

    #[test]
    fn test_output_processor_from_args_hints() {
        let mut config_file = Builder::new().suffix(".yaml").tempfile().unwrap();

        const CONFIG_DATA: &str = r#"
            views:
              foo:
                default_fields: ["a", "b", "c"]
              bar:
                fields:
                  - name: "b"
                    min_width: 1
            command_hints:
              res:
                cmd:
                  - cmd_hint1
                  - cmd_hint2
                cmd2: [cmd2_hint1]
              res2:
                cmd: []
            hints:
              - hint1
              - hint2
            enable_hints: true
        "#;

        write!(config_file, "{CONFIG_DATA}").unwrap();

        let op = OutputProcessor::from_args(
            &Cli::parse_from([
                "osc",
                "--cli-config",
                &config_file.path().as_os_str().to_string_lossy(),
                "auth",
                "show",
            ]),
            Some("res"),
            Some("cmd"),
        );
        assert_eq!(
            Some(vec![
                "hint1".to_string(),
                "hint2".to_string(),
                "cmd_hint1".to_string(),
                "cmd_hint2".to_string()
            ]),
            op.hints
        );
    }
}
