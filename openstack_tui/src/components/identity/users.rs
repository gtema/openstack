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
    cloud_worker::types::{IdentityUserFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{as_string, OutputConfig, StructTable},
};

const TITLE: &str = "Identity Users";

#[derive(Deserialize, StructTable)]
pub struct UserData {
    #[structable(title = "Name")]
    name: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Email")]
    email: String,
    #[structable(title = "Domain")]
    domain_id: String,
    #[structable(title = "Enabled")]
    enabled: bool,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Pwd expiry")]
    password_expires_at: String,
}

pub type IdentityUsers<'a> = TableViewComponentBase<'a, UserData, IdentityUserFilters>;

impl Component for IdentityUsers<'_> {
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
                if let Mode::IdentityUsers = current_mode {
                    return Ok(Some(Action::RequestCloudResource(Resource::IdentityUsers(
                        self.get_filters().clone(),
                    ))));
                }
            }
            Action::Mode(Mode::IdentityUsers) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(Resource::IdentityUsers(
                    self.get_filters().clone(),
                ))));
            }
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::IdentityUsers(_),
                data,
            } => {
                self.set_data(data)?;
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
