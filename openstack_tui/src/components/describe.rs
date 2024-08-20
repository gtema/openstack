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

use super::{Component, Frame};
use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{action::Action, config::Config, mode::Mode, utils::PALETTES};

#[derive(Default)]
pub struct Describe {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    pub keymap: HashMap<KeyEvent, Action>,
    pub text: Vec<String>,
    pub last_events: Vec<KeyEvent>,
    line_scroll: u16,
}

impl Describe {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn tick(&mut self) {
        self.last_events.drain(..);
    }

    pub fn render_tick(&mut self) {}

    fn set_data(&mut self, data: Value) -> Result<()> {
        let data: serde_yaml::Value = serde_json::from_value(data)?;
        self.text = serde_yaml::to_string(&data)?
            .split("\n")
            .map(String::from)
            .collect::<Vec<_>>();
        Ok(())
    }

    pub fn up(&mut self) {
        if self.line_scroll > 0 {
            self.line_scroll -= 1
        };
    }

    pub fn down(&mut self) {
        self.line_scroll += 1;
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title(Title::from(" Describe ").alignment(Alignment::Center))
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(PALETTES[0].c900));

        let text: Vec<Line> = self.text.clone().into_iter().map(Line::from).collect();
        let paragraph = Paragraph::new(text)
            .block(block)
            .scroll((self.line_scroll, 0));
        f.render_widget(paragraph, area);
    }
}

impl Component for Describe {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.down(),
                KeyCode::Char('k') | KeyCode::Up => self.up(),
                _ => {}
            }
        }
        Ok(None)
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>> {
        if let Action::Describe(data) = action {
            self.set_data(data)?;
        };
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        self.render(f, area);
        Ok(())
    }
}
