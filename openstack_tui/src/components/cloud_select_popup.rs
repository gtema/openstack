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
    prelude::*,
    widgets::{block::*, *},
};

use crate::{
    action::Action,
    components::{Component, FuzzySelectList},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::centered_rect,
};

const TITLE: &str = " Select cloud to connect: ";

pub struct CloudSelect {
    config: Config,
    fuzzy_list: FuzzySelectList,
}

impl Default for CloudSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudSelect {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            fuzzy_list: FuzzySelectList::new(),
        }
    }
}

impl Component for CloudSelect {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config.clone();
        self.fuzzy_list.register_config_handler(config.clone())?;
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        if let Action::Clouds(ref clouds) = action {
            self.fuzzy_list.set_items(clouds.clone());
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.fuzzy_list.handle_key_events(key)?;
        if key.code == KeyCode::Enter {
            if let Some(cloud) = self.fuzzy_list.selected() {
                return Ok(Some(Action::ConnectToCloud(cloud.clone())));
            }
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let area = centered_rect(25, 25, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(TITLE.white()).centered())
            .title_bottom(
                Line::from(" (↑) move up | (↓) move down | (Enter) to connect ")
                    .gray()
                    .right_aligned(), //.alignment(Alignment::Right),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
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
