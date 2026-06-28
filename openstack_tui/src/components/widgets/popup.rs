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

/// Generic popup widget usable for any modal dialog in the TUI.
///
/// It renders a centered block with optional title, bottom instruction line
/// and arbitrary inner content supplied by the caller.
pub struct Popup<C> {
    config: Config,
    title: Line<'static>,
    bottom_title: Option<Line<'static>>,
    content: C,
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
        }
    }

    pub fn with_bottom_title(mut self, txt: impl Into<Line<'static>>) -> Self {
        self.bottom_title = Some(txt.into());
        self
    }

    pub fn set_content<T>(&mut self, new_content: T)
    where
        T: Widget,
    {
        // SAFETY: we replace the stored content via transmute; used only when caller knows the type.
        // For simplicity we keep the original generic type; this method is primarily for mutable refs.
        // In practice popups are constructed once and not changed.
        // TODO: consider redesigning to avoid this limitation.
        unsafe { std::ptr::write(&mut self.content as *mut C as *mut T, new_content) }
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
        // Use a reasonable size; callers can adjust via custom content that respects its own constraints.
        let ar = centered_rect_fixed(70, 20, area);
        let mut block = Block::default()
            .title_top(self.title.clone().centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg);
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
