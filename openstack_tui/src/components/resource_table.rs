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

use ratatui::{prelude::*, widgets::*};
use std::rc::Rc;

use crate::config::Config;

/// Generic table widget for any resource type.
/// Mirrors the rendering logic from `TableViewComponentBase::render_table`
/// but is stateless – all data is passed by reference.
pub struct ResourceTable<'a> {
    /// Header row (already upper‑cased).
    pub header: Row<'a>,
    /// Rows of string cells.
    pub rows: &'a [Vec<String>],
    /// Pre‑computed styles for each row.
    pub row_styles: &'a [Style],
    /// Column width constraints.
    pub column_widths: &'a [Constraint],
    /// Currently selected row index.
    pub selected: Option<usize>,
    /// Scrollbar state.
    pub scroll: ScrollbarState,
    /// Styling configuration (shared via Rc to avoid cloning).
    pub config: Rc<Config>,
}

impl<'a> Widget for ResourceTable<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Outer block that mirrors the original UI.
        let block = Block::default()
            .borders(Borders::RIGHT)
            .padding(Padding::right(1))
            .border_style(Style::default().fg(self.config.styles.border_fg));

        let inner = block.inner(area);
        let selected_style = Style::default().add_modifier(Modifier::REVERSED).fg(self
            .config
            .styles
            .table
            .row_fg_selected);

        let header = self.header.clone().style(
            Style::default()
                .fg(self.config.styles.table.header_fg)
                .bg(self.config.styles.table.header_bg),
        );

        let rows = self
            .rows
            .iter()
            .zip(self.row_styles.iter())
            .map(|(data, style)| {
                let cells = data.iter().map(|c| Cell::from(Text::from(c.clone())));
                Row::new(cells).style(*style).height(1)
            });

        let table = Table::default()
            .header(header)
            .rows(rows)
            .widths(self.column_widths.iter().cloned())
            .row_highlight_style(selected_style)
            .bg(self.config.styles.buffer_bg)
            .block(block)
            .highlight_spacing(HighlightSpacing::Always);

        // Use a fresh TableState to apply the highlight.
        let mut state = TableState::default();
        if let Some(s) = self.selected {
            state.select(Some(s));
        }
        ratatui::prelude::StatefulWidget::render(table, inner, buf, &mut state);

        // Render scrollbar when needed.
        if inner.height as usize * self.column_widths.len() < self.rows.len() {
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .style(Style::default().fg(self.config.styles.border_fg))
                .render(
                    area.inner(Margin {
                        vertical: 1,
                        horizontal: 1,
                    }),
                    buf,
                    &mut self.scroll.clone(),
                );
        }
    }
}
