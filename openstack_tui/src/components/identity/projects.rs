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

use crossterm::event::KeyEvent;
use eyre::Result;
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::types::{
        ApiRequest, IdentityApiRequest, IdentityProjectApiRequest, IdentityProjectList,
    },
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Identity Projects";

#[derive(Deserialize, StructTable)]
pub struct ProjectData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "ID")]
    id: String,
    #[structable(title = "Parent ID")]
    parent_id: String,
    #[structable(title = "Enabled")]
    enabled: bool,
    #[structable(title = "Domain ID")]
    domain_id: String,
}

pub type IdentityProjects<'a> = TableViewComponentBase<'a, ProjectData, IdentityProjectList>;

impl Component for IdentityProjects<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.set_command_tx(tx)
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::CloudChangeScope(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                if let Mode::IdentityProjects = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        IdentityProjectApiRequest::List(self.get_filters().clone()),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::IdentityProjects,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    IdentityProjectApiRequest::List(self.get_filters().clone()),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request:
                    ApiRequest::Identity(IdentityApiRequest::Project(IdentityProjectApiRequest::List(
                        _,
                    ))),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::SwitchToProject => {
                if let Some(project) = self.get_selected() {
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
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.handle_key_events(key)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        self.draw(f, area, TITLE)
    }
}
