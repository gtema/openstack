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

use crate::components::FuzzySelectState;
use crate::config::Config;
use crate::utils::centered_rect_fixed;
use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::widgets::{
    Block, BorderType, Borders, Clear, List, ListItem, Padding, Paragraph, StatefulWidget,
};

/// State for the fuzzy-select popup widget.
pub struct FuzzySelectPopupState {
    state: FuzzySelectState,
}

impl Default for FuzzySelectPopupState {
    fn default() -> Self {
        Self::new()
    }
}

impl FuzzySelectPopupState {
    pub fn new() -> Self {
        Self {
            state: FuzzySelectState::new(),
        }
    }

    /// Replaces all items and resets filter.
    pub fn set_items<I, S>(&mut self, items: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state.set_items(items);
    }

    /// Returns the currently selected name (if any).
    pub fn selected(&self) -> Option<&String> {
        self.state.selected()
    }

    /// Returns the currently selected index (if any).
    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected_index()
    }

    /// Process a key event: navigation, input, filter.
    pub fn handle_key(&mut self, key: &KeyCode) {
        self.state.handle_key(key);
    }

    /// Get a reference to the inner fuzzy state for advanced access.
    pub fn inner(&self) -> &FuzzySelectState {
        &self.state
    }

    /// Get a mutable reference to the inner fuzzy state.
    pub fn inner_mut(&mut self) -> &mut FuzzySelectState {
        &mut self.state
    }
}

// ---------------------------------------------------------------------------
// FuzzySelectList  –  renders the filter input + list in a split area
// ---------------------------------------------------------------------------

/// ratatui StatefulWidget that renders a filter input box on top and
/// a scrollable list of filtered items below.
///
/// Takes `&mut FuzzySelectState` and config.
pub struct FuzzySelectList<'a> {
    pub config: &'a Config,
}

impl<'a> FuzzySelectList<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl StatefulWidget for FuzzySelectList<'_> {
    type State = FuzzySelectState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if area.height < 5 {
            return;
        }

        // Input (3 rows: top border + content + bottom border) at rows 0-2, list from row 3+.
        // list_inner.y already skipped title_top (+1), so row 0 of list_inner is usable.
        let input_area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 3,
        };
        let list_area = Rect {
            x: area.x,
            y: area.y + 3,
            width: area.width,
            height: area.height.saturating_sub(3),
        };

        // --- Input line (shows filter text) ---
        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.title_filters_fg));
        let filter_text = state.input.as_deref().unwrap_or("");
        let cursor_visible = (state.init_time.elapsed().as_secs() % 2) == 0;
        let cursor = if cursor_visible { "█" } else { " " };
        let input_line = Text::from(Line::from(format!("{}{}", filter_text, cursor)));
        let input = Paragraph::new(input_line)
            .block(input_block)
            .style(Style::default().fg(self.config.styles.item_fg));
        input.render(input_area, buf);

        // --- List (shows filtered items) ---
        let filtered: Vec<String> = state.filtered_items.clone();
        let list_items: Vec<ListItem> = filtered
            .iter()
            .map(|item| ListItem::new(item.as_str()).fg(self.config.styles.item_fg))
            .collect();

        let widget_list = List::default()
            .items(list_items)
            .style(Style::default().fg(self.config.styles.popup_item_title_fg))
            .highlight_style(Style::new().bg(self.config.styles.item_selected_bg))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(self.config.styles.popup_border_fg)),
            );

        StatefulWidget::render(widget_list, list_area, buf, &mut state.list_state);
    }
}

// ---------------------------------------------------------------------------
// FuzzySelectPopup  –  centered overlay block that embeds FuzzySelectList
// ---------------------------------------------------------------------------

/// Stateless ratatui StatefulWidget: centered popup overlay block that
/// embeds a [`FuzzySelectList`] in its inner area.
pub struct FuzzySelectPopup<'a> {
    config: &'a Config,
    title: Line<'a>,
    is_loading: bool,
    width_percent: u16,
}

impl<'a> FuzzySelectPopup<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            title: Line::default(),
            is_loading: false,
            width_percent: 60,
        }
    }

    pub fn title(mut self, title: impl Into<Line<'a>>) -> Self {
        self.title = title.into();
        self
    }

    pub fn loading(mut self, v: bool) -> Self {
        self.is_loading = v;
        self
    }
}

impl StatefulWidget for FuzzySelectPopup<'_> {
    type State = FuzzySelectPopupState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Compute popup height based on item count; cap to available screen.
        let item_count = state.inner().all_items.len().max(1) as u16;
        // Overhead: 2 (popup_block borders) + 2 (list_inner title overlays) + 3 (input area) + 2 (list borders)
        const HEIGHT_OVERHEAD: u16 = 9;
        const POPUP_HEIGHT_MIN: u16 = 12;
        let popup_height = (item_count + HEIGHT_OVERHEAD)
            .max(POPUP_HEIGHT_MIN)
            .min(area.height);

        let area = centered_rect_fixed(self.width_percent, popup_height, area);

        let mut title_vec: Vec<Span> = self.title.spans.to_vec();
        if self.is_loading {
            title_vec.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        }

        let popup_block = Block::default()
            .title_top(Line::from(title_vec).centered())
            .title_bottom(
                Line::from(" (↑/↓) navigate | (Enter) select | (Esc) reset | type to filter")
                    .gray()
                    .right_aligned(),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .bg(self.config.styles.popup_bg)
            .padding(Padding::horizontal(1));

        let inner = popup_block.inner(area);

        Clear::render(Clear, area, buf);
        popup_block.render(area, buf);

        // Inner area already accounts for borders and padding.
        // list_inner skips title_top (+1) and title_bottom (-2) overlays.
        let list_inner = Rect {
            x: inner.x,
            y: inner.y + 1,
            width: inner.width,
            height: inner.height.saturating_sub(2),
        };

        FuzzySelectList::new(self.config).render(list_inner, buf, state.inner_mut());
    }
}
