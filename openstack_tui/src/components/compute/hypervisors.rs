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

use openstack_types::compute::v2::hypervisor::response::list_detailed::HypervisorResponse;

use crate::{
    action::Action,
    cloud_worker::compute::v2::{
        ComputeApiRequest, ComputeHypervisorApiRequest, ComputeHypervisorList,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "Compute Hypervisors";
const VIEW_CONFIG_KEY: &str = "compute.hypervisor";

impl ResourceKey for HypervisorResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type ComputeHypervisors<'a> =
    TableViewComponentBase<'a, HypervisorResponse, ComputeHypervisorList>;

impl Component for ComputeHypervisors<'_> {
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
                if let Mode::ComputeHypervisors = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        ComputeHypervisorApiRequest::ListDetailed(Box::new(
                            self.get_filters().clone(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::ComputeHypervisors,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    ComputeHypervisorApiRequest::ListDetailed(Box::new(self.get_filters().clone())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Compute(ComputeApiRequest::Hypervisor(req)),
                data,
            } => {
                if let ComputeHypervisorApiRequest::ListDetailed(_) = *req {
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
