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

//! TUI header component

use eyre::Result;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use std::cmp::max;
//use ratatui::{layout::Flex, prelude::*, widgets::*};

use crate::{
    action::Action,
    components::Component,
    config::{key_event_to_string_with_unicode, CommandType, Config},
    error::TuiError,
    mode::Mode,
};

#[derive(Debug, Clone)]
pub struct Header {
    config: Config,
    current_mode: Mode,

    cloud_name: String,
    project_name: String,
    domain_name: String,
    /// Connection data
    connection_data_rows: Vec<(String, String)>,
    /// Global bindings rows
    global_bindings_rows: Vec<(String, String)>,
    /// Keybindings of the current mode (filters)
    mode_filter_keybindings: Vec<(String, String)>,
    /// Keybindings of the current mode (actions)
    mode_action_keybindings: Vec<(String, String)>,
    size: Size,
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            current_mode: Mode::default(),
            size: Size::new(0, 0),

            cloud_name: String::new(),
            project_name: String::new(),
            domain_name: String::new(),
            mode_action_keybindings: Vec::new(),
            mode_filter_keybindings: Vec::new(),
            connection_data_rows: Vec::new(),
            global_bindings_rows: vec![
                (String::from("<:>"), String::from("Select resource")),
                (String::from("<F2>"), String::from("Switch cloud")),
                (String::from("<F4>"), String::from("Switch project")),
            ],
        }
    }

    fn app_tick(&mut self) -> Result<()> {
        Ok(())
    }

    fn render_tick(&mut self) -> Result<()> {
        Ok(())
    }

    /// Render keybindings with given style and count of rows as 2 columns
    ///
    /// Returns remainder area
    fn draw_keybindings<'a, I>(
        &self,
        keybindings: I,
        style: Style,
        count_rows: usize,
        f: &mut Frame<'_>,
        rect: Rect,
    ) -> Result<Rect, TuiError>
    where
        I: Iterator<Item = &'a (String, String)>,
    {
        let mut remainder: Rect = rect;
        for col_bindings in &keybindings.chunks(count_rows) {
            let mut bindings_width: usize = 0;
            let mut descriptions_width: usize = 0;
            // Collect data into rows simultaneously counting max widths
            let (bindings, descriptions): (Vec<ListItem>, Vec<ListItem>) = col_bindings
                .map(|x| (x.0.clone().into(), x.1.clone().into()))
                .map(|(k, d): (ListItem, ListItem)| {
                    bindings_width = max(bindings_width, k.width());
                    descriptions_width = max(descriptions_width, d.width());
                    (k, d)
                })
                .collect();

            let bindings_list = List::default().items(bindings.clone()).style(style);
            let descriptions_list = List::default()
                .items(descriptions.clone())
                .style(Style::new());

            // Split current area into 3 (binding, description, remainder)
            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(bindings_width as u16),
                    Constraint::Length(descriptions_width as u16),
                    Constraint::Percentage(100),
                ])
                .spacing(2)
                .split(remainder);
            f.render_widget(bindings_list, cols[0]);
            f.render_widget(descriptions_list, cols[1]);
            // Update the remainder pointer
            remainder = cols[2];
        }
        Ok(remainder)
    }
}

impl Component for Header {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        self.current_mode = current_mode;
        match action {
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ConnectToCloud(ref cloud) => {
                // Save information about reconnecting to the cloud
                self.cloud_name = cloud.clone();
                self.project_name.clear();
                self.domain_name.clear();
                self.connection_data_rows.clear();
                self.connection_data_rows
                    .extend([(String::from("Cloud:"), self.cloud_name.clone())]);
            }
            Action::Mode { .. } | Action::PrevMode => {
                self.current_mode = current_mode;
                self.mode_filter_keybindings.clear();
                self.mode_action_keybindings.clear();
                if let Some(keymap) = self.config.mode_keybindings.get(&self.current_mode) {
                    // Update mode keybindings rows with the current mode bindings
                    keymap
                        .iter()
                        .map(|(k, v)| {
                            (
                                format!(
                                    "<{}>",
                                    k.iter()
                                        .map(key_event_to_string_with_unicode)
                                        .collect::<Vec<_>>()
                                        .join("")
                                ),
                                v.description.clone().unwrap_or(String::from("")),
                                v.r#type.clone(),
                            )
                        })
                        .sorted()
                        .for_each(|(k, v, t)| match t {
                            CommandType::ApiRequestAction => {
                                self.mode_action_keybindings.push((k, v));
                            }
                            CommandType::Filter => {
                                self.mode_filter_keybindings.push((k, v));
                            }
                        });
                }
            }
            Action::ConnectedToCloud(ref auth_token) => {
                // Update information about cloud we are connected to
                self.connection_data_rows.clear();
                self.connection_data_rows
                    .push((String::from("Cloud:"), self.cloud_name.clone()));
                if let Some(project) = &auth_token.project {
                    if let Some(domain) = &project.domain {
                        if let Some(name) = &domain.name {
                            self.domain_name = name.clone();
                            self.connection_data_rows
                                .push((String::from("Domain:"), self.domain_name.clone()));
                        }
                    }
                    if let Some(name) = &project.name {
                        self.project_name = name.clone();
                        self.connection_data_rows
                            .push((String::from("Project:"), self.project_name.clone()));
                    }
                } else if let Some(domain) = &auth_token.domain {
                    if let Some(name) = &domain.name {
                        self.domain_name = name.clone();
                        self.connection_data_rows
                            .push((String::from("Domain:"), self.domain_name.clone()));
                    }
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), TuiError> {
        self.size = rect.as_size();
        // Count number of rows (-1 to keep some spacing)
        if let Some(count_rows) = (self.size.height as usize).checked_sub(1) {
            // Split whole area first into 3 columns
            let rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Min(40),         // Connection stats
                    Constraint::Length(25),      // Global keybindings
                    Constraint::Percentage(100), // Mode keybindings
                ])
                .spacing(2)
                .split(rect);

            // Connection info column
            let (c1, c2): (Vec<ListItem>, Vec<ListItem>) = self
                .connection_data_rows
                .iter()
                .map(|x| (x.0.clone().into(), x.1.clone().into()))
                .collect();

            let c1_list = List::default()
                .items(c1.clone())
                .style(Style::new().fg(Color::Yellow));
            let c2_list = List::default().items(c2.clone()).style(Style::new());

            let cols = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(8), Constraint::Percentage(100)])
                .spacing(2)
                .split(rects[0]);
            f.render_widget(c1_list, cols[0]);
            f.render_widget(c2_list, cols[1]);

            // Global keybindings column
            self.draw_keybindings(
                self.global_bindings_rows.iter(),
                Style::new().fg(Color::Magenta),
                count_rows,
                f,
                rects[1],
            )?;

            let mut remainder = rects[2];
            // Mode filter keybindings
            if !self.mode_filter_keybindings.is_empty() {
                remainder = self.draw_keybindings(
                    self.mode_filter_keybindings.iter(),
                    Style::new().fg(Color::LightBlue),
                    count_rows,
                    f,
                    remainder,
                )?;
            }

            // Mode action keybindings
            if !self.mode_action_keybindings.is_empty() {
                self.draw_keybindings(
                    self.mode_action_keybindings.iter(),
                    Style::new().fg(Color::Red),
                    count_rows,
                    f,
                    remainder,
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
