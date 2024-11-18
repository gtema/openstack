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

use crate::{
    action::Action, components::Component, config::Config, error::TuiError, mode::Mode,
    utils::centered_rect,
};

pub struct ErrorPopup {
    config: Config,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
    text: Vec<String>,
    scroll: (u16, u16),
}

impl Default for ErrorPopup {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorPopup {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            keymap: HashMap::new(),
            text: Vec::new(),
            last_events: Vec::new(),
            scroll: (0, 0),
        }
    }

    pub fn render_tick(&mut self) {}
    pub fn scroll_right(&mut self) {
        self.scroll.0 = self.scroll.0.saturating_add(1);
    }
    pub fn scroll_left(&mut self) {
        self.scroll.0 = self.scroll.0.saturating_sub(1);
    }
    pub fn scroll_down(&mut self) {
        self.scroll.1 = self.scroll.1.saturating_add(1);
    }
    pub fn scroll_up(&mut self) {
        self.scroll.1 = self.scroll.1.saturating_sub(1);
    }
}

impl Component for ErrorPopup {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        if let Action::Error(ref msg) = action {
            self.text = strip_ansi_escapes::strip_str(msg)
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        match key.code {
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Right => self.scroll_right(),
            KeyCode::Left => self.scroll_left(),
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let ar = centered_rect(30, 25, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(" Error ").red().centered())
            .title_bottom(Line::from("(Esc) to close").gray().right_aligned())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg)
            .border_style(Style::default().fg(self.config.styles.popup_border_error_fg));
        let text: Vec<Line> = self.text.clone().into_iter().map(Line::from).collect();
        let paragraph = Paragraph::new(text)
            .block(popup_block)
            .scroll((self.scroll.1, self.scroll.0));

        frame.render_widget(Clear, ar);
        frame.render_widget(paragraph, ar);

        Ok(())
    }
}
