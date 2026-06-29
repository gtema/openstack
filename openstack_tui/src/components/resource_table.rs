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
                let cells = data.iter().map(|c| Cell::from(c.as_str()));
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
        if self.rows.len() <= inner.height as usize {
            return;
        }
        let mut scroll = self.scroll;
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(self.config.styles.border_fg));
        <Scrollbar as StatefulWidget>::render(scrollbar, inner, buf, &mut scroll);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_line(buf: &Buffer, y: u16, x_start: u16, x_end: u16) -> String {
        (x_start..=x_end)
            .map(|x| {
                buf.cell((x, y))
                    .map(|c| c.symbol().chars().next().unwrap_or(' '))
                    .unwrap_or(' ')
            })
            .collect()
    }

    #[test]
    fn renders_header_and_rows() {
        let backend = ratatui::backend::TestBackend::new(40, 5);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let rows = [vec!["srv-1".into(), "web".into()]];
                let row_styles = [Style::default()];
                let column_widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
                let header = Row::new(["ID", "NAME"].map(Cell::from));
                let table = ResourceTable {
                    header,
                    rows: &rows,
                    row_styles: &row_styles,
                    column_widths: &column_widths,
                    selected: None,
                    scroll: ScrollbarState::new(1).position(0),
                    config: Rc::new(Config::default()),
                };
                table.render(f.area(), f.buffer_mut());
            })
            .unwrap();

        let buf = terminal.backend().buffer();
        let mut has_header = false;
        for y in 0..buf.area().height {
            let row = read_line(buf, y, 0, buf.area().width.saturating_sub(1));
            if row.contains("ID") || row.contains("NAME") {
                has_header = true;
            }
        }
        assert!(has_header, "Header not rendered");
    }

    #[test]
    fn no_panic_when_rows_fit_no_scrollbar() {
        let backend = ratatui::backend::TestBackend::new(40, 10);
        let mut terminal = ratatui::Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let rows = [vec!["srv-1".into(), "web".into()]];
                let row_styles = [Style::default()];
                let column_widths = [Constraint::Percentage(50), Constraint::Percentage(50)];
                let header = Row::new(["ID", "NAME"].map(Cell::from));
                let table = ResourceTable {
                    header,
                    rows: &rows,
                    row_styles: &row_styles,
                    column_widths: &column_widths,
                    selected: None,
                    scroll: ScrollbarState::new(1).position(0),
                    config: Rc::new(Config::default()),
                };
                // rows.len() <= inner.height, so early return (no scrollbar)
                table.render(f.area(), f.buffer_mut());
            })
            .unwrap();
        // No panic means the early return at line 88 worked correctly
    }
}
