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

//! Single line text input used to type a search query.

use ratatui::{prelude::*, widgets::*};

use crate::{components::Frame, config::Config};

/// Height of the rendered input (single text line plus borders)
pub const SEARCH_INPUT_HEIGHT: u16 = 3;

/// Search query together with the "being typed" state.
#[derive(Debug, Default)]
pub struct SearchInput {
    text: String,
    active: bool,
}

impl SearchInput {
    /// Current query
    pub fn value(&self) -> &str {
        &self.text
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Whether the query is currently being typed
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn push(&mut self, c: char) {
        self.text.push(c);
    }

    pub fn pop(&mut self) {
        self.text.pop();
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// Render the input in the given (full width) area
    pub fn draw(&self, f: &mut Frame<'_>, area: Rect, config: &Config) {
        let widget = Paragraph::new(Line::from(vec![
            Span::styled(" /", config.styles.title_filters_fg),
            Span::raw(self.text.as_str()),
            Span::styled("█", config.styles.title_filters_fg),
        ]))
        .style(
            Style::new()
                .fg(config.styles.table.row_fg)
                .bg(config.styles.buffer_bg),
        )
        .block(
            Block::bordered()
                .title(" Search ")
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(config.styles.border_fg)),
        );
        f.render_widget(widget, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edits_text() {
        let mut input = SearchInput::default();
        assert!(input.is_empty());
        input.push('a');
        input.push('b');
        input.pop();
        assert_eq!(input.value(), "a");
        input.clear();
        assert!(input.is_empty());
    }
}
