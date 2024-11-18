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
    cloud_worker::types::{ComputeServerFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Compute Servers";

#[derive(Deserialize, StructTable)]
pub struct ServerData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    status: String,
    #[structable(title = "Created")]
    created: String,
    #[structable(title = "Updated")]
    updated: String,
}

pub type ComputeServers<'a> = TableViewComponentBase<'a, ServerData, ComputeServerFilters>;

impl Component for ComputeServers<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)
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
                if let Mode::ComputeServers = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServers(self.get_filters().clone()),
                    )));
                }
            }
            Action::Mode(Mode::ComputeServers) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeServers(self.get_filters().clone()),
                )));
            }
            Action::Tick => {
                self.app_tick()?;
                if let Mode::ComputeServers = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServers(self.get_filters().clone()),
                    )));
                }
            }
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::ComputeServers(_),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::ResourceData {
                resource: Resource::ComputeServerConsoleOutput(id),
                data,
            } => {
                if let Some(server_id) = self.get_selected_resource_id()? {
                    if server_id == id {
                        self.set_loading(false);
                        return Ok(Some(Action::Describe(data)));
                    }
                }
            }
            Action::SetComputeServerFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeServers(self.get_filters().clone()),
                )));
            }
            Action::ShowServerConsoleOutput => {
                if let Some(server_id) = self.get_selected_resource_id()? {
                    self.set_loading(true);
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServerConsoleOutput(server_id),
                    )));
                }
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.handle_key_events(key)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        self.draw(f, area, TITLE)
    }
}
