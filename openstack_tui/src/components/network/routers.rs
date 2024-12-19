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
    cloud_worker::types::{
        ApiRequest, NetworkApiRequest, NetworkRouterApiRequest, NetworkRouterList,
    },
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Routers";

#[derive(Deserialize, StructTable)]
pub struct RouterData {
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    status: String,
    #[structable(title = "Created")]
    #[serde(rename = "created_at")]
    created: String,
    #[structable(title = "Updated")]
    #[serde(rename = "updated_at")]
    updated: String,
}

pub type NetworkRouters<'a> = TableViewComponentBase<'a, RouterData, NetworkRouterList>;

impl NetworkRouters<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(&self, mut filters: NetworkRouterList) -> NetworkRouterList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some(Vec::from(["name".into()]));
            filters.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> NetworkRouterList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

impl Component for NetworkRouters<'_> {
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
                if let Mode::NetworkRouters = current_mode {
                    self.set_loading(true);
                    self.set_data(Vec::new())?;
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        NetworkRouterApiRequest::List(Box::new(self.normalized_filters())),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::NetworkRouters,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkRouterApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            // Action::ShowRouterSubnets => {
            //     // only if we are currently in the IdentityGroup mode
            //     if current_mode == Mode::RouterNetworks {
            //         // and have command_tx
            //         if let Some(command_tx) = self.get_command_tx() {
            //             // and have a selected entry
            //             if let Some(group_row) = self.get_selected() {
            //                 command_tx.send(Action::SetRouterSubnetListFilters(
            //                     RouterSubnetListFilters {
            //                         network_id: Some(group_row.id.clone()),
            //                         network_name: Some(group_row.name.clone()),
            //                     },
            //                 ))?;
            //                 return Ok(Some(Action::Mode(Mode::RouterSubnets)));
            //             }
            //         }
            //     }
            // }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Network(NetworkApiRequest::Router(req)),
                data,
            } => {
                if let NetworkRouterApiRequest::List(_) = *req {
                    self.set_data(data)?;
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
