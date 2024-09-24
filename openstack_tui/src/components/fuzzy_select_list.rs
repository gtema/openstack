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
use std::cmp;

use crate::{action::Action, components::Component, config::Config};

/// List with fuzzy searching and scroll
pub struct FuzzySelectList {
    config: Config,
    all_items: Vec<String>,
    filtered_items: Vec<String>,
    input: Option<String>,
    state: ListState,
    scroll_state: ScrollbarState,
    area_size: Size,
}

impl Default for FuzzySelectList {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzySelectList {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            all_items: Vec::new(),
            filtered_items: Vec::new(),
            input: None,
            state: ListState::default(),
            scroll_state: ScrollbarState::new(0),
            area_size: Size::new(0, 0),
        }
    }

    pub fn set_items<S: AsRef<str>>(&mut self, items: Vec<S>) -> &mut Self {
        self.all_items = items.iter().map(|x| x.as_ref().to_string()).collect();
        self.filtered_items = self.all_items.clone();
        self.input = None;
        self
    }

    pub fn selected(&self) -> Option<&String> {
        self.state
            .selected()
            .and_then(|pos| self.filtered_items.get(pos))
    }

    pub fn reset_filter(&mut self) -> Result<()> {
        self.input = None;
        self.filter()
    }

    pub fn cursor_first(&mut self) -> Result<()> {
        self.state.select_first();
        self.scroll_state.first();
        Ok(())
    }

    pub fn cursor_last(&mut self) -> Result<()> {
        self.state.select_last();
        self.scroll_state.last();
        Ok(())
    }

    fn cursor_up(&mut self) -> Result<()> {
        self.state.select_previous();
        self.scroll_state.prev();
        Ok(())
    }

    fn cursor_down(&mut self) -> Result<()> {
        self.state.select_next();
        self.scroll_state.next();
        Ok(())
    }

    pub fn cursor_page_down(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => cmp::min(
                i.saturating_add(self.area_size.height as usize),
                self.filtered_items.len(),
            ),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(self.area_size.height as usize),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        Ok(())
    }

    fn filter(&mut self) -> Result<()> {
        self.filtered_items = match &self.input {
            Some(filter) => self
                .all_items
                .clone()
                .into_iter()
                .filter(|x| x.contains(filter))
                .collect(),
            None => self.all_items.clone(),
        };
        if self.state.selected().is_none() && !self.filtered_items.is_empty() {
            self.state.select(Some(0));
        }
        Ok(())
    }
}

impl Component for FuzzySelectList {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match &key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Backspace => {
                if let Some(ref mut input) = self.input {
                    input.pop();
                    if input.is_empty() {
                        self.input = None;
                    }
                };
                self.filter()?;
            }
            KeyCode::Char(i) => {
                self.input.get_or_insert(String::new()).push(*i);
                self.filter()?;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<()> {
        self.area_size = area.as_size();

        let layout =
            Layout::vertical([Constraint::Min(3), Constraint::Percentage(100)]).split(area);
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.fg));

        let input = Paragraph::new(self.input.clone().unwrap_or_default()).block(input_block);

        frame.render_widget(input, layout[0]);

        let data_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.buffer_bg));

        let mut rows: Vec<ListItem> = Vec::new();
        for item in &self.filtered_items {
            rows.push(ListItem::new(item.clone().fg(self.config.styles.item_fg)));
        }

        let list = List::default()
            .items(self.filtered_items.clone())
            .style(self.config.styles.popup_item_title_fg)
            .highlight_style(Style::new().bg(self.config.styles.item_selected_bg))
            .block(data_block);
        frame.render_stateful_widget(list, layout[1], &mut self.state);

        Ok(())
    }
}
