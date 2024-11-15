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

use eyre::Result;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};

use crate::{
    action::Action,
    components::Component,
    config::{key_event_to_string, Config},
    mode::Mode,
};

#[derive(Debug, Clone)]
pub struct Header {
    config: Config,
    current_mode: Mode,

    cloud_name: String,
    project_name: String,
    domain_name: String,
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

/// Ensure string is smaller than the length otherwise truncate it adding ".." as a suffix to
/// indicate truncation has happened
fn ensure_max_length(value: &str, max_length: usize) -> String {
    if value.len() <= max_length {
        value.to_owned()
    } else if max_length == 0 {
        String::from("")
    } else if max_length == 1 {
        return String::from(".");
    } else if max_length == 2 {
        return format!("{}.", value.get(0..1).unwrap_or("."));
    } else {
        let mut result = value.to_owned();
        result.replace_range(max_length - 2.., "..");
        return result;
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            current_mode: Mode::default(),

            cloud_name: String::new(),
            project_name: String::new(),
            domain_name: String::new(),
        }
    }

    fn app_tick(&mut self) -> Result<()> {
        Ok(())
    }

    fn render_tick(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Component for Header {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>> {
        self.current_mode = current_mode;
        match action {
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ConnectToCloud(ref cloud) => {
                self.cloud_name = cloud.clone();
                self.project_name.clear();
                self.domain_name.clear();
            }
            Action::Mode(mode) => self.current_mode = mode,
            Action::ConnectedToCloud(ref auth_token) => {
                if let Some(project) = &auth_token.project {
                    if let Some(name) = &project.name {
                        self.project_name = name.clone()
                    }
                    if let Some(domain) = &project.domain {
                        if let Some(name) = &domain.name {
                            self.domain_name = name.clone();
                        }
                    }
                };
            }
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<()> {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(40), // first column
                Constraint::Min(40), // second column
                Constraint::Min(40), // second column
                Constraint::Percentage(100),
            ])
            .split(rect);

        let connect_info_rows = [
            Row::new(vec![
                Span::styled("Cloud:", Style::new().yellow()),
                Span::from(ensure_max_length(&self.cloud_name, 30)),
            ]),
            Row::new(vec![
                Span::styled("Domain:", Style::new().yellow()),
                Span::from(ensure_max_length(&self.domain_name, 30)),
            ]),
            Row::new(vec![
                Span::styled("Project:", Style::new().yellow()),
                Span::from(ensure_max_length(&self.project_name, 30)),
            ]),
        ];
        // Columns widths are constrained in the same way as Layout...
        let widths = [Constraint::Length(8), Constraint::Length(30)];
        let widths2 = [Constraint::Min(4), Constraint::Fill(35)];

        let connect_info_table = Table::new(connect_info_rows, widths)
            .column_spacing(1)
            .style(Style::new().white());

        f.render_widget(connect_info_table.clone(), rects[0]);

        let global_shortcuts_rows = [
            Row::new(vec![
                Span::styled("<:>", Style::new().yellow()),
                Span::from("Select resource"),
            ]),
            Row::new(vec![
                Span::styled("<F2>", Style::new().yellow()),
                Span::from("Switch cloud"),
            ]),
            Row::new(vec![
                Span::styled("<F4>", Style::new().yellow()),
                Span::from("Select project scope"),
            ]),
        ];
        let global_shortcuts_table = Table::new(global_shortcuts_rows, widths2)
            .column_spacing(1)
            .style(Style::new().white());

        f.render_widget(global_shortcuts_table.clone(), rects[1]);

        if let Some(keymap) = self.config.mode_keybindings.get(&self.current_mode) {
            let mut shortcuts_rows: Vec<Row> = Vec::new();
            for (k, v) in keymap
                .iter()
                // Map data to be able to sort by string representation
                .map(|(k, v)| {
                    (
                        k.iter()
                            .map(key_event_to_string)
                            .collect::<Vec<_>>()
                            .join(""),
                        v.description.clone().unwrap_or(String::from("")),
                    )
                })
                .sorted()
            {
                shortcuts_rows.push(Row::new(vec![
                    Span::styled(format!("<{}>", k), Style::new().yellow()),
                    v.into(),
                ]));
            }
            let shortcuts_table = Table::new(shortcuts_rows, widths2)
                .column_spacing(1)
                .style(Style::new().white());

            f.render_widget(shortcuts_table.clone(), rects[2]);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_max_length() {
        let original = String::from("123456789");
        assert_eq!(String::from(""), ensure_max_length(&original, 0));
        assert_eq!(String::from("."), ensure_max_length(&original, 1));
        assert_eq!(String::from("1."), ensure_max_length(&original, 2));
        assert_eq!(String::from("1.."), ensure_max_length(&original, 3));
        assert_eq!(String::from("12.."), ensure_max_length(&original, 4));
        assert_eq!(String::from("123.."), ensure_max_length(&original, 5));
        assert_eq!(String::from("1234.."), ensure_max_length(&original, 6));
        assert_eq!(String::from("12345.."), ensure_max_length(&original, 7));
        assert_eq!(String::from("123456.."), ensure_max_length(&original, 8));
        assert_eq!(original, ensure_max_length(&original, 9));
        assert_eq!(original, ensure_max_length(&original, 10));
        assert_eq!(original, ensure_max_length(&original, 20));
    }
}
