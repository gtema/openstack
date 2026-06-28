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

use super::Component;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use eyre::{OptionExt, Result};
use ratatui::{
    prelude::*,
    style::palette::tailwind,
    widgets::{
        Block, BorderType, Borders, Clear, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
};
use serde_json::Value;
use std::cmp;

use crate::{action::Action, config::Config, error::TuiError, mode::Mode};

#[derive(Default)]
pub struct Describe {
    config: Config,
    text: Vec<String>,
    title: Option<String>,
    is_focused: bool,
    is_loading: bool,
    max_row_length: u16,
    content_scroll: (u16, u16),
    content_size: Size,
    vscroll_state: ScrollbarState,
    hscroll_state: ScrollbarState,
}

impl Describe {
    pub fn new() -> Self {
        Self {
            is_focused: true,
            ..Default::default()
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    pub fn set_data(&mut self, data: Value) -> Result<()> {
        if data.is_string() {
            self.text = data
                .as_str()
                .ok_or_eyre("Cannot access data as string")?
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        } else if data.is_null() {
            self.text.clear();
        } else {
            self.text.clear();
            let data: serde_yaml::Value = serde_json::from_value(data)?;
            self.text = serde_yaml::to_string(&data)?
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();
        }
        self.max_row_length = self.text.iter().map(String::len).max().unwrap_or(0) as u16;
        self.content_scroll = (0, 0);
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

    pub fn set_focus(&mut self, focus: bool) -> Result<()> {
        self.is_focused = focus;
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
        if self.max_row_length > self.content_size.width {
            self.content_scroll.1 = self.content_scroll.1.saturating_sub(1);
            self.hscroll_state = self.hscroll_state.position(self.content_scroll.1.into());
        }
        Ok(())
    }

    fn render_inner(&mut self, area: Rect, buf: &mut Buffer) {
        let (focus_title_style, border_type, border_col, text_col) = match self.is_focused {
            true => (
                Style::new().white(),
                BorderType::Double,
                self.config.styles.border_fg,
                self.config.styles.fg,
            ),
            false => (
                Style::default(),
                BorderType::Plain,
                tailwind::SLATE.c600,
                Color::Rgb(128, 128, 128),
            ),
        };

        let mut title = vec![
            self.title
                .clone()
                .unwrap_or(String::from(" Describe "))
                .white(),
        ];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        }
        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .title_style(focus_title_style)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_type(border_type)
            .border_style(Style::default().fg(border_col))
            .style(Style::new().fg(text_col).bg(self.config.styles.buffer_bg));
        self.content_size = block.inner(area).as_size();
        let inner = block.inner(area);

        block.render(area, buf);
        let text: Vec<Line> = self.text.iter().map(|s| Line::from(s.as_str())).collect();
        let paragraph = Paragraph::new(text).scroll((self.content_scroll.0, self.content_scroll.1));
        paragraph.render(inner, buf);

        if usize::from(self.content_size.height) < self.text.len() {
            self.vscroll_state = self
                .vscroll_state
                .content_length(
                    (self.text.len() as u16)
                        .saturating_sub(self.content_size.height)
                        .into(),
                )
                .viewport_content_length(self.content_size.height.into());
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(self.config.styles.border_fg));
            <Scrollbar as StatefulWidget>::render(scrollbar, inner, buf, &mut self.vscroll_state);
        }
        if self.content_size.width < self.max_row_length {
            self.hscroll_state = self
                .hscroll_state
                .content_length(
                    self.max_row_length
                        .saturating_sub(self.content_size.width)
                        .into(),
                )
                .viewport_content_length(self.content_size.width.into());
            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::HorizontalBottom)
                .style(Style::default().fg(self.config.styles.border_fg));
            <Scrollbar as StatefulWidget>::render(scrollbar, inner, buf, &mut self.hscroll_state);
        }
    }
}

impl Component for Describe {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
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

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::SetDescribeLoading(val) => {
                self.set_data(Value::Null)?;
                self.set_loading(val);
            }
            Action::SetDescribeApiResponseData(data) => {
                self.set_loading(false);
                self.set_data(data)?;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        Widget::render(Clear, area, f.buffer_mut());
        self.render_inner(area, f.buffer_mut());
        Ok(())
    }
}
