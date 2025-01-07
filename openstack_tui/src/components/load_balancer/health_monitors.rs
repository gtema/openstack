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
    cloud_worker::load_balancer::v2::{
        LoadBalancerApiRequest, LoadBalancerHealthmonitorApiRequest, LoadBalancerHealthmonitorList,
    },
    cloud_worker::types::ApiRequest,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, ResourceKey, StructTable},
};

const TITLE: &str = "LB HealthMonitors";
const VIEW_CONFIG_KEY: &str = "load-balancer.healthmonitor";

#[derive(Deserialize, StructTable)]
pub struct HealthMonitorData {
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    operating_status: String,
    #[structable(title = "Type")]
    r#type: String,
}

impl ResourceKey for HealthMonitorData {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type LoadBalancerHealthMonitors<'a> =
    TableViewComponentBase<'a, HealthMonitorData, LoadBalancerHealthmonitorList>;

impl Component for LoadBalancerHealthMonitors<'_> {
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
                if let Mode::LoadBalancerHealthMonitors = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        LoadBalancerHealthmonitorApiRequest::List(Box::new(
                            self.get_filters().clone(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::LoadBalancerHealthMonitors,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    LoadBalancerHealthmonitorApiRequest::List(Box::new(self.get_filters().clone())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::LoadBalancer(LoadBalancerApiRequest::Healthmonitor(res)),
                data,
            } => {
                if let LoadBalancerHealthmonitorApiRequest::List(_) = *res {
                    self.set_data(data)?;
                }
            }
            Action::SetLoadBalancerHealthMonitorListFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    LoadBalancerHealthmonitorApiRequest::List(Box::new(self.get_filters().clone())),
                ))));
            }
            // Action::DeleteLoadBalancer => {
            //     // only if we are currently in the right mode
            //     if current_mode == Mode::LoadBalancerLoadBalancers {
            //         // and have command_tx
            //         if let Some(command_tx) = self.get_command_tx() {
            //             // and have a selected entry
            //             if let Some(selected_entry) = self.get_selected() {
            //                 // send action to set SecurityGroupRulesListFilters
            //                 command_tx.send(Action::Confirm(ApiRequest::LoadBalancerLoadBalancerDelete(
            //                     LoadBalancerLoadBalancerDelete {
            //                         image_id: selected_entry.id.clone(),
            //                         image_name: Some(selected_entry.name.clone()),
            //                     },
            //                 )))?;
            //             }
            //         }
            //     }
            // }
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
