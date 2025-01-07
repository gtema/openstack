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
    cloud_worker::compute::v2::{
        ComputeApiRequest, ComputeServerApiRequest, ComputeServerInstanceActionApiRequest,
        ComputeServerInstanceActionList, ComputeServerInstanceActionShowBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, ResourceKey, StructTable},
};

const TITLE: &str = "ServerInstanceAction Actions";
const VIEW_CONFIG_KEY: &str = "compute.server/instance_action";

#[derive(Deserialize, StructTable)]
pub struct ServerInstanceActionData {
    #[structable(title = "Id", wide)]
    #[serde(rename = "request_id")]
    id: String,
    #[structable(title = "Action")]
    action: String,
    #[structable(title = "Message", optional)]
    message: Option<String>,
    #[structable(title = "Started")]
    start_time: String,
    #[structable(title = "User")]
    user_id: String,
    #[structable(title = "Server ID", wide)]
    instance_uuid: String,
}

impl ResourceKey for ServerInstanceActionData {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type ComputeServerInstanceActions<'a> =
    TableViewComponentBase<'a, ServerInstanceActionData, ComputeServerInstanceActionList>;

impl Component for ComputeServerInstanceActions<'_> {
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
                if let Mode::ComputeServerInstanceActions = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        ComputeServerInstanceActionApiRequest::List(Box::new(
                            self.get_filters().clone(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::ComputeServerInstanceActions,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerInstanceActionApiRequest::List(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Compute(ComputeApiRequest::Server(req)),
                data,
            } => {
                if let ComputeServerApiRequest::InstanceAction(x) = *req {
                    if let ComputeServerInstanceActionApiRequest::List(_) = *x {
                        self.set_data(data)?;
                    }
                }
            }
            Action::SetComputeServerInstanceActionListFilters(filters) => {
                self.set_filters(*filters);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerInstanceActionApiRequest::List(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
            }
            Action::ShowComputeServerInstanceActionEvents => {
                // only if we are currently in the IdentityGroup mode
                if current_mode == Mode::ComputeServerInstanceActions {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set server instance action filters
                            let mut req = ComputeServerInstanceActionShowBuilder::default();
                            req.id(selected_entry.id.clone());
                            req.server_id(selected_entry.instance_uuid.clone());
                            if let Some(name) = &self.get_filters().server_name {
                                req.server_name(name.clone());
                            }
                            command_tx.send(Action::SetComputeServerInstanceActionShowFilters(
                                req.build().wrap_err("cannot prepare request")?,
                            ))?;
                            // and switch mode
                            command_tx.send(Action::Mode {
                                mode: Mode::ComputeServerInstanceActionEvents,
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
