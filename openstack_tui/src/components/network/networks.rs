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
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::types::{NetworkNetworkFilters, NetworkSubnetFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Networks";

#[derive(Deserialize, StructTable)]
pub struct NetworkData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    status: String,
    #[structable(title = "Created")]
    #[serde(rename = "created_at")]
    created: String,
    #[structable(title = "Updated")]
    #[serde(rename = "updated_at")]
    updated: String,
}

pub type NetworkNetworks<'a> = TableViewComponentBase<'a, NetworkData, NetworkNetworkFilters>;

impl<'a> Component for NetworkNetworks<'a> {
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
                if let Mode::NetworkNetworks = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::NetworkNetworks(self.get_filters().clone()),
                    )));
                }
            }
            Action::Mode(Mode::NetworkNetworks) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::NetworkNetworks(self.get_filters().clone()),
                )));
            }
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::NetworkNetworks(_),
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
            KeyCode::Enter => {
                if let Some(command_tx) = self.get_command_tx() {
                    if let Some(x) = self.get_selected_raw() {
                        command_tx.send(Action::NetworkSubnetFilter(NetworkSubnetFilters {
                            network_id: x.get("id").unwrap().as_str().map(String::from).clone(),
                        }))?;
                    }
                }
                return Ok(Some(Action::Mode(Mode::NetworkSubnets)));
            }
            _ => {}
        }
        self.handle_key_events(key)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        self.draw(f, area, TITLE)
    }
}
