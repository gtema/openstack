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
use ratatui::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use structable::{StructTable, StructTableOptions};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::identity::v3::{
        IdentityApiRequest, IdentityAuthProjectApiRequest, IdentityAuthProjectListBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, FuzzySelect, FuzzySelectState},
    config::Config,
    error::TuiError,
    mode::Mode,
};

const TITLE: &str = " Select project to switch to: ";

#[derive(Debug, Deserialize, StructTable)]
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
    popup_state: FuzzySelectState,
    is_loading: bool,
    action_tx: Option<UnboundedSender<Action>>,
    items_fetched: bool,
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
            popup_state: FuzzySelectState::new(),
            is_loading: true,
            action_tx: None,
            items_fetched: false,
        }
    }
}

impl Component for ProjectSelect {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config.clone();
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match &action {
            Action::ConnectToCloud(_) | Action::CloudChangeScope(_) => {
                self.is_loading = true;
                self.items_fetched = false;
                self.items.clear();
                self.popup_state.set_items(Vec::<String>::new());
            }
            Action::ConnectedToCloud(_) => {
                // Mark items_fetched=true so the generic is_loading guard at
                // the bottom doesn't re-issue. Issue the fetch directly so we
                // refresh after cloud change.
                self.items_fetched = true;
                self.items.clear();
                self.popup_state.set_items(Vec::<String>::new());
                self.is_loading = true;
                if let Some(tx) = &self.action_tx {
                    let req = IdentityAuthProjectListBuilder::default()
                        .build()
                        .wrap_err("cannot prepare auth project list request")?;
                    tx.send(Action::PerformApiRequest(ApiRequest::from(
                        IdentityAuthProjectApiRequest::List(Box::new(req)),
                    )))?;
                }
                self.is_loading = false;
            }

            Action::SelectProject => {
                // Only trigger a fetch when there's no data, nothing in-flight,
                // and no refresh already initiated by ConnectedToCloud.
                if !self.items_fetched && self.items.is_empty() && !self.is_loading {
                    self.is_loading = true;
                    self.items_fetched = false;
                    self.items.clear();
                    self.popup_state.set_items(Vec::<String>::new());
                }
            }
            Action::ApiResponsesData {
                data,
                request: ApiRequest::Identity(IdentityApiRequest::Auth(_req)),
            } => self.on_data(data.clone())?,
            _ => {}
        }

        if self.is_loading && self.action_tx.is_some() && !self.items_fetched {
            let req = IdentityAuthProjectListBuilder::default()
                .build()
                .wrap_err("cannot prepare auth project list request")?;
            self.is_loading = false;
            return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                IdentityAuthProjectApiRequest::List(Box::new(req)),
            ))));
        }

        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.popup_state.handle_key(&key.code);
        if key.code == KeyCode::Enter
            && let Some(selected_name) = self.popup_state.selected()
            && let Some(project) = self.items.iter().find(|p| &p.name == selected_name)
        {
            let new_project = openstack_sdk::types::identity::v3::Project {
                id: Some(project.id.clone()),
                name: Some(project.name.clone()),
                domain: Some(openstack_sdk::types::identity::v3::Domain {
                    id: Some(project.domain_id.clone()),
                    name: None,
                }),
            };
            let new_scope = openstack_sdk::auth::authtoken::AuthTokenScope::Project(new_project);
            return Ok(Some(Action::CloudChangeScope(Box::new(new_scope))));
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        frame.render_stateful_widget(
            FuzzySelect::new(&self.config)
                .as_popup(true)
                .title(TITLE)
                .loading(self.is_loading),
            area,
            &mut self.popup_state,
        );
        Ok(())
    }
}

impl ProjectSelect {
    fn on_data(&mut self, data: Vec<Value>) -> Result<(), TuiError> {
        let mut items: Vec<ProjectData> = serde_json::from_value(serde_json::Value::Array(data))?;
        items.sort_by_key(|x| x.name.to_string().to_lowercase());
        let names: Vec<String> = items.iter().map(|p| p.name.clone()).collect();
        self.items = items;
        self.popup_state.set_items(names);
        self.items_fetched = true;
        self.is_loading = false;
        Ok(())
    }
}
