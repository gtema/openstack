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
use eyre::{Result, WrapErr};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use serde::Deserialize;
use serde_json::Value;
use structable_derive::StructTable;

use crate::{
    action::Action,
    cloud_worker::identity::v3::{
        IdentityApiRequest, IdentityAuthApiRequest, IdentityAuthProjectApiRequest,
        IdentityAuthProjectListBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, FuzzySelectList},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable, centered_rect_fixed},
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
    items: Vec<ProjectData>,
    is_loading: bool,
    fuzzy_list: FuzzySelectList,
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
            items: Vec::new(),
            is_loading: true,
            fuzzy_list: FuzzySelectList::new(),
        }
    }

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    fn set_data(&mut self, data: Vec<Value>) -> Result<(), TuiError> {
        let mut items: Vec<ProjectData> =
            serde_json::from_value(serde_json::Value::Array(data.clone()))?;
        items.sort_by_key(|x| x.name.clone());

        self.items = items;
        self.fuzzy_list
            .set_items(self.items.iter().map(|x| x.name.clone()));
        self.set_loading(false);
        Ok(())
    }
}

impl Component for ProjectSelect {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::ConnectToCloud(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    IdentityAuthProjectApiRequest::List(Box::new(
                        IdentityAuthProjectListBuilder::default()
                            .build()
                            .wrap_err("cannot prepare auth project list request")?,
                    )),
                ))));
            }
            Action::ApiResponsesData {
                request: ApiRequest::Identity(IdentityApiRequest::Auth(req)),
                data,
            } => {
                if let IdentityAuthApiRequest::Project(_) = *req {
                    self.set_data(data)?;
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.fuzzy_list.handle_key_events(key)?;
        if key.code == KeyCode::Enter {
            if let Some(selected) = self.fuzzy_list.selected() {
                if let Some(project) = self.items.iter().find(|item| item.name == *selected) {
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
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _area: Rect) -> Result<(), TuiError> {
        let area = centered_rect_fixed(50, 35, frame.area());
        let mut title = vec![TITLE.white()];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        }

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

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);

        self.fuzzy_list.draw(frame, inner)?;
        Ok(())
    }
}
