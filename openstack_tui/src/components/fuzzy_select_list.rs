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

use crate::{action::Action, components::Component, config::Config, error::TuiError};
use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

/// State for a fuzzy-select list: owns filtered items, input text, and cursor position.
///
/// Used by both the legacy [`FuzzySelectList`] (Component) and
/// [`crate::components::FuzzySelectPopup`] (StatefulWidget).
pub struct FuzzySelectState {
    pub all_items: Vec<String>,
    pub filtered_items: Vec<String>,
    pub input: Option<String>,
    pub init_time: std::time::Instant,
    pub list_state: ListState,
}

impl Default for FuzzySelectState {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzySelectState {
    pub fn new() -> Self {
        Self {
            all_items: Vec::new(),
            filtered_items: Vec::new(),
            input: Some(String::new()),
            init_time: std::time::Instant::now(),
            list_state: ListState::default(),
        }
    }

    /// Replaces items and resets filter.
    pub fn set_items<I, S>(&mut self, items: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_items = items.into_iter().map(|x| x.as_ref().to_string()).collect();
        self.filtered_items = self.all_items.clone();
        self.input = Some(String::new());
        self.init_time = std::time::Instant::now();
        self.list_state.select(None);
        if !self.filtered_items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    /// Returns the currently selected name (if any).
    pub fn selected(&self) -> Option<&String> {
        self.list_state
            .selected()
            .and_then(|pos| self.filtered_items.get(pos))
    }

    /// Returns the currently selected index (if any).
    pub fn selected_index(&self) -> Option<usize> {
        self.list_state.selected()
    }

    /// Apply filter based on current input text.
    fn apply_filter(&mut self) {
        let filter = self.input.clone();
        self.filtered_items = self
            .all_items
            .iter()
            .filter(|x| {
                filter
                    .as_deref()
                    .map(|f| x.to_lowercase().contains(&f.to_lowercase()))
                    .unwrap_or(true)
            })
            .cloned()
            .collect();
        self.list_state.select(None);
        if self.list_state.selected().is_none() && !self.filtered_items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn handle_key(&mut self, key: &KeyCode) {
        match key {
            KeyCode::Down | KeyCode::Char('j') => self.list_state.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Home => self.list_state.select_first(),
            KeyCode::End => self.list_state.select_last(),
            KeyCode::Backspace | KeyCode::Delete => {
                if let Some(ref mut input) = self.input {
                    input.pop();
                    if input.is_empty() {
                        self.input = Some(String::new());
                    }
                }
                self.apply_filter();
            }
            KeyCode::Esc => {
                self.input = Some(String::new());
                self.init_time = std::time::Instant::now();
                self.filtered_items = self.all_items.clone();
                self.list_state.select(None);
                if !self.filtered_items.is_empty() {
                    self.list_state.select(Some(0));
                }
            }
            KeyCode::Char(c) => {
                self.input.get_or_insert_with(String::new).push(*c);
                self.apply_filter();
            }
            _ => {}
        }
    }

    pub fn reset_filter(&mut self) {
        self.input = Some(String::new());
        self.filtered_items = self.all_items.clone();
        self.list_state.select(None);
        if !self.filtered_items.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn cursor_up(&mut self) {
        self.list_state.select_previous();
    }
    pub fn cursor_down(&mut self) {
        self.list_state.select_next();
    }
    pub fn cursor_first(&mut self) {
        self.list_state.select_first();
    }
    pub fn cursor_last(&mut self) {
        self.list_state.select_last();
    }
}

/// (Legacy) Component that wraps a fuzzy-select list with input box and scroll.
pub struct FuzzySelectList {
    config: Config,
    state: FuzzySelectState,
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
            state: FuzzySelectState::new(),
        }
    }

    pub fn set_items<I, S>(&mut self, items: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state.set_items(items);
        self
    }

    pub fn selected(&self) -> Option<&String> {
        self.state.selected()
    }

    pub fn reset_filter(&mut self) -> Result<()> {
        self.state.reset_filter();
        Ok(())
    }
}

impl Component for FuzzySelectList {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.state.handle_key(&key.code);
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.fg));
        let input =
            Paragraph::new(self.state.input.as_deref().unwrap_or_default()).block(input_block);
        frame.render_widget(input, area);

        let data_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.buffer_bg));
        let mut list_items = Vec::new();
        for item in &self.state.filtered_items {
            list_items.push(ListItem::new(item.as_str()).fg(self.config.styles.item_fg));
        }
        let list = List::default()
            .items(list_items)
            .style(self.config.styles.popup_item_title_fg)
            .highlight_style(Style::new().bg(self.config.styles.item_selected_bg))
            .block(data_block);
        frame.render_stateful_widget(list, area, &mut self.state.list_state);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(items: &'static [&str]) -> FuzzySelectState {
        let mut s = FuzzySelectState::new();
        s.set_items(items);
        s
    }

    // ── construction ────────────────────────────────────────────

    #[test]
    fn new_is_empty() {
        let s = FuzzySelectState::new();
        assert!(s.all_items.is_empty());
        assert!(s.filtered_items.is_empty());
        assert_eq!(s.input.as_deref(), Some(""));
        assert!(s.list_state.selected().is_none());
    }

    #[test]
    fn set_items_populates_and_selects_first() {
        let mut s = FuzzySelectState::new();
        s.set_items(["alpha", "beta", "gamma"]);
        assert_eq!(s.all_items, ["alpha", "beta", "gamma"]);
        assert_eq!(s.filtered_items, s.all_items);
        assert_eq!(s.selected_index(), Some(0));
    }

    #[test]
    fn set_items_empty_yields_no_selection() {
        let mut s = FuzzySelectState::new();
        s.set_items::<Vec<&str>, _>(vec![]);
        assert!(s.filtered_items.is_empty());
        assert!(s.selected_index().is_none());
    }

    // ── filtering ───────────────────────────────────────────────

    #[test]
    fn filter_substring_case_insensitive() {
        let mut s = make_state(&["Hello", "world", "HELPER", "foo"]);
        s.input = Some("he".to_string());
        s.apply_filter();
        assert_eq!(s.filtered_items, ["Hello", "HELPER"]);
    }

    #[test]
    fn filter_no_match() {
        let mut s = make_state(&["one", "two", "three"]);
        s.input = Some("zzz".to_string());
        s.apply_filter();
        assert!(s.filtered_items.is_empty());
    }

    #[test]
    fn filter_none_selects_all() {
        let mut s = make_state(&["a", "b", "c"]);
        let items_before = s.filtered_items.clone();
        s.input = None;
        s.apply_filter();
        assert_eq!(s.filtered_items, items_before);
    }

    #[test]
    fn filter_resets_selection_to_first() {
        let mut s = make_state(&["a", "b", "c"]);
        s.list_state.select(Some(2));
        s.input = Some("a".to_string());
        s.apply_filter();
        assert_eq!(s.selected_index(), Some(0));
    }

    // ── keyboard navigation ─────────────────────────────────────

    #[test]
    fn handle_key_char_filters() {
        let mut s = make_state(&["foo", "bar", "baz"]);
        s.handle_key(&KeyCode::Char('b'));
        assert!(s.filtered_items.iter().all(|i| i.starts_with('b')));
    }

    #[test]
    fn handle_key_backspace_removes_char() {
        let mut s = make_state(&["foo", "bar", "baz"]);
        s.handle_key(&KeyCode::Char('b'));
        assert_eq!(s.filtered_items.len(), 2);
        s.handle_key(&KeyCode::Backspace);
        assert_eq!(s.filtered_items.len(), 3);
    }

    #[test]
    fn handle_key_esc_clears_filter() {
        let mut s = make_state(&["foo", "bar", "baz"]);
        s.handle_key(&KeyCode::Char('b'));
        assert!(s.filtered_items.len() < 3);
        s.handle_key(&KeyCode::Esc);
        assert_eq!(s.filtered_items.len(), 3);
        assert_eq!(s.input.as_deref(), Some(""));
        assert_eq!(s.selected_index(), Some(0));
    }

    #[test]
    fn arrow_down_moves_cursor() {
        let mut s = make_state(&["a", "b", "c"]);
        assert_eq!(s.selected_index(), Some(0));
        s.handle_key(&KeyCode::Down);
        assert_eq!(s.selected_index(), Some(1));
        s.handle_key(&KeyCode::Down);
        assert_eq!(s.selected_index(), Some(2));
    }

    #[test]
    fn arrow_up_moves_cursor() {
        let mut s = make_state(&["a", "b", "c"]);
        s.handle_key(&KeyCode::Down);
        s.handle_key(&KeyCode::Down);
        assert_eq!(s.selected_index(), Some(2));
        s.handle_key(&KeyCode::Up);
        assert_eq!(s.selected_index(), Some(1));
    }

    #[test]
    fn home_end_keys() {
        let mut s = make_state(&["a", "b", "c"]);
        s.handle_key(&KeyCode::End);
        // ListState::select_last sets index to usize::MAX; clamping to content size
        // happens at render time, not in the state.
        assert!(s.selected_index().is_some());
        s.handle_key(&KeyCode::Home);
        assert_eq!(s.selected_index(), Some(0));
    }

    #[test]
    fn jk_keys_are_aliases_for_arrows() {
        let mut s = make_state(&["a", "b", "c"]);
        s.handle_key(&KeyCode::Char('j'));
        assert_eq!(s.selected_index(), Some(1));
        s.handle_key(&KeyCode::Char('k'));
        assert_eq!(s.selected_index(), Some(0));
    }

    // ── cursor helpers ──────────────────────────────────────────

    #[test]
    fn cursor_first_moves_to_zero() {
        let mut s = make_state(&["a", "b", "c"]);
        s.cursor_down();
        s.cursor_first();
        assert_eq!(s.selected_index(), Some(0));
    }

    #[test]
    fn cursor_last_moves_to_end() {
        let mut s = make_state(&["a", "b", "c"]);
        s.cursor_last();
        // ListState::select_last sets to usize::MAX; actual last-item clamping
        // occurs at render time, so selected() returns None here.
        assert!(s.selected_index().is_some());
        assert_eq!(s.selected(), None); // usize::MAX out of bounds
    }

    #[test]
    fn cursor_on_empty_list() {
        let mut s = FuzzySelectState::new();
        s.cursor_down();
        // ListState on empty list wraps to usize::MAX
        assert!(s.selected_index().is_some());
        assert_eq!(s.selected(), None);
    }

    // ── selected ────────────────────────────────────────────────

    #[test]
    fn selected_returns_item_name() {
        let mut s = make_state(&["one", "two", "three"]);
        s.cursor_down();
        assert_eq!(s.selected().map(String::as_str), Some("two"));
    }

    #[test]
    fn selected_on_empty_is_none() {
        let s = FuzzySelectState::new();
        assert!(s.selected().is_none());
    }

    // ── reset_filter ────────────────────────────────────────────

    #[test]
    fn reset_filter_restores_all_items() {
        let mut s = make_state(&["alpha", "beta", "gamma"]);
        s.input = Some("gamma".to_string());
        s.apply_filter();
        assert_eq!(s.filtered_items, ["gamma"]);
        s.reset_filter();
        assert_eq!(s.filtered_items, s.all_items);
        assert_eq!(s.input.as_deref(), Some(""));
        assert_eq!(s.selected_index(), Some(0));
    }
}
