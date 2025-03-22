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

use crate::{
    action::Action,
    cloud_worker::dns::v2::{
        DnsApiRequest, DnsRecordsetList, DnsRecordsetListBuilder, DnsRecordsetListBuilderError,
        DnsZone, DnsZoneApiRequest, DnsZoneDelete, DnsZoneDeleteBuilder, DnsZoneDeleteBuilderError,
        DnsZoneList,
    },
    cloud_worker::types::ApiRequest,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "DNS Zones";
const VIEW_CONFIG_KEY: &str = "dns.zone";

impl ResourceKey for DnsZone {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&DnsZone> for DnsZoneDelete {
    type Error = DnsZoneDeleteBuilderError;
    fn try_from(value: &DnsZone) -> Result<Self, Self::Error> {
        let mut builder = DnsZoneDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

impl TryFrom<&DnsZone> for DnsRecordsetList {
    type Error = DnsRecordsetListBuilderError;
    fn try_from(value: &DnsZone) -> Result<Self, Self::Error> {
        let mut builder = DnsRecordsetListBuilder::default();
        if let Some(val) = &value.id {
            builder.zone_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.zone_name(val.clone());
        }
        builder.build()
    }
}

pub type DnsZones<'a> = TableViewComponentBase<'a, DnsZone, DnsZoneList>;

impl Component for DnsZones<'_> {
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
                if let Mode::DnsZones = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        DnsZoneApiRequest::List(Box::new(self.get_filters().clone())),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::DnsZones,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    DnsZoneApiRequest::List(Box::new(self.get_filters().clone())),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Dns(DnsApiRequest::Zone(req)),
                data,
            } => {
                if let DnsZoneApiRequest::List(_) = *req {
                    self.set_data(data)?;
                }
            }
            Action::SetDnsZoneListFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    DnsZoneApiRequest::List(Box::new(self.get_filters().clone())),
                ))));
            }
            Action::ShowDnsZoneRecordsets => {
                // only if we are currently in the servers mode
                if current_mode == Mode::DnsZones {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            command_tx.send(Action::SetDnsRecordsetListFilters(
                                DnsRecordsetList::try_from(selected_entry)
                                    .wrap_err("error preparing OpenStack request")?,
                            ))?;
                            // and switch mode
                            command_tx.send(Action::Mode {
                                mode: Mode::DnsRecordsets,
                                stack: true,
                            })?;
                        }
                    }
                }
            }
            Action::DeleteDnsZone => {
                // only if we are currently in the right mode
                if current_mode == Mode::DnsZones {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set filters
                            command_tx.send(Action::Confirm(ApiRequest::from(
                                DnsZoneApiRequest::Delete(Box::new(
                                    DnsZoneDelete::try_from(selected_entry)
                                        .wrap_err("error preparing OpenStack request")?,
                                )),
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
