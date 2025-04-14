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
use tokio::sync::mpsc::UnboundedSender;

use openstack_types::network::v2::network::response::list::NetworkResponse;

use crate::{
    action::Action,
    cloud_worker::network::v2::{
        NetworkApiRequest, NetworkNetworkApiRequest, NetworkNetworkList, NetworkSubnetList,
        NetworkSubnetListBuilder, NetworkSubnetListBuilderError,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "Networks";
const VIEW_CONFIG_KEY: &str = "network.network";

impl ResourceKey for NetworkResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&NetworkResponse> for NetworkSubnetList {
    type Error = NetworkSubnetListBuilderError;
    fn try_from(value: &NetworkResponse) -> Result<Self, Self::Error> {
        let mut builder = NetworkSubnetListBuilder::default();
        if let Some(val) = &value.id {
            builder.network_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.network_name(val.clone());
        }
        builder.build()
    }
}

pub type NetworkNetworks<'a> = TableViewComponentBase<'a, NetworkResponse, NetworkNetworkList>;

impl NetworkNetworks<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(&self, mut filters: NetworkNetworkList) -> NetworkNetworkList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some(Vec::from(["name".into()]));
            filters.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> NetworkNetworkList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

impl Component for NetworkNetworks<'_> {
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
                if let Mode::NetworkNetworks = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        NetworkNetworkApiRequest::List(Box::new(self.normalized_filters())),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::NetworkNetworks,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkNetworkApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            Action::ShowNetworkSubnets => {
                // only if we are currently in the expected mode
                if current_mode == Mode::NetworkNetworks {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            command_tx.send(Action::SetNetworkSubnetListFilters(
                                NetworkSubnetList::try_from(selected_entry)
                                    .wrap_err("error preparing OpenStack request")?,
                            ))?;
                            return Ok(Some(Action::Mode {
                                mode: Mode::NetworkSubnets,
                                stack: true,
                            }));
                        }
                    }
                }
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Network(NetworkApiRequest::Network(req)),
                data,
            } => {
                if let NetworkNetworkApiRequest::List(_) = *req {
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
