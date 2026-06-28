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

use crate::{components::Component, config::Config, utils::centered_rect_fixed};
use ratatui::{prelude::*, widgets::*};

/// Popup width specification
enum PopupWidth {
    /// Fixed character width
    Fixed(u16),
    /// Percentage of available width
    Percentage(u16),
}

/// Generic popup widget usable for any modal dialog in the TUI.
///
/// It renders a centered block with optional title, bottom instruction line
/// and arbitrary inner content supplied by the caller.
pub struct Popup<C> {
    config: Config,
    title: Line<'static>,
    bottom_title: Option<Line<'static>>,
    content: C,
    width: PopupWidth,
    border_color: Option<Color>,
}

impl<C> Popup<C>
where
    C: Widget,
{
    pub fn new(config: Config, title: impl Into<Line<'static>>, content: C) -> Self {
        Self {
            config,
            title: title.into(),
            bottom_title: None,
            content,
            width: PopupWidth::Percentage(70),
            border_color: None,
        }
    }

    pub fn with_bottom_title(mut self, txt: impl Into<Line<'static>>) -> Self {
        self.bottom_title = Some(txt.into());
        self
    }

    pub fn with_width_chars(mut self, chars: u16) -> Self {
        self.width = PopupWidth::Fixed(chars);
        self
    }

    pub fn with_width_percent(mut self, percent: u16) -> Self {
        self.width = PopupWidth::Percentage(percent);
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}

impl<C> Component for Popup<C>
where
    C: Widget + Clone,
{
    fn register_config_handler(&mut self, config: Config) -> Result<(), crate::error::TuiError> {
        self.config = config;
        Ok(())
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), crate::error::TuiError> {
        let popup_width = match &self.width {
            PopupWidth::Fixed(w) => *w,
            PopupWidth::Percentage(p) => (area.width as f64 * *p as f64 / 100.0) as u16,
        };
        let ar = centered_rect_fixed(popup_width, 20, area);
        let mut block = Block::default()
            .title_top(self.title.clone().centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg);
        if let Some(border_fg) = self.border_color {
            block = block.border_style(Style::default().fg(border_fg));
        }
        if let Some(bottom) = &self.bottom_title {
            block = block.title_bottom(bottom.clone().right_aligned());
        }
        // Render background clear first
        f.render_widget(Clear, ar);
        f.render_widget(block.clone(), ar);
        // Render inner content inside the inner area of the block
        f.render_widget(self.content.clone(), block.inner(ar));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

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
    fn popup_new_sets_fields() {
        let p: Popup<Paragraph> = Popup::new(Config::default(), "T", Paragraph::new("c"));
        assert_eq!(
            p.title
                .spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>(),
            "T"
        );
        assert!(p.bottom_title.is_none());
    }

    #[test]
    fn popup_with_bottom_title() {
        let p: Popup<Paragraph> =
            Popup::new(Config::default(), "T", Paragraph::new("")).with_bottom_title("bot");
        assert_eq!(
            p.bottom_title
                .as_ref()
                .unwrap()
                .spans
                .iter()
                .map(|s| s.content.clone())
                .collect::<String>(),
            "bot"
        );
    }

    #[test]
    fn popup_draw_renders_centered_block_with_title() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let mut p: Popup<Paragraph> =
                    Popup::new(Config::default(), "TestTitle", Paragraph::new("Hi"));
                p.draw(f, f.area()).unwrap();
            })
            .unwrap();

        let buf = terminal.backend().buffer();
        let mut found = false;
        for y in 0..buf.area().height {
            let row = read_line(buf, y, 0, buf.area().width - 1);
            if row.contains("TestTitle") {
                found = true;
                break;
            }
        }
        assert!(found, "TestTitle not found in buffer:\n{:#?}", buf);
    }

    #[test]
    fn popup_draw_with_bottom_title_shows() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let mut p: Popup<Paragraph> =
                    Popup::new(Config::default(), "Top", Paragraph::new(""))
                        .with_bottom_title("BottomLine");
                p.draw(f, f.area()).unwrap();
            })
            .unwrap();

        let buf = terminal.backend().buffer();
        let mut found = false;
        for y in 0..buf.area().height {
            let row = read_line(buf, y, 0, buf.area().width - 1);
            if row.contains("BottomLine") {
                found = true;
                break;
            }
        }
        assert!(found, "BottomLine not found in buffer:\n{:#?}", buf);
    }

    #[test]
    fn popup_draw_clears_background() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let block = Block::default().title("Old").borders(Borders::ALL);
                f.render_widget(block, f.area());
                let mut p: Popup<Paragraph> =
                    Popup::new(Config::default(), "Popup", Paragraph::new(""));
                p.draw(f, f.area()).unwrap();
            })
            .unwrap();

        let buf = terminal.backend().buffer();
        // 70% of 80 = 56, centered at x=(80-56)/2 = 12, y=(24-20)/2 = 2
        assert_ne!(buf.cell((12, 2)).unwrap().symbol(), " ");
    }

    #[test]
    fn popup_inner_smaller_than_popup() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let popup_width = (f.area().width as f64 * 70.0 / 100.0) as u16;
                let pa = crate::utils::centered_rect_fixed(popup_width, 20, f.area());
                let block = Block::default()
                    .padding(Padding::uniform(1))
                    .borders(Borders::ALL);
                let inner = block.inner(pa);
                assert!(inner.x > pa.x);
                assert!(inner.y > pa.y);
                assert!(inner.x + inner.width < pa.x + pa.width);
                assert!(inner.y + inner.height < pa.y + pa.height);
            })
            .unwrap();
    }

    #[test]
    fn popup_register_config_updates() {
        let mut p: Popup<Paragraph> = Popup::new(Config::default(), "X", Paragraph::new(""));
        let c = Config::default();
        p.register_config_handler(c.clone()).unwrap();
        assert_eq!(p.config.styles.popup_bg, c.styles.popup_bg);
    }
}
