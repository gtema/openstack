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
    cloud_worker::types::{IdentityApplicationCredentialFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{as_string, OutputConfig, StructTable},
};

const TITLE: &str = "Application Credentials";

#[derive(Deserialize, StructTable)]
pub struct ApplicationCredentialData {
    /// ApplicationCredential id (used for related operations)
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name")]
    name: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Expires as")]
    expires_at: String,
    #[structable(title = "Unrestricted")]
    unrestricted: bool,
}

pub type IdentityApplicationCredentials<'a> =
    TableViewComponentBase<'a, ApplicationCredentialData, IdentityApplicationCredentialFilters>;

impl Component for IdentityApplicationCredentials<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.set_command_tx(tx);
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::CloudChangeScope(_) | Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                // Unset the filters since in new cloud everything is different
                if let Some(command_tx) = self.get_command_tx() {
                    command_tx.send(Action::IdentityApplicationCredentialFilter(
                        IdentityApplicationCredentialFilters {
                            user_id: String::new(),
                            user_name: None,
                        },
                    ))?;
                }
            }
            Action::Mode(Mode::IdentityApplicationCredentials) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::IdentityApplicationCredentials(self.get_filters().clone()),
                )));
            }
            Action::IdentityApplicationCredentialFilter(filters) => {
                self.set_filters(filters);
                self.set_data(Vec::new())?;
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::IdentityApplicationCredentials(self.get_filters().clone()),
                )));
            }
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::IdentityApplicationCredentials(_),
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