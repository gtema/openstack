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
// #![allow(dead_code, unused_imports, unused_mut)]
use ratatui::{
    prelude::*,
    widgets::{StatefulWidget, Widget},
};

#[derive(Debug, Clone)]
pub struct Button<'text> {
    text: Text<'text>,
    theme: Theme,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    #[default]
    Normal,
    Selected,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    normal_text: Color,
    normal_background: Color,
    selected_text: Color,
    selected_background: Color,
}

impl Default for Theme {
    fn default() -> Self {
        themes::NORMAL
    }
}

/// Config
impl<'text> Button<'text> {
    pub fn new<T: Into<Text<'text>>>(text: T) -> Self {
        Self {
            text: text.into(),
            theme: Theme::default(),
        }
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
}

impl Widget for &Button<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let theme = self.theme;

        let fg = theme.normal_text;
        let bg = theme.normal_background;
        buf.set_style(area, (fg, bg));

        self.text.clone().centered().render(area, buf);
    }
}

impl StatefulWidget for &Button<'_> {
    type State = ButtonState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let theme = self.theme;

        let style = match state {
            ButtonState::Normal => Style::new()
                .fg(theme.normal_text)
                //.bg(theme.normal_background)
                .add_modifier(Modifier::DIM),
            ButtonState::Selected => Style::new()
                .fg(theme.selected_text)
                .bg(theme.selected_background)
                .add_modifier(Modifier::BOLD),
        };

        buf.set_style(area, style);
        self.text.clone().centered().render(area, buf);
    }
}

pub mod themes {
    use super::Theme;
    use ratatui::style::palette::tailwind;

    pub const NORMAL: Theme = Theme {
        normal_text: tailwind::SLATE.c300,
        normal_background: tailwind::SLATE.c800,
        selected_text: tailwind::SLATE.c50,
        selected_background: tailwind::BLUE.c400,
    };

    pub const RED: Theme = Theme {
        normal_text: tailwind::RED.c200,
        normal_background: tailwind::RED.c800,
        selected_text: tailwind::RED.c100,
        selected_background: tailwind::RED.c700,
    };

    pub const GREEN: Theme = Theme {
        normal_text: tailwind::GREEN.c200,
        normal_background: tailwind::GREEN.c800,
        selected_text: tailwind::GREEN.c100,
        selected_background: tailwind::GREEN.c700,
    };

    pub const BLUE: Theme = Theme {
        normal_text: tailwind::BLUE.c200,
        normal_background: tailwind::BLUE.c800,
        selected_text: tailwind::BLUE.c100,
        selected_background: tailwind::BLUE.c700,
    };
}
