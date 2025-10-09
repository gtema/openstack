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

use eyre::{Result, WrapErr};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use std::cmp;
use tokio::sync::mpsc::UnboundedSender;

use crate::utils::centered_rect_fixed;
use crate::{
    action::Action,
    //cloud_worker::network::v2::{
    //    NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleDelete,
    //    NetworkSecurityGroupRuleDeleteBuilder, NetworkSecurityGroupRuleDeleteBuilderError,
    //    NetworkSecurityGroupRuleList,
    //},
    cloud_worker::types::ApiRequest,
    components::Component,
    config::Config,
    error::TuiError,
    mode::Mode,
    tui::Event,
    utils::ResourceKey,
};

const TITLE: &str = " Create security group rule: ";

#[derive(Default)]
pub struct CreateSecurityGroupRuleComponent {
    config: Config,
    port_range_min: String,
    security_group_id: String,

    max_row_length: u16,
    text: Vec<String>,
    cursor_pos: (u16, u16),

    content_scroll: (u16, u16),
    content_size: Size,
    vscroll_state: ScrollbarState,
    hscroll_state: ScrollbarState,
}

//impl StatefulWidget for CreateSecurityGroupRuleComponent {
//    type State = i32;
//
//    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
//        let table = Table::default()
//            //.header(vec![])
//            .rows(vec![Row::new(vec!["Port range min"])]);
//        //let greeting = format!("Hello, {}!", self.name);
//        //buf.set_string(area.x, area.y, greeting, Style::default());
//    }
//}

impl CreateSecurityGroupRuleComponent {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            port_range_min: String::new(),
            security_group_id: String::new(),
            text: vec![
                "first".into(),
                "second longer".into(),
                "trhird df sdffds".into(),
            ],
            cursor_pos: (0, 0),
            ..Default::default()
        }
    }

    //pub fn cursor_up(&mut self) -> Result<()> {
    //    self.cursor_pos.0 = self.cursor_pos.0.saturating_sub(1);
    //    if self.text.len() as u16 > self.content_size.height {
    //        self.content_scroll.0 = self.content_scroll.0.saturating_sub(1);
    //        self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
    //    }
    //    Ok(())
    //}

    //pub fn cursor_down(&mut self) -> Result<()> {
    //    self.cursor_pos.0 = self.cursor_pos.0.saturating_add(1);
    //    if self.text.len() as u16 > self.content_size.height {
    //        self.content_scroll.0 = cmp::min(
    //            self.content_scroll.0.saturating_add(1),
    //            (self.text.len() as u16).saturating_sub(self.content_size.height),
    //        );
    //        self.vscroll_state = self.vscroll_state.position(self.content_scroll.0.into());
    //    }
    //    Ok(())
    //}

    //pub fn cursor_right(&mut self) -> Result<()> {
    //    self.cursor_pos.1 = self.cursor_pos.1.saturating_add(1);
    //    if self.max_row_length > self.content_size.width {
    //        self.content_scroll.1 = cmp::min(
    //            self.content_scroll.1.saturating_add(1),
    //            self.max_row_length.saturating_sub(self.content_size.width),
    //        );
    //        self.hscroll_state = self.hscroll_state.position(self.content_scroll.1.into());
    //    }
    //    Ok(())
    //}

    //pub fn cursor_left(&mut self) -> Result<()> {
    //    self.cursor_pos.1 = self.cursor_pos.1.saturating_sub(1);
    //    if self.max_row_length > self.content_size.height {
    //        self.content_scroll.1 = self.content_scroll.1.saturating_sub(1);
    //        self.hscroll_state = self.hscroll_state.position(self.content_scroll.1.into());
    //    }
    //    Ok(())
    //}
}

impl Component for CreateSecurityGroupRuleComponent {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    //fn handle_events(&mut self, event: Option<Event>) -> Result<Option<Action>, TuiError> {
    //    self.editor_event_handler
    //        .on_key_event(event, &mut self.editor_state);
    //    Ok(None)
    //}

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        //self.handle_key_events(key)

        //self.editor_event_handler
        //    .on_key_event(key, &mut self.editor_state);
        //if key.kind == KeyEventKind::Press {
        //    match key.code {
        //        KeyCode::Char('j') | KeyCode::Down => self.cursor_down()?,
        //        KeyCode::Char('k') | KeyCode::Up => self.cursor_up()?,
        //        //KeyCode::Home => self.cursor_first()?,
        //        //KeyCode::End => self.cursor_last()?,
        //        //KeyCode::PageUp => self.cursor_page_up()?,
        //        //KeyCode::PageDown => self.cursor_page_down()?,
        //        KeyCode::Left => self.cursor_left()?,
        //        KeyCode::Right => self.cursor_right()?,
        //        // KeyCode::Char('w') => {
        //        //     self.wrap = !self.wrap;
        //        // }
        //        _ => {}
        //    }
        //}
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        let area = centered_rect_fixed(50, 35, frame.area());
        let mut title = vec![TITLE.white()];

        let popup_block = Block::default()
            .title_top(Line::from(title).centered())
            .title_bottom(
                Line::from(" (↑) move up | (↓) move down | (Enter) to select | (Esc) to close ")
                    .gray()
                    .right_aligned(),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .bg(self.config.styles.popup_bg)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.popup_border_fg));
        let inner = popup_block.inner(area);
        //let block = Block::default()
        //    .borders(Borders::RIGHT)
        //    .padding(Padding::right(1))
        //    .border_style(Style::default().fg(self.config.styles.border_fg));
        ////.inner(inner);
        //bl//ock.inner(inner);

        //let text: Vec<Line> = self.text.clone().into_iter().map(Line::from).collect();
        //let paragraph = Paragraph::new(text)
        //    .block(popup_block);
        //    //.scroll((self.content_scroll.0, self.content_scroll.1));

        //////self.content_size = block.inner(area).as_size();
        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        //frame.render_widget(paragraph, area);

        ////self.draw(frame, area)

        // EditorView::new(&mut self.editor_state)
        //     .theme(EditorTheme::default())
        //     .wrap(true) // line wrapping
        //     .render(inner, frame.buffer_mut());
        Ok(())
    }
}
