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
use ratatui::{layout::Rect, prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::{Component, FuzzySelectList},
    config::Config,
    error::TuiError,
    utils::centered_rect_fixed,
};

#[derive(Default)]
pub struct ApiRequestSelect {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    fuzzy_list: FuzzySelectList,
}

impl ApiRequestSelect {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for ApiRequestSelect {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.fuzzy_list.set_items(config.mode_aliases.keys());
        self.config = config;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.fuzzy_list.handle_key_events(key)?;
        if key.code == KeyCode::Enter
            && let Some(selected) = self.fuzzy_list.selected()
            && let Some(item) = self.config.mode_aliases.get(selected.as_str())
        {
            self.fuzzy_list.reset_filter()?;
            return Ok(Some(Action::Mode {
                mode: *item,
                stack: false,
            }));
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let area = centered_rect_fixed(50, 35, frame.area());
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
