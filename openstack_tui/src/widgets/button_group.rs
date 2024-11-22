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

use crate::widgets::button::{Button, ButtonState};
use ratatui::{layout::Flex, prelude::*, widgets::StatefulWidget};

#[derive(Clone, Debug, Default)]
pub struct ButtonGroupState {
    selected: Option<usize>,
    count: Option<usize>,
}

impl ButtonGroupState {
    /// Create new ButtonGroup state and pre-select button
    pub fn new(selected: Option<usize>) -> Self {
        Self {
            selected,
            count: None,
        }
    }

    /// Select button specified
    pub fn select(&mut self, idx: Option<usize>) -> &mut Self {
        self.selected = idx;
        self
    }

    /// Get selected button index or None if all are unset
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Select next button.
    ///
    /// Note that count of buttons may be unknown yet in which case index is being set to 0 and
    /// will be corrected during the render.
    pub fn select_next(&mut self) {
        self.selected = match self.selected {
            None => Some(0),
            Some(x) => match self.count {
                None => Some(0),
                Some(cnt) => {
                    if x + 1 >= cnt {
                        Some(0)
                    } else {
                        Some(x + 1)
                    }
                }
            },
        }
    }

    /// Select previous button.
    ///
    /// Note that count of buttons may be unknown yet in which case index is being set to
    /// usize::MAX and will be corrected during the render.
    pub fn select_previous(&mut self) {
        self.selected = match self.selected {
            None => Some(usize::MAX),
            Some(x) => match self.count {
                None => Some(usize::MAX),
                Some(cnt) => {
                    if x == 0 {
                        Some(cnt - 1)
                    } else {
                        Some(x - 1)
                    }
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonGroup<'a> {
    buttons: Vec<Button<'a>>,
}

impl<'a> From<Vec<Button<'a>>> for ButtonGroup<'a> {
    fn from(data: Vec<Button<'a>>) -> Self {
        Self { buttons: data }
    }
}

impl StatefulWidget for &ButtonGroup<'_> {
    type State = ButtonGroupState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.count.is_none() {
            state.count = Some(self.buttons.len());
            if let Some(usize::MAX) = state.selected {
                state.select(Some(self.buttons.len()));
            }
        }
        let buttons_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(15); self.buttons.len()])
            .spacing(2)
            .flex(Flex::Center)
            .split(area);
        for (idx, button) in self.buttons.iter().enumerate() {
            let mut button_state = if let Some(selected) = state.selected {
                if idx == selected {
                    ButtonState::Selected
                } else {
                    ButtonState::Normal
                }
            } else {
                ButtonState::Normal
            };
            <&Button as StatefulWidget>::render(
                button,
                buttons_layout[idx],
                buf,
                &mut button_state,
            );
        }
    }
}
