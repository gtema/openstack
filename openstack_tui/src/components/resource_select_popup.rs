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
    layout::Rect,
    prelude::*,
    widgets::{block::*, *},
};
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::{Component, FuzzySelectList},
    config::Config,
    mode::Mode,
    utils::centered_rect,
};

pub struct ResourceSelect {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
    resources: HashMap<&'static str, Mode>,
    fuzzy_list: FuzzySelectList,
}

impl Default for ResourceSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceSelect {
    pub fn new() -> Self {
        let mut slf = Self {
            resources: HashMap::from([
                ("flavors", Mode::ComputeFlavors),
                ("servers", Mode::ComputeServers),
                ("projects", Mode::IdentityProjects),
                ("images", Mode::ImageImages),
                ("networks", Mode::NetworkNetworks),
                ("subnets", Mode::NetworkSubnets),
            ]),
            command_tx: None,
            config: Config::default(),
            keymap: HashMap::new(),
            last_events: Vec::new(),
            fuzzy_list: FuzzySelectList::new(),
        };
        let mut res: Vec<&str> = slf.resources.keys().clone().copied().collect();
        res.sort();
        slf.fuzzy_list.set_items(res);
        slf
    }

    pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn tick(&mut self) {
        self.last_events.drain(..);
    }

    pub fn render_tick(&mut self) {}
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
        self.fuzzy_list.handle_key_events(key)?;
        if key.code == KeyCode::Enter {
            if let Some(selected) = self.fuzzy_list.selected() {
                if let Some(item) = self.resources.get(selected.as_str()) {
                    self.fuzzy_list.reset_filter()?;
                    return Ok(Some(Action::Mode(*item)));
                }
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<()> {
        let area = centered_rect(25, 25, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(" Select resource to display ").centered())
            .title_bottom(
                Line::from(" (↑) move up | (↓) move down | (Enter) to select | (Esc) to close ")
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
        self.fuzzy_list.draw(frame, inner)?;

        Ok(())
    }
}
