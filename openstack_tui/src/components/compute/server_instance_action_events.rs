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
use structable::{StructTable, StructTableOptions};
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    cloud_worker::compute::v2::{
        ComputeApiRequest, ComputeServerApiRequest, ComputeServerInstanceActionApiRequest,
        ComputeServerInstanceActionShow,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "InstanceAction Events";
const VIEW_CONFIG_KEY: &str = "compute.server/instance_action/event";

/// Event type
#[derive(Clone, Debug, Deserialize, StructTable)]
pub struct ServerInstanceActionEventData {
    /// Even details
    #[structable(optional)]
    pub details: Option<String>,

    /// Event summary
    pub event: String,

    /// Finish time of the event
    #[structable(optional)]
    pub finish_time: Option<String>,

    /// Hostname
    #[structable(optional)]
    pub host: Option<String>,

    /// Host ID
    #[structable(optional)]
    pub host_id: Option<String>,

    /// Result
    #[structable(optional)]
    pub result: Option<String>,

    /// Event start time
    #[structable(optional)]
    pub start_time: Option<String>,

    /// Traceback
    #[structable(optional)]
    pub traceback: Option<String>,
}

impl ResourceKey for ServerInstanceActionEventData {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type ComputeServerInstanceActionEvents<'a> =
    TableViewComponentBase<'a, ServerInstanceActionEventData, ComputeServerInstanceActionShow>;

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
            Action::Mode {
                mode: Mode::ComputeServerInstanceActionEvents,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerInstanceActionApiRequest::Get(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponseData {
                request: ApiRequest::Compute(ComputeApiRequest::Server(req)),
                data,
            } => {
                if let ComputeServerApiRequest::InstanceAction(x) = *req {
                    if let ComputeServerInstanceActionApiRequest::Get(_) = *x {
                        if let Some(events) = data.get("events") {
                            if let Some(ar) = events.as_array() {
                                self.set_data(ar.to_vec())?;
                            }
                        }
                    }
                }
            }
            Action::SetComputeServerInstanceActionShowFilters(req) => {
                self.set_filters(req);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerInstanceActionApiRequest::Get(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
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
