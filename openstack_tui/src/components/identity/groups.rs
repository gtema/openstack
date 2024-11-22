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
    cloud_worker::types::{ApiRequest, IdentityGroupFilters, IdentityGroupUserFilters},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Identity Groups";

/// Group resource data
#[derive(Deserialize, StructTable)]
pub struct GroupData {
    /// Group ID (not shown by default but necessary for fetching related resources)
    #[structable(title = "Id", wide)]
    id: String,
    /// Name
    #[structable(title = "Name")]
    name: String,
    /// Domain ID
    #[structable(title = "Domain")]
    domain_id: String,
    /// Group description
    #[structable(title = "Description")]
    description: String,
}

pub type IdentityGroups<'a> = TableViewComponentBase<'a, GroupData, IdentityGroupFilters>;

impl Component for IdentityGroups<'_> {
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
                if let Mode::IdentityGroups = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::IdentityGroups(
                        self.get_filters().clone(),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::IdentityGroups,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::IdentityGroups(
                    self.get_filters().clone(),
                ))));
            }
            Action::ShowIdentityGroupUsers => {
                // only if we are currently in the IdentityGroup mode
                if current_mode == Mode::IdentityGroups {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(group_row) = self.get_selected() {
                            // send action to set GroupUserFilters
                            command_tx.send(Action::SetIdentityGroupUserFilters(
                                IdentityGroupUserFilters {
                                    group_id: group_row.id.clone(),
                                    group_name: Some(group_row.name.clone()),
                                },
                            ))?;
                            // and switch mode
                            command_tx.send(Action::Mode {
                                mode: Mode::IdentityGroupUsers,
                                stack: true,
                            })?;
                        }
                    }
                }
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::IdentityGroups(_),
                data,
            } => {
                self.set_data(data)?;
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
