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
use serde_json::json;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;
use tracing::debug;

use crate::{
    action::Action,
    cloud_worker::compute::v2::{
        ComputeApiRequest, ComputeServerApiRequest, ComputeServerDelete,
        ComputeServerGetConsoleOutputBuilder, ComputeServerInstanceActionListBuilder,
        ComputeServerList,
    },
    cloud_worker::types::ApiRequest,
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

impl ComputeServers<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(&self, mut filters: ComputeServerList) -> ComputeServerList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some("display_name".into());
            filters.sort_dir = Some("asc".into());
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> ComputeServerList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

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
                        ComputeServerApiRequest::ListDetailed(Box::new(self.normalized_filters())),
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
                    ComputeServerApiRequest::ListDetailed(Box::new(self.normalized_filters())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Compute(ComputeApiRequest::Server(req)),
                data,
            } => {
                if let ComputeServerApiRequest::ListDetailed(_) = *req {
                    self.set_data(data)?;
                }
            }
            Action::ApiResponseData {
                request: ApiRequest::Compute(ComputeApiRequest::Server(req)),
                data,
            } => {
                if let ComputeServerApiRequest::GetConsoleOutput(_) = *req {
                    if let Some(command_tx) = &self.get_command_tx() {
                        command_tx.send(Action::SetDescribeApiResponseData(
                            data.get("output")
                                .unwrap_or(&json!("bad data returned by API"))
                                .to_owned(),
                        ))?;
                        command_tx.send(Action::Mode {
                            mode: Mode::Describe,
                            stack: true,
                        })?;
                        self.set_loading(false);
                    } else {
                        debug!("No command_tx");
                    }
                }
            }
            Action::SetComputeServerListFilters(filters) => {
                self.set_filters(*filters);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeServerApiRequest::ListDetailed(Box::new(self.get_filters().clone())),
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
                        ComputeServerApiRequest::GetConsoleOutput(Box::new(
                            ComputeServerGetConsoleOutputBuilder::default()
                                .id(server_id.clone())
                                .os_get_console_output(crate::cloud_worker::compute::v2::server::get_console_output::OsGetConsoleOutputBuilder::default().build().wrap_err("cannot prepare os-get-console-output structure")?)
                                .build()
                                .wrap_err("cannot prepare request")?,
                        )),
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
                                Box::new(
                                    ComputeServerInstanceActionListBuilder::default()
                                        .server_id(selected_entry.id.clone())
                                        .server_name(selected_entry.name.clone())
                                        .build()
                                        .wrap_err("cannot prepare request")?,
                                ),
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
                                ComputeServerApiRequest::Delete(Box::new(ComputeServerDelete {
                                    id: selected_entry.id.clone(),
                                    name: Some(selected_entry.name.clone()),
                                })),
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
