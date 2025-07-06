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
use eyre::{Result, eyre};
use openstack_sdk::auth::auth_helper::{AuthHelper as SdkAuthHelper, AuthHelperError};
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{block::*, *},
};
use secrecy::SecretString;
use std::time::Duration;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot,
};
use tracing::{debug, info, instrument, trace};

use crate::{
    action::Action,
    cloud_worker::{
        AuthAction,
        types::{ApiRequest, ConfirmableRequest},
    },
    components::Component,
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::centered_rect_fixed,
    widgets::{
        button::Button,
        button_group::{ButtonGroup, ButtonGroupState},
    },
};

pub struct AuthHelper {
    config: Config,
    command_tx: Option<UnboundedSender<Action>>,
    ok_button: Button<'static>,
    /// Raw user input.
    input: Option<String>,
    /// Auth data prompt.
    prompt: Option<String>,
    /// Connection name for the credential data
    connection_name: Option<String>,
    /// Whether the input data is sensitive and should be masked.
    is_sensitive: bool,
    /// Channel through which the AuthHelper is supposed to send the oneshot sender channel to be
    /// used for providing the user provided data.
    auth_helper_control_channel_receiver: mpsc::Receiver<oneshot::Sender<AuthAction>>,
    /// Channel to send the user provided data. One time use.
    auth_helper_callback_channel: Option<oneshot::Sender<AuthAction>>,
}

impl AuthHelper {
    pub(crate) fn new(
        auth_helper_control_channel_receiver: mpsc::Receiver<oneshot::Sender<AuthAction>>,
    ) -> Self {
        Self {
            config: Config::default(),
            command_tx: None,
            ok_button: Button::new("OK"),
            input: None,
            prompt: None,
            connection_name: None,
            is_sensitive: true,
            auth_helper_control_channel_receiver,
            auth_helper_callback_channel: None,
        }
    }

    pub fn render_tick(&mut self) {}

    /// Inform the auth_helper about the cancellation.
    pub fn send_cancel(&mut self) -> Result<(), TuiError> {
        if let Some(command_tx) = self.auth_helper_callback_channel.take() {
            command_tx
                .send(AuthAction::Cancel)
                .map_err(|_| eyre!("error sending authentication cancellation to the helper"))?;
        }
        Ok(())
    }
    /// Inform the main app about auth helper completion.
    pub fn send_complete(&mut self) -> Result<(), TuiError> {
        if let Some(command_tx) = &self.command_tx {
            command_tx.send(Action::AuthHelperCompleted)?;
        }
        Ok(())
    }

    /// Reset the previously entered data.
    pub fn reset_data(&mut self) -> Result<(), TuiError> {
        self.input = None;
        self.prompt = None;
        self.connection_name = None;
        self.is_sensitive = true;
        Ok(())
    }
}

impl Component for AuthHelper {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    #[instrument(skip(self, key), fields(prompt=self.prompt))]
    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        match &key.code {
            KeyCode::Enter => {
                trace!("User confirmed the authentication data");
                if let Some(command_tx) = self.auth_helper_callback_channel.take() {
                    trace!("Sending data to the auth helper");
                    command_tx
                        .send(if self.is_sensitive {
                            AuthAction::Secret(SecretString::new(
                                self.input.clone().unwrap().into(),
                            ))
                        } else {
                            AuthAction::Data(self.input.clone().unwrap())
                        })
                        .map_err(|_| {
                            eyre!("error sending the requested data to the auth helper")
                        })?;
                }
                self.send_complete()?;
                self.reset_data()?;
            }
            KeyCode::Esc => {
                // Cancel
                trace!("User refused to provide the authentication data");
                self.send_cancel()?;
                self.send_complete()?;
                self.reset_data()?;
            }
            KeyCode::Backspace | KeyCode::Delete => {
                if let Some(ref mut input) = self.input {
                    input.pop();
                    if input.is_empty() {
                        self.input = None;
                    }
                };
            }
            KeyCode::Char(i) => {
                self.input.get_or_insert(String::new()).push(*i);
            }
            _ => {}
        }
        Ok(None)
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        if let Action::AuthDataRequired {
            prompt,
            connection_name,
            is_sensitive,
        } = action
        {
            self.prompt = Some(prompt.clone());
            self.connection_name = connection_name;
            self.is_sensitive = is_sensitive;
            if let Ok(channel) = self.auth_helper_control_channel_receiver.try_recv() {
                self.auth_helper_callback_channel = Some(channel);
            }
        };
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let ar = centered_rect_fixed(50, 10, frame.area());
        let mut title = if let Some(cloud) = &self.connection_name {
            format!("Connecting to {cloud}")
        } else {
            "Cloud connection".into()
        };
        let popup_block = Block::default()
            .title_top(Line::from(title).light_yellow().centered())
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg)
            .border_style(Style::default().fg(self.config.styles.popup_border_confirm_fg));

        let input_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.config.styles.fg));

        let prompt = Paragraph::new(format!(
            "Please provide the {}:",
            self.prompt.clone().unwrap_or_default()
        ));
        let input = Paragraph::new(
            self.input
                .clone()
                .map(|v| {
                    if self.is_sensitive {
                        mask_string_except_last(v)
                    } else {
                        v
                    }
                })
                .unwrap_or_default(),
        )
        .block(input_block);

        let text_buttons_layout = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Percentage(100),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(popup_block.inner(ar));

        frame.render_widget(Clear, ar);
        frame.render_widget(popup_block, ar);
        frame.render_widget(prompt, text_buttons_layout[0]);
        frame.render_widget(input, text_buttons_layout[1]);
        frame.render_widget(&self.ok_button, text_buttons_layout[3]);

        Ok(())
    }
}

/// Mask the string only keeping the last character visible
fn mask_string_except_last(s: String) -> String {
    let len = s.chars().count();
    if len == 0 {
        return String::new();
    }

    let mut masked_string = String::new();

    // Mask all characters except the last one
    for _ in s.chars().take(len - 1) {
        masked_string.push('*');
    }

    // Append the last character
    if let Some(last_char) = s.chars().last() {
        masked_string.push(last_char);
    }

    masked_string
}
