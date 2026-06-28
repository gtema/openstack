// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::config::Config;

use super::state::FuzzySelectState;

/// ratatui StatefulWidget: filter input + list of string options, with optional popup overlay.
///
/// Use `.as_popup(true)` to enable centered popup styling (overlay block, title, loading
/// indicator, help text). Without it, renders inline in the given area.
pub struct FuzzySelect<'a> {
    config: &'a Config,
    title: Line<'a>,
    is_loading: bool,
    as_popup: bool,
    popup_width_percent: u16,
}

impl<'a> FuzzySelect<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self {
            config,
            title: Line::default(),
            is_loading: false,
            as_popup: false,
            popup_width_percent: 60,
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

    /// Enable centered popup overlay styling (default: false).
    pub fn as_popup(mut self, v: bool) -> Self {
        self.as_popup = v;
        self
    }

    /// Set popup width as percentage of available space (default: 60).
    pub fn popup_width_percent(mut self, v: u16) -> Self {
        self.popup_width_percent = v;
        self
    }
}

impl StatefulWidget for FuzzySelect<'_> {
    type State = FuzzySelectState;

    fn render(self, area: Rect, buf: &mut Buffer, widget_state: &mut Self::State) {
        if area.height < 5 {
            return;
        }

        if self.as_popup && area.height >= 10 {
            use crate::utils::centered_rect_fixed;

            let item_count = widget_state.all_items.len().max(1) as u16;
            const OVERHEAD: u16 = 9;
            let popup_height = (item_count + OVERHEAD).max(10).min(area.height);
            let popup_width_percent = self.popup_width_percent.clamp(20, 95);
            let popup_area = centered_rect_fixed(popup_width_percent, popup_height, area);

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
                    Line::from("(↑/↓) navigate | (Enter) select | (Esc) reset | type to filter")
                        .gray()
                        .right_aligned(),
                )
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .bg(self.config.styles.popup_bg)
                .padding(Padding::horizontal(1));

            let inner = popup_block.inner(popup_area);
            let list_inner = Rect {
                x: inner.x,
                y: inner.y + 1,
                width: inner.width,
                height: inner.height.saturating_sub(2),
            };

            // Update page size based on visible list area height
            // (Set before inner render so PageDown/PageUp uses current viewport)
            widget_state.set_popup_height(list_inner.height);

            Clear::render(Clear, popup_area, buf);
            popup_block.render(popup_area, buf);

            FuzzySelect {
                config: self.config,
                title: Line::default(),
                is_loading: false,
                as_popup: false,
                popup_width_percent: 0,
            }
            .render(list_inner, buf, widget_state);
        } else {
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

            let input_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.config.styles.fg));
            let filter_text = widget_state.input.as_deref().unwrap_or("");
            let cursor_visible = (widget_state.init_time.elapsed().as_millis() % 200) < 100;
            let cursor = if cursor_visible { "█" } else { " " };
            let input_line = Text::from(Line::from(format!("{}{}", filter_text, cursor)));
            let input = Paragraph::new(input_line)
                .block(input_block)
                .style(Style::default().fg(self.config.styles.item_fg));
            input.render(input_area, buf);

            let list_items: Vec<ListItem> = widget_state
                .filtered_items
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
                        .border_style(Style::default().fg(self.config.styles.buffer_bg)),
                );

            StatefulWidget::render(widget_list, list_area, buf, &mut widget_state.list_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widgets::fuzzy_select::state::FuzzySelectState;

    fn area(w: u16, h: u16) -> Rect {
        Rect::new(0, 0, w, h)
    }

    fn config() -> Config {
        Config::default()
    }

    fn s(items: &[&str]) -> FuzzySelectState {
        let mut st = FuzzySelectState::new();
        st.set_items(items);
        st
    }

    fn cell(buf: &Buffer, x: u16, y: u16) -> &str {
        buf.cell((x, y)).map(|c| c.symbol()).unwrap_or(" ")
    }

    fn line(buf: &Buffer, y: u16, xs: std::ops::Range<u16>) -> String {
        xs.map(|x| {
            buf.cell((x, y))
                .map(|c| c.symbol().chars().next().unwrap_or(' '))
                .unwrap_or(' ')
        })
        .collect()
    }

    #[test]
    fn render_tiny_area_returns_early() {
        let a = area(80, 4);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut FuzzySelectState::new());
        assert_eq!(cell(&buf, 0, 0), " ");
    }

    #[test]
    fn render_inline_shows_input_and_list() {
        let a = area(40, 10);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut s(&["a", "b"]));
        assert_eq!(cell(&buf, 0, 0), "┌");
        assert_eq!(cell(&buf, 0, 3), "┌");
    }

    #[test]
    fn render_inline_shows_filtered_items() {
        let a = area(30, 8);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut s(&["one", "two"]));
        assert_eq!(cell(&buf, 1, 4), "o");
    }

    #[test]
    fn render_inline_empty_state() {
        let a = area(30, 8);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut FuzzySelectState::new());
        assert_eq!(cell(&buf, 0, 3), "┌");
    }

    #[test]
    fn render_popup_shows_overlay() {
        let a = area(80, 20);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config())
            .as_popup(true)
            .title("Title")
            .render(a, &mut buf, &mut s(&["a"]));
        assert_eq!(cell(&buf, 0, 0), " ");
        assert_ne!(cell(&buf, 10, 5), " ");
    }

    #[test]
    fn render_popup_fallback_to_inline() {
        let a = area(80, 9);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config())
            .as_popup(true)
            .render(a, &mut buf, &mut s(&["a"]));
        assert_eq!(cell(&buf, 0, 0), "┌");
    }

    #[test]
    fn render_popup_shows_loading_in_title() {
        let a = area(80, 20);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config())
            .as_popup(true)
            .title("X")
            .loading(true)
            .render(a, &mut buf, &mut s(&["a"]));
        let t = line(&buf, 5, 0..a.width);
        assert!(t.contains("Loading"), "'Loading' not in buffer row: {}", t);
    }

    #[test]
    fn render_filter_text_visible_in_input() {
        let a = area(40, 10);
        let mut st = s(&["alpha", "beta"]);
        st.input = Some("be".to_string());
        st.filtered_items = vec!["beta".to_string()];
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut st);
        assert_eq!(cell(&buf, 1, 1), "b");
        assert_eq!(cell(&buf, 2, 1), "e");
    }

    #[test]
    fn render_width_percent_clamped_no_panic() {
        let a = area(80, 20);
        let mut buf = Buffer::empty(a);
        let mut st = s(&["a"]);
        FuzzySelect::new(&config())
            .as_popup(true)
            .popup_width_percent(5)
            .render(a, &mut buf, &mut st);
        FuzzySelect::new(&config())
            .as_popup(true)
            .popup_width_percent(100)
            .render(a, &mut buf, &mut st);
    }

    #[test]
    fn render_boundary_height_5_no_op() {
        let a = area(40, 4);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut s(&["x"]));
        assert_eq!(cell(&buf, 0, 0), " ");
    }

    #[test]
    fn render_boundary_height_6_inline() {
        let a = area(30, 6);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config()).render(a, &mut buf, &mut s(&["x"]));
        assert_eq!(cell(&buf, 0, 0), "┌");
    }

    #[test]
    fn render_popup_empty_state() {
        let a = area(80, 20);
        let mut buf = Buffer::empty(a);
        FuzzySelect::new(&config())
            .as_popup(true)
            .title("Empty")
            .render(a, &mut buf, &mut FuzzySelectState::new());
        assert_ne!(cell(&buf, 0, 0), "┌");
        assert_ne!(cell(&buf, 10, 5), " ");
    }

    #[test]
    fn builder_methods_set_fields() {
        let c = config();
        let w = FuzzySelect::new(&c)
            .title("MyTitle")
            .loading(true)
            .as_popup(true)
            .popup_width_percent(80);
        assert!(w.is_loading);
        assert!(w.as_popup);
        assert_eq!(w.popup_width_percent, 80);
        let t: String = w.title.spans.iter().map(|s| s.content.clone()).collect();
        assert_eq!(t, "MyTitle");
    }
}
