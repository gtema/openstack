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
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    prelude::*,
    style::palette::tailwind,
    widgets::{block::*, *},
};
use std::cmp;

use crate::{
    action::Action,
    utils::{centered_rect, TableColors},
};

const TITLE: &str = " Select cloud to connect: ";

pub struct CloudSelect {
    content_size: Size,
    clouds: Vec<String>,
    state: ListState,
    scroll_state: ScrollbarState,
    colors: TableColors,
    user_input: Option<String>,
}

impl CloudSelect {
    pub fn new(colors: TableColors, data: Vec<String>) -> Self {
        let clouds_count = data.len();
        let clouds = data;
        Self {
            content_size: Size::new(0, 0),
            clouds,
            state: ListState::default(),
            scroll_state: ScrollbarState::new(clouds_count - 1),
            colors,
            user_input: None,
        }
    }

    pub fn cursor_first(&mut self) -> Result<()> {
        self.state.select_first();
        self.scroll_state.first();
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_last(&mut self) -> Result<()> {
        self.state.select_last();
        self.scroll_state.last();
        self.user_input = None;
        Ok(())
    }

    fn cursor_up(&mut self) -> Result<()> {
        self.state.select_previous();
        self.scroll_state.prev();
        self.user_input = None;
        Ok(())
    }

    fn cursor_down(&mut self) -> Result<()> {
        self.state.select_next();
        self.scroll_state.next();
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_page_down(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => cmp::min(
                i.saturating_add(self.content_size.height as usize),
                self.clouds.len(),
            ),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(self.content_size.height as usize),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.user_input = None;
        Ok(())
    }

    pub fn select_by_filter(&mut self) -> Result<()> {
        if let Some(input) = &self.user_input {
            if !input.is_empty() {
                let mut found = false;
                for (idx, item) in self.clouds.iter().enumerate() {
                    if item.starts_with(input) {
                        self.state.select(Some(idx));
                        self.scroll_state = self.scroll_state.position(idx);
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.user_input = None;
                }
            }
        }
        Ok(())
    }
}

impl Component for CloudSelect {
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Enter => {
                if let Some(cloud_pos) = self.state.selected() {
                    if let Some(cloud) = self.clouds.get(cloud_pos) {
                        return Ok(Some(Action::ConnectToCloud(cloud.clone())));
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(ref mut input) = self.user_input {
                    input.pop();
                    if input.is_empty() {
                        self.user_input = None;
                    }
                };
                self.select_by_filter()?;
            }
            KeyCode::Char(i) => {
                self.user_input.get_or_insert(String::new()).push(i);
                self.select_by_filter()?;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<()> {
        let area = centered_rect(25, 25, frame.area());
        let mut title = vec![TITLE.white()];
        if let Some(input) = &self.user_input {
            title.push(Span::styled(
                format!("(prefix: {})", input),
                tailwind::BLUE.c400,
            ));
        }
        let popup_block = Block::default()
            .title_top(Line::from(title).centered())
            .title_bottom(
                Line::from(" (↑) move up | (↓) move down | (Enter) to connect ")
                    .gray()
                    .right_aligned(), //.alignment(Alignment::Right),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .style(tailwind::GREEN.c900)
            .bg(self.colors.header_bg)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().white());
        let inner = popup_block.inner(area);
        self.content_size = inner.as_size();

        let mut rows: Vec<ListItem> = Vec::new();
        for cloud in &self.clouds {
            if let Some(input) = &self.user_input {
                if cloud.starts_with(input) {
                    rows.push(ListItem::from(Line::from(vec![
                        Span::styled(input.clone(), tailwind::RED.c950),
                        Span::raw(
                            cloud
                                .strip_prefix(input)
                                .expect("Cloud name contains user_input prefix"),
                        ),
                    ])));
                } else {
                    rows.push(ListItem::new(cloud.clone().fg(self.colors.row_fg)));
                }
            } else {
                rows.push(ListItem::new(cloud.clone().fg(self.colors.row_fg)));
            }
        }
        let list = List::default()
            .items(rows)
            .block(popup_block)
            .style(self.colors.header_fg)
            .highlight_style(Style::new().bg(self.colors.selected_style_fg));

        frame.render_widget(Clear, area);
        frame.render_stateful_widget(list, area, &mut self.state);

        if usize::from(self.content_size.height) < self.clouds.len() {
            frame.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .end_symbol(None),
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                }),
                &mut self.scroll_state,
            );
        }

        Ok(())
    }
}