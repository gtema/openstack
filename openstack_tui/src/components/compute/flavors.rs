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
use eyre::{Result, WrapErr};
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::compute::v2::{ComputeFlavorList, ComputeServerListBuilder},
    cloud_worker::types::*,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{as_string, OutputConfig, ResourceKey, StructTable},
};

const TITLE: &str = "Compute Flavors";
const VIEW_CONFIG_KEY: &str = "compute.flavor";

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

impl ResourceKey for FlavorData {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type ComputeFlavors<'a> = TableViewComponentBase<'a, FlavorData, ComputeFlavorList>;

impl ComputeFlavors<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(&self, mut filters: ComputeFlavorList) -> ComputeFlavorList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some("name".into());
            filters.sort_dir = Some("asc".into());
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> ComputeFlavorList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

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
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        ComputeFlavorApiRequest::ListDetailed(Box::new(self.normalized_filters())),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::ComputeFlavors,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeFlavorApiRequest::ListDetailed(Box::new(self.normalized_filters())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Compute(ComputeApiRequest::Flavor(req)),
                data,
            } => {
                if let ComputeFlavorApiRequest::ListDetailed(_) = *req {
                    self.set_data(data)?;
                }
            }
            Action::ShowComputeServersWithFlavor => {
                // only if we are currently in the flavors mode
                if current_mode == Mode::ComputeFlavors {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set SecurityGroupRulesList
                            command_tx.send(Action::SetComputeServerListFilters(Box::new(
                                ComputeServerListBuilder::default()
                                    .flavor(selected_entry.id.clone())
                                    .build()
                                    .wrap_err("cannot prepare filters")?,
                            )))?;
                            // and switch mode
                            command_tx.send(Action::Mode {
                                mode: Mode::ComputeServers,
                                stack: true,
                            })?;
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
