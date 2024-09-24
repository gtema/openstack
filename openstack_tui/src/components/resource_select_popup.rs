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

use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::*,
    widgets::{block::*, *},
};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action, components::Component, config::Config, mode::Mode, utils::centered_rect,
};

#[derive(Debug, Clone, Default)]
enum CursorAt {
    #[default]
    Service,
    Resource,
}

pub struct ResourceSelect {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
    resources: HashMap<&'static str, HashMap<&'static str, Mode>>,
    service_state: ListState,
    resource_state: ListState,
    cursor_at: CursorAt,
}

impl Default for ResourceSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceSelect {
    pub fn new() -> Self {
        Self {
            resources: HashMap::from([
                (
                    "Compute",
                    HashMap::from([
                        ("Flavors", Mode::ComputeFlavors),
                        ("Servers", Mode::ComputeServers),
                    ]),
                ),
                (
                    "Identity",
                    HashMap::from([("Projects", Mode::IdentityProjects)]),
                ),
                ("Image", HashMap::from([("Images", Mode::ImageImages)])),
                (
                    "Network",
                    HashMap::from([
                        ("Networks", Mode::NetworkNetworks),
                        ("Subnets", Mode::NetworkSubnets),
                    ]),
                ),
            ]),
            command_tx: None,
            config: Config::default(),
            keymap: HashMap::new(),
            last_events: Vec::new(),
            service_state: ListState::default(),
            resource_state: ListState::default(),
            cursor_at: CursorAt::default(),
        }
    }

    pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn tick(&mut self) {
        self.last_events.drain(..);
    }

    pub fn render_tick(&mut self) {}

    fn cursor_up(&mut self) {
        let state = match self.cursor_at {
            CursorAt::Service => &mut self.service_state,
            CursorAt::Resource => &mut self.resource_state,
        };
        state.select_previous();
    }

    fn cursor_down(&mut self) {
        let state = match self.cursor_at {
            CursorAt::Service => &mut self.service_state,
            CursorAt::Resource => &mut self.resource_state,
        };
        state.select_next();
    }

    fn cursor_right(&mut self) {
        if let CursorAt::Service = self.cursor_at {
            self.cursor_at = CursorAt::Resource;
            self.resource_state.select(Some(0));
        }
    }

    fn cursor_left(&mut self) {
        if let CursorAt::Resource = self.cursor_at {
            self.cursor_at = CursorAt::Service;
            self.resource_state.select(None);
        };
    }
}

impl Component for ResourceSelect {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Down => self.cursor_down(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Left => self.cursor_left(),
            KeyCode::Enter => {
                if let CursorAt::Resource = self.cursor_at {
                    if let (Some(service_pos), Some(resource_pos)) = (
                        self.service_state.selected(),
                        self.resource_state.selected(),
                    ) {
                        if let Some((_service, resources)) = self.resources.iter().nth(service_pos)
                        {
                            if let Some((_resource, mode)) = resources.iter().nth(resource_pos) {
                                return Ok(Some(Action::Mode(*mode)));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<()> {
        let area = centered_rect(25, 25, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(" Select resource to display ").centered())
            .title_bottom(
                Line::from("(↑) move up | (↓) move down | (←|→) switch")
                    .gray()
                    .right_aligned(),
            )
            .borders(Borders::ALL)
            .bg(self.config.styles.popup_bg)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.popup_border_fg));
        let inner = popup_block.inner(area);
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        let horizontal = Layout::horizontal([Constraint::Min(9), Constraint::Min(20)]);

        let [service_area, resource_area] = horizontal.areas(inner);

        let service_block = Block::default().title(" Service ").borders(Borders::ALL);

        let service_list = List::new(self.resources.keys().cloned())
            .block(service_block)
            .style(self.config.styles.popup_item_title_fg)
            .highlight_symbol(">>")
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        frame.render_stateful_widget(service_list, service_area, &mut self.service_state);

        let resource_block = Block::default().title(" Resource ").borders(Borders::ALL);
        let mut resource_list = List::default()
            .block(resource_block)
            .style(self.config.styles.popup_item_title_fg)
            .highlight_symbol(">>")
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED));
        if let Some(service_pos) = self.service_state.selected() {
            if let Some((_service, resources)) = self.resources.iter().nth(service_pos) {
                resource_list = resource_list.items(resources.keys().cloned());
            }
        }
        frame.render_stateful_widget(resource_list, resource_area, &mut self.resource_state);

        Ok(())
    }
}
