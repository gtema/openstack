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
use tracing::debug;

use crate::{
    action::Action,
    cloud_worker::types::{
        ApiRequest, ComputeApiRequest, ComputeServerApiRequest, ComputeServerDelete,
        ComputeServerGetConsoleOutput, ComputeServerInstanceActionList, ComputeServerList,
    },
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Compute Servers";

#[derive(Deserialize, StructTable)]
pub struct ServerData {
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    status: String,
    #[structable(title = "Created")]
    created: String,
    #[structable(title = "Updated")]
    updated: String,
}

pub type ComputeServers<'a> = TableViewComponentBase<'a, ServerData, ComputeServerList>;

impl ComputeServers<'_> {}

impl Component for ComputeServers<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.set_command_tx(tx)
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::CloudChangeScope(_) | Action::ConnectToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                if let Mode::ComputeServers = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        ComputeServerApiRequest::List(self.get_filters().clone()),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::ComputeServers,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerApiRequest::List(self.get_filters().clone()),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request:
                    ApiRequest::Compute(ComputeApiRequest::Server(ComputeServerApiRequest::List(_))),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::ApiResponseData {
                request:
                    ApiRequest::Compute(ComputeApiRequest::Server(
                        ComputeServerApiRequest::GetConsoleOutput(_),
                    )),
                data,
            } => {
                if let Some(command_tx) = &self.get_command_tx() {
                    command_tx.send(Action::SetDescribeApiResponseData(data.clone()))?;
                    command_tx.send(Action::Mode {
                        mode: Mode::Describe,
                        stack: true,
                    })?;
                    self.set_loading(false);
                } else {
                    debug!("No command_tx");
                }
            }
            Action::SetComputeServerListFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerApiRequest::List(self.get_filters().clone()),
                ))));
            }
            Action::ShowServerConsoleOutput => {
                if let Some(server_id) = self.get_selected_resource_id()? {
                    if let Some(command_tx) = &self.get_command_tx() {
                        command_tx.send(Action::SetDescribeLoading(true))?;
                        command_tx.send(Action::Mode {
                            mode: Mode::Describe,
                            stack: true,
                        })?;
                    }
                    //self.set_loading(true);
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        ComputeServerApiRequest::GetConsoleOutput(ComputeServerGetConsoleOutput {
                            server_id,
                            length: Some(1000),
                        }),
                    ))));
                }
            }
            Action::ShowComputeServerInstanceActions => {
                // only if we are currently in the servers mode
                if current_mode == Mode::ComputeServers {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set SecurityGroupRulesList
                            command_tx.send(Action::SetComputeServerInstanceActionListFilters(
                                ComputeServerInstanceActionList {
                                    server_id: Some(selected_entry.id.clone()),
                                    server_name: Some(selected_entry.name.clone()),
                                    request_id: None,
                                },
                            ))?;
                            // and switch mode
                            command_tx.send(Action::Mode {
                                mode: Mode::ComputeServerInstanceActions,
                                stack: true,
                            })?;
                        }
                    }
                }
            }
            Action::DeleteComputeServer => {
                // only if we are currently in the IdentityGroup mode
                if current_mode == Mode::ComputeServers {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set SecurityGroupRulesList
                            command_tx.send(Action::Confirm(ApiRequest::from(
                                ComputeServerApiRequest::Delete(ComputeServerDelete {
                                    server_id: selected_entry.id.clone(),
                                    server_name: Some(selected_entry.name.clone()),
                                }),
                            )))?;
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
