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

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::types::{IdentityProjectFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
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

pub type IdentityProjects<'a> = TableViewComponentBase<'a, ProjectData, IdentityProjectFilters>;

impl<'a> Component for IdentityProjects<'a> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)?;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.set_command_tx(tx);
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::CloudChangeScope(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                if let Mode::IdentityProjects = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::IdentityProjects(self.get_filters().clone()),
                    )));
                }
            }
            Action::Mode(Mode::IdentityProjects) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::IdentityProjects(self.get_filters().clone()),
                )));
            }
            Action::Tick => {
                self.app_tick()?;
            }
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::IdentityProjects(_),
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

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        match key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Left => self.cursor_left()?,
            KeyCode::Right => self.cursor_right()?,
            KeyCode::Tab => self.key_tab()?,
            _ => {}
        }
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
            if let Some(x) = self.get_selected_raw() {
                return Ok(Some(Action::Describe(x.clone())));
            }
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let areas = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(area);

        self.render_content(TITLE, f, areas[0])?;
        self.render_footer(f, areas[1]);
        Ok(())
    }
}
