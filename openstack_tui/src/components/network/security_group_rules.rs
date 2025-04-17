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
use tokio::sync::mpsc::UnboundedSender;

use openstack_types::network::v2::security_group_rule::response::list::SecurityGroupRuleResponse;

use crate::{
    action::Action,
    cloud_worker::network::v2::{
        NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleList,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "SecurityGroupRules";
const VIEW_CONFIG_KEY: &str = "network.security_group_rule";

impl ResourceKey for SecurityGroupRuleResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type NetworkSecurityGroupRules<'a> =
    TableViewComponentBase<'a, SecurityGroupRuleResponse, NetworkSecurityGroupRuleList>;

impl NetworkSecurityGroupRules<'_> {
    /// Normalize filters
    ///
    /// Add preferred sorting
    fn normalize_filters(
        &self,
        mut filters: NetworkSecurityGroupRuleList,
    ) -> NetworkSecurityGroupRuleList {
        if filters.sort_key.is_none() {
            filters.sort_key = Some(vec![
                "ethertype".into(),
                "direction".into(),
                "protocol".into(),
                "port_range_min".into(),
            ]);
            filters.sort_dir = Some(vec!["asc".into(), "asc".into(), "asc".into(), "asc".into()]);
        }
        filters
    }

    /// Normalized filters
    fn normalized_filters(&self) -> NetworkSecurityGroupRuleList {
        self.normalize_filters(self.get_filters().clone())
            .to_owned()
    }
}

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
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        NetworkSecurityGroupRuleApiRequest::List(Box::new(
                            self.normalized_filters(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::NetworkSecurityGroupRules,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkSecurityGroupRuleApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            Action::SetNetworkSecurityGroupRuleListFilters(filters) => {
                self.set_filters(filters);
                self.set_data(Vec::new())?;
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    NetworkSecurityGroupRuleApiRequest::List(Box::new(self.normalized_filters())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(req)),
                data,
            } => {
                if let NetworkSecurityGroupRuleApiRequest::List(_) = *req {
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
