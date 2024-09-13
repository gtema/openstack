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
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use eyre::{OptionExt, Result};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use serde_json::Value;
use std::cmp;
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
    wrap: bool,
    max_row_length: u16,
    content_scroll: (u16, u16),
    content_size: Size,
    vscroll_state: ScrollbarState,
    hscroll_state: ScrollbarState,
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
        if data.is_string() {
            self.text = data
                .as_str()
                .ok_or_eyre("Cannot treat data as string")?
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        } else {
            let data: serde_yaml::Value = serde_json::from_value(data)?;
            self.text = serde_yaml::to_string(&data)?
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        }
        self.max_row_length = self.text.iter().map(String::len).max().unwrap_or(0) as u16;
        self.vscroll_state = ScrollbarState::default().content_length(
            self.text
                .len()
                .saturating_sub(self.content_size.height as usize),
        );
        self.hscroll_state = ScrollbarState::default().content_length(
            self.max_row_length
                .saturating_sub(self.content_size.width)
                .into(),
        );

        Ok(())
    }

    pub fn cursor_up(&mut self) -> Result<()> {
        if self.text.len() as u16 > self.content_size.height {
            self.content_scroll.0 = self.content_scroll.0.saturating_sub(1);
            self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        }
        Ok(())
    }

    pub fn cursor_down(&mut self) -> Result<()> {
        if self.text.len() as u16 > self.content_size.height {
            self.content_scroll.0 = cmp::min(
                self.content_scroll.0.saturating_add(1),
                (self.text.len() as u16).saturating_sub(self.content_size.height),
            );
            self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        }
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<()> {
        if self.text.len() as u16 > self.content_size.height {
            self.content_scroll.0 = cmp::min(
                self.content_scroll
                    .0
                    .saturating_sub(self.content_size.height),
                (self.text.len() as u16).saturating_sub(self.content_size.height),
            );
            self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        }
        Ok(())
    }

    pub fn cursor_page_down(&mut self) -> Result<()> {
        if self.text.len() as u16 > self.content_size.height {
            self.content_scroll.0 = cmp::min(
                self.content_scroll
                    .0
                    .saturating_add(self.content_size.height),
                (self.text.len() as u16).saturating_sub(self.content_size.height),
            );
            self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        }
        Ok(())
    }

    pub fn cursor_first(&mut self) -> Result<()> {
        self.content_scroll.0 = 0;
        self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        Ok(())
    }

    pub fn cursor_last(&mut self) -> Result<()> {
        self.content_scroll.0 = (self.text.len() as u16).saturating_sub(self.content_size.height);
        self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
        Ok(())
    }

    pub fn cursor_right(&mut self) -> Result<()> {
        if self.max_row_length > self.content_size.width {
            self.content_scroll.1 = cmp::min(
                self.content_scroll.1.saturating_add(1),
                self.max_row_length.saturating_sub(self.content_size.width),
            );
            self.hscroll_state = self.hscroll_state.position(self.content_scroll.1.into());
        }
        Ok(())
    }

    pub fn cursor_left(&mut self) -> Result<()> {
        if self.max_row_length > self.content_size.height {
            self.content_scroll.1 = self.content_scroll.1.saturating_sub(1);
            self.hscroll_state = self.hscroll_state.position(self.content_scroll.1.into());
        }
        Ok(())
    }

    fn render(&mut self, f: &mut Frame<'_>, area: Rect) {
        let block = Block::default()
            .title(Title::from(" Describe ").alignment(Alignment::Center))
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(PALETTES[0].c900));
        self.content_size = block.inner(area).as_size();

        let text: Vec<Line> = self.text.clone().into_iter().map(Line::from).collect();
        let paragraph = Paragraph::new(text)
            .block(block)
            .scroll((self.content_scroll.0, self.content_scroll.1));
        f.render_widget(paragraph, area);

        if usize::from(self.content_size.height) < self.text.len() {
            self.render_vscrollbar(f, area);
        }
        if self.content_size.width < self.max_row_length {
            self.render_hscrollbar(f, area);
        }
    }

    pub fn render_vscrollbar(&mut self, f: &mut Frame, area: Rect) {
        self.vscroll_state = self
            .vscroll_state
            .content_length(
                (self.text.len() as u16)
                    .saturating_sub(self.content_size.height)
                    .into(),
            )
            .viewport_content_length(self.content_size.height.into());
        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.vscroll_state,
        );
    }

    pub fn render_hscrollbar(&mut self, f: &mut Frame, area: Rect) {
        self.hscroll_state = self
            .hscroll_state
            .content_length(
                self.max_row_length
                    .saturating_sub(self.content_size.width)
                    .into(),
            )
            .viewport_content_length(self.content_size.height.into());
        f.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.hscroll_state,
        );
    }
}

impl Component for Describe {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        if key.kind == KeyEventKind::Press {
            match key.code {
                KeyCode::Char('j') | KeyCode::Down => self.cursor_down()?,
                KeyCode::Char('k') | KeyCode::Up => self.cursor_up()?,
                KeyCode::Home => self.cursor_first()?,
                KeyCode::End => self.cursor_last()?,
                KeyCode::PageUp => self.cursor_page_up()?,
                KeyCode::PageDown => self.cursor_page_down()?,
                KeyCode::Left => self.cursor_left()?,
                KeyCode::Right => self.cursor_right()?,
                // KeyCode::Char('w') => {
                //     self.wrap = !self.wrap;
                // }
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
