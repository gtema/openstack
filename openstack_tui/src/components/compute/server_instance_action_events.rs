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
    cloud_worker::types::{ComputeServerInstanceActionFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "InstanceAction Events";

#[derive(Deserialize, StructTable)]
pub struct ServerInstanceActionEventData {
    #[structable(title = "Event")]
    event: String,
    #[structable(title = "Result")]
    result: String,
    #[structable(title = "Started", optional)]
    start_time: Option<String>,
    #[structable(title = "Finished", optional)]
    finish_time: Option<String>,
    #[structable(title = "Host", optional)]
    host: Option<String>,
}

pub type ComputeServerInstanceActionEvents<'a> =
    TableViewComponentBase<'a, ServerInstanceActionEventData, ComputeServerInstanceActionFilters>;

impl Component for ComputeServerInstanceActionEvents<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.set_command_tx(tx)
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::CloudChangeScope(_) => {
                self.set_loading(false);
                self.set_data(Vec::new())?;
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(false);
                self.set_data(Vec::new())?;
            }
            Action::Mode(Mode::ComputeServerInstanceActionEvents) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeServerInstanceAction(self.get_filters().clone()),
                )));
            }
            Action::DescribeResource => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ResourceData {
                resource: Resource::ComputeServerInstanceAction(_),
                data,
            } => {
                if let Some(events) = data.get("events") {
                    if let Some(ar) = events.as_array() {
                        self.set_data(ar.to_vec())?;
                    }
                }
            }
            Action::SetComputeServerInstanceActionFilters(filters) => {
                if filters.request_id.is_some() {
                    self.set_filters(filters);
                    self.set_loading(true);
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServerInstanceAction(self.get_filters().clone()),
                    )));
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
