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

use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use ratatui::{layout::Rect, prelude::*, widgets::*};
use std::io::Write;

use crate::{
    action::Action, components::Component, config::Config, error::TuiError, mode::Mode,
    utils::centered_rect_fixed,
};

const BOTTOM_TITLE: &str = "(Esc) to close";
const BOTTOM_TITLE_WITH_REPORT: &str = "(r) to report, (Esc) to close";

pub struct ErrorPopup {
    config: Config,
    text: Vec<String>,
    msg: Option<String>,
    source: Option<Box<Action>>,
    scroll: (u16, u16),
}

impl Default for ErrorPopup {
    fn default() -> Self {
        Self::new()
    }
}

impl ErrorPopup {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            text: Vec::new(),
            msg: None,
            source: None,
            scroll: (0, 0),
        }
    }

    pub fn scroll_right(&mut self) {
        self.scroll.0 = self.scroll.0.saturating_add(1);
    }
    pub fn scroll_left(&mut self) {
        self.scroll.0 = self.scroll.0.saturating_sub(1);
    }
    pub fn scroll_down(&mut self) {
        self.scroll.1 = self.scroll.1.saturating_add(1);
    }
    pub fn scroll_up(&mut self) {
        self.scroll.1 = self.scroll.1.saturating_sub(1);
    }

    /// Report the error information
    ///
    /// Open browser at the issue tracker with basic information and please to upload prepared
    /// crash information.
    pub fn report(&mut self) -> Result<(), TuiError> {
        if let Some(source) = &self.source {
            let mut url = url::Url::parse(concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new"))?;
            url.query_pairs_mut()
                .append_pair("title", &self.msg.clone().unwrap_or("Error in TUI".into()));
            url.query_pairs_mut().append_pair("labels", "bug,ostui");
            let mut body: Vec<String> = vec![format!(
                "# Error\n```{}```\n",
                self.msg.clone().unwrap_or("Unknown error".into())
            )];
            if let Some(dir) = crate::utils::get_data_dir() {
                let crash_data_path = dir.join(format!(
                    "crash.{}.txt",
                    chrono::Local::now().format("%Y-%m-%dT%H%M%S")
                ));
                let mut crash_data_file = std::fs::File::create(&crash_data_path)?;
                crash_data_file.write_all(b"[error]\n")?;
                crash_data_file.write_all(
                    self.msg
                        .clone()
                        .unwrap_or("Unknown error".into())
                        .as_bytes(),
                )?;
                crash_data_file.write_all(b"\n\n")?;
                crash_data_file.write_all(b"[source]\n")?;
                crash_data_file.write_all(
                    serde_json::to_string_pretty(&source)
                        .unwrap_or_default()
                        .as_bytes(),
                )?;
                crash_data_file.write_all(b"\n\n")?;
                crash_data_file.write_all(b"[version]\n")?;
                crash_data_file.write_all(env!("CARGO_PKG_VERSION").as_bytes())?;

                match &**source {
                    Action::ApiResponsesData { request, .. }
                    | Action::ApiResponseData { request, .. } => {
                        body.push("## Request\n\n".into());
                        body.push(format!("```\n{request}\n```"));
                    }
                    _ => {}
                }
                body.push(format!(
                "<!-- Please review sensitive data in the crash information file prepared in `{}` and upload it - it helps to locate problem faster -->",
                crash_data_path.as_os_str().to_string_lossy()
            ));
            }
            url.query_pairs_mut().append_pair("body", &body.join("\n"));
            open::that(url.as_str())?;
        }
        Ok(())
    }
}

impl Component for ErrorPopup {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        if let Action::Error {
            ref msg,
            ref action,
        } = action
        {
            self.msg = Some(strip_ansi_escapes::strip_str(msg));
            self.source = action.clone();

            self.text = strip_ansi_escapes::strip_str(msg)
                .split("\n")
                .map(String::from)
                .collect::<Vec<_>>();

            if self.source.is_some() {
                self.text.push(String::new());
                self.text.push(String::from(
                    "Please consider reporting the issue (press `r`).",
                ));
            }
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        match key.code {
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Right => self.scroll_right(),
            KeyCode::Left => self.scroll_left(),
            KeyCode::Char('r') => self.report()?,
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let ar = centered_rect_fixed(120, 20, frame.area());
        let popup_block = Block::default()
            .title_top(Line::from(" Error ").red().centered())
            .title_bottom(
                Line::from(if self.source.is_some() {
                    BOTTOM_TITLE_WITH_REPORT
                } else {
                    BOTTOM_TITLE
                })
                .gray()
                .right_aligned(),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .padding(Padding::uniform(1))
            .bg(self.config.styles.popup_bg)
            .border_style(Style::default().fg(self.config.styles.popup_border_error_fg));
        let text: Vec<Line> = self.text.clone().into_iter().map(Line::from).collect();
        let paragraph = Paragraph::new(text)
            .block(popup_block)
            .scroll((self.scroll.1, self.scroll.0));

        frame.render_widget(Clear, ar);
        frame.render_widget(paragraph, ar);

        Ok(())
    }
}
