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
    cloud_worker::types::{ApiRequest, ComputeFlavorFilters, ComputeServerFilters},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{as_string, OutputConfig, StructTable},
};

const TITLE: &str = "Compute Flavors";

#[derive(Deserialize, StructTable)]
pub struct FlavorData {
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "vCPU")]
    #[serde(deserialize_with = "as_string")]
    vcpus: String,
    #[serde(deserialize_with = "as_string")]
    ram: String,
    #[serde(deserialize_with = "as_string")]
    disk: String,
    #[serde(rename = "OS-FLV-DISABLED:disabled", deserialize_with = "as_string")]
    disabled: String,
}

pub type ComputeFlavors<'a> = TableViewComponentBase<'a, FlavorData, ComputeFlavorFilters>;

impl Component for ComputeFlavors<'_> {
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
                if let Mode::ComputeFlavors = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::ComputeFlavors(
                        self.get_filters().clone(),
                    ))));
                }
            }
            Action::Mode(Mode::ComputeFlavors) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::ComputeFlavors(
                    self.get_filters().clone(),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::ComputeFlavors(_),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::ShowComputeServersWithFlavor => {
                // only if we are currently in the flavors mode
                if current_mode == Mode::ComputeFlavors {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set SecurityGroupRulesFilters
                            command_tx.send(Action::SetComputeServerFilters(
                                ComputeServerFilters {
                                    all_tenants: None,
                                    flavor_id: Some(selected_entry.id.clone()),
                                    flavor_name: Some(selected_entry.name.clone()),
                                },
                            ))?;
                            // and switch mode
                            command_tx.send(Action::Mode(Mode::ComputeServers))?;
                        }
                    }
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
