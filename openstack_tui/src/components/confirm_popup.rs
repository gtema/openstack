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

#![allow(dead_code, unused_imports, unused_mut)]

use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{block::*, *},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use crate::{
    action::Action,
    cloud_worker::types::{ApiRequest, ConfirmableRequest},
    components::Component,
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::centered_rect,
    widgets::{
        button::Button,
        button_group::{ButtonGroup, ButtonGroupState},
    },
};

pub struct ConfirmPopup {
    command_tx: Option<UnboundedSender<Action>>,
    request: ApiRequest,
    config: Config,
    button_group: ButtonGroup<'static>,
    button_group_state: ButtonGroupState,
}

impl ConfirmPopup {
    pub fn new(request: &ApiRequest) -> Self {
        Self {
            command_tx: None,
            request: request.clone(),
            config: Config::default(),
            button_group: ButtonGroup::from(vec![Button::new("Cancel (Esc)"), Button::new("OK")]),
            button_group_state: ButtonGroupState::new(Some(0)),
        }
    }

    pub fn render_tick(&mut self) {}
}

impl Component for ConfirmPopup {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        match key.code {
            KeyCode::Left | KeyCode::BackTab => {
                self.button_group_state.select_previous();
            }
            KeyCode::Right | KeyCode::Tab => {
                self.button_group_state.select_next();
            }
            KeyCode::Enter => {
                match self.button_group_state.selected() {
                    Some(0) => {
                        // Cancel
                        if let Some(command_tx) = &self.command_tx {
                            command_tx.send(Action::ConfirmRejected(self.request.clone()))?;
                        }
                    }
                    Some(1) => {
                        // Approve
                        if let Some(command_tx) = &self.command_tx {
                            command_tx.send(Action::ConfirmAccepted(self.request.clone()))?;
                        }
                    }
                    _ => {}
                }
            }
            KeyCode::Esc => {
                // Cancel
                if let Some(command_tx) = &self.command_tx {
                    command_tx.send(Action::ConfirmRejected(self.request.clone()))?;
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let ar = centered_rect(30, 20, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(" Confirm ").light_yellow().centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg)
            .border_style(Style::default().fg(self.config.styles.popup_border_confirm_fg));

        let line: Line = Line::from(
            self.request
                .get_confirm_message()
                .clone()
                .unwrap_or(String::from("Are you sure?")),
        )
        .alignment(Alignment::Center)
        .white();

        let text = vec![line];
        let text_buttons_layout = Layout::default()
            .constraints([Constraint::Percentage(100), Constraint::Length(1)].as_ref())
            .split(popup_block.inner(ar));

        let paragraph = Paragraph::new(text);

        frame.render_widget(Clear, ar);
        frame.render_widget(popup_block, ar);
        frame.render_widget(paragraph, text_buttons_layout[0]);
        frame.render_stateful_widget(
            &self.button_group,
            text_buttons_layout[1],
            &mut self.button_group_state,
        );

        Ok(())
    }
}
