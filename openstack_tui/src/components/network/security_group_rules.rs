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
    cloud_worker::types::{ApiRequest, NetworkSecurityGroupRuleFilters},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{as_string, OutputConfig, StructTable},
};

const TITLE: &str = "SecurityGroupRules";

#[derive(Deserialize, StructTable)]
pub struct NetworkData {
    #[structable(title = "Id", wide)]
    id: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Ethertype")]
    ethertype: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Direction")]
    direction: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Protocol")]
    protocol: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Range Min")]
    port_range_min: String,
    #[serde(default, deserialize_with = "as_string")]
    #[structable(title = "Range Max")]
    port_range_max: String,
}

pub type NetworkSecurityGroupRules<'a> =
    TableViewComponentBase<'a, NetworkData, NetworkSecurityGroupRuleFilters>;

impl Component for NetworkSecurityGroupRules<'_> {
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
                if let Mode::NetworkSecurityGroupRules = current_mode {
                    return Ok(Some(Action::PerformApiRequest(
                        ApiRequest::NetworkSecurityGroupRules(self.get_filters().clone()),
                    )));
                }
            }
            Action::Mode {
                mode: Mode::NetworkSecurityGroupRules,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(
                    ApiRequest::NetworkSecurityGroupRules(self.get_filters().clone()),
                )));
            }
            Action::SetNetworkSecurityGroupRuleFilters(filters) => {
                self.set_filters(filters);
                self.set_data(Vec::new())?;
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(
                    ApiRequest::NetworkSecurityGroupRules(self.get_filters().clone()),
                )));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::NetworkSecurityGroupRules(_),
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
