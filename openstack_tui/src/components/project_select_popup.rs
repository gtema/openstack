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
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use serde::Deserialize;
use serde_json::Value;
use std::cmp;
use structable_derive::StructTable;

use crate::{
    action::Action,
    cloud_worker::types::{IdentityAuthProjectFilters, Resource},
    components::Component,
    config::Config,
    mode::Mode,
    utils::{centered_rect, OutputConfig, StructTable},
};

const TITLE: &str = " Select project to switch to: ";

#[derive(Deserialize, StructTable)]
pub struct ProjectData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "ID")]
    id: String,
    #[structable(title = "Domain ID")]
    domain_id: String,
    #[structable(title = "Enabled")]
    enabled: bool,
}

pub struct ProjectSelect {
    config: Config,
    content_size: Size,
    items: Vec<ProjectData>,
    state: ListState,
    scroll_state: ScrollbarState,
    user_input: Option<String>,
    is_loading: bool,
}

impl Default for ProjectSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectSelect {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            content_size: Size::new(0, 0),
            items: Vec::new(),
            state: ListState::default(),
            scroll_state: ScrollbarState::new(0),
            user_input: None,
            is_loading: true,
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    fn set_data(&mut self, data: Vec<Value>) -> Result<()> {
        let mut items: Vec<ProjectData> =
            serde_json::from_value(serde_json::Value::Array(data.clone()))?;
        items.sort_by_key(|x| x.name.clone());

        self.items = items;
        self.state.select_first();
        self.scroll_state = ScrollbarState::new(self.items.len().saturating_sub(1));
        self.set_loading(false);
        Ok(())
    }

    pub fn cursor_first(&mut self) -> Result<()> {
        self.state.select_first();
        self.scroll_state.first();
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_last(&mut self) -> Result<()> {
        self.state.select_last();
        self.scroll_state.last();
        self.user_input = None;
        Ok(())
    }

    fn cursor_up(&mut self) -> Result<()> {
        self.state.select_previous();
        self.scroll_state.prev();
        self.user_input = None;
        Ok(())
    }

    fn cursor_down(&mut self) -> Result<()> {
        self.state.select_next();
        self.scroll_state.next();
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_page_down(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => cmp::min(
                i.saturating_add(self.content_size.height as usize),
                self.items.len(),
            ),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.user_input = None;
        Ok(())
    }

    pub fn cursor_page_up(&mut self) -> Result<()> {
        let i = match self.state.selected() {
            Some(i) => i.saturating_sub(self.content_size.height as usize),
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i);
        self.user_input = None;
        Ok(())
    }

    pub fn select_by_filter(&mut self) -> Result<()> {
        if let Some(input) = &self.user_input {
            if !input.is_empty() {
                let mut found = false;
                for (idx, item) in self.items.iter().enumerate() {
                    if item.name.starts_with(input) {
                        self.state.select(Some(idx));
                        self.scroll_state = self.scroll_state.position(idx);
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.user_input = None;
                }
            }
        }
        Ok(())
    }
}

impl Component for ProjectSelect {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }
    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::ConnectToCloud(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::IdentityAuthProjects(IdentityAuthProjectFilters {}),
                )));
            }
            Action::ResourcesData {
                resource: Resource::IdentityAuthProjects(_),
                data,
            } => {
                self.set_data(data)?;
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Enter => {
                if let Some(pos) = self.state.selected() {
                    if let Some(project) = self.items.get(pos) {
                        let new_project = openstack_sdk::types::identity::v3::Project {
                            id: Some(project.id.clone()),
                            name: Some(project.name.clone()),
                            domain: Some(openstack_sdk::types::identity::v3::Domain {
                                id: Some(project.domain_id.clone()),
                                name: None,
                            }),
                        };
                        let new_scope =
                            openstack_sdk::auth::authtoken::AuthTokenScope::Project(new_project);
                        return Ok(Some(Action::CloudChangeScope(new_scope)));
                    }
                }
            }
            KeyCode::Backspace => {
                if let Some(ref mut input) = self.user_input {
                    input.pop();
                    if input.is_empty() {
                        self.user_input = None;
                    }
                };
                self.select_by_filter()?;
            }
            KeyCode::Char(i) => {
                self.user_input.get_or_insert(String::new()).push(i);
                self.select_by_filter()?;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<()> {
        let area = centered_rect(25, 25, frame.area());
        let mut title = vec![TITLE.white()];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        }

        if let Some(input) = &self.user_input {
            title.push(Span::styled(
                format!("(prefix: {})", input),
                self.config.styles.popup_title_fg,
            ));
        }
        let popup_block = Block::default()
            .title_top(Line::from(title).centered())
            .title_bottom(
                Line::from(" (↑) move up | (↓) move down | (Enter) to select | (Esc) to close ")
                    .gray()
                    .right_aligned(), //.alignment(Alignment::Right),
            )
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .bg(self.config.styles.popup_bg)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.popup_border_fg));
        let inner = popup_block.inner(area);
        self.content_size = inner.as_size();
        frame.render_widget(Clear, area);

        let mut rows: Vec<ListItem> = Vec::new();
        for item in &self.items {
            if let Some(input) = &self.user_input {
                if item.name.starts_with(input) {
                    rows.push(ListItem::from(Line::from(vec![
                        Span::styled(input.clone(), self.config.styles.item_highlight_fg),
                        Span::raw(
                            item.name
                                .strip_prefix(input)
                                .expect("Project name contains user_input prefix"),
                        ),
                    ])));
                } else {
                    rows.push(ListItem::new(
                        item.name.clone().fg(self.config.styles.popup_item_title_fg),
                    ));
                }
            } else {
                rows.push(ListItem::new(
                    item.name.clone().fg(self.config.styles.popup_item_title_fg),
                ));
            }
        }
        let list = List::default()
            .items(rows)
            .block(popup_block)
            .style(self.config.styles.popup_item_title_fg)
            .highlight_style(Style::new().bg(self.config.styles.item_selected_bg));

        frame.render_stateful_widget(list, area, &mut self.state);

        if usize::from(self.content_size.height) < self.items.len() {
            frame.render_stateful_widget(
                Scrollbar::default()
                    .orientation(ScrollbarOrientation::VerticalRight)
                    .style(Style::default().fg(self.config.styles.popup_border_fg)),
                area.inner(Margin {
                    vertical: 1,
                    horizontal: 1,
                }),
                &mut self.scroll_state,
            );
        }

        Ok(())
    }
}
