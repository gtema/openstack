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

use openstack_types::dns::v2::recordset::response::list::RecordsetResponse;

use crate::{
    action::Action,
    cloud_worker::dns::v2::{
        DnsApiRequest, DnsRecordsetApiRequest, DnsRecordsetList, DnsZoneApiRequest,
        DnsZoneRecordsetApiRequest, DnsZoneRecordsetList,
    },
    cloud_worker::types::ApiRequest,
    components::{Component, table_view::TableViewComponentBase},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "DNS Recordsets";
const VIEW_CONFIG_KEY: &str = "dns.recordset";

impl ResourceKey for RecordsetResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl From<DnsRecordsetList> for DnsZoneRecordsetList {
    fn from(value: DnsRecordsetList) -> Self {
        Self {
            _type: value._type,
            data: value.data,
            description: value.description,
            limit: value.limit,
            //market: value.market,
            name: value.name,
            sort_dir: value.sort_dir,
            sort_key: value.sort_key,
            status: value.status,
            ttl: value.ttl,
            zone_name: value.zone_name,
            zone_id: value.zone_id.unwrap_or_default(),
            ..Default::default()
        }
    }
}

pub type DnsRecordsets<'a> = TableViewComponentBase<'a, RecordsetResponse, DnsRecordsetList>;

impl Component for DnsRecordsets<'_> {
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
                if let Mode::DnsRecordsets = current_mode {
                    if self.get_filters().zone_id.is_some() {
                        return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                            DnsZoneRecordsetApiRequest::List(Box::new(
                                self.get_filters().clone().into(),
                            )),
                        ))));
                    } else {
                        return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                            DnsRecordsetApiRequest::List(Box::new(self.get_filters().clone())),
                        ))));
                    }
                }
            }
            Action::Mode {
                mode: Mode::DnsRecordsets,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                if self.get_filters().zone_id.is_some() {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        DnsZoneRecordsetApiRequest::List(Box::new(
                            self.get_filters().clone().into(),
                        )),
                    ))));
                } else {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        DnsRecordsetApiRequest::List(Box::new(self.get_filters().clone())),
                    ))));
                }
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::Dns(req),
                data,
            } => match req {
                DnsApiRequest::Recordset(_) => {
                    self.set_data(data)?;
                }
                DnsApiRequest::Zone(sub) => {
                    if let DnsZoneApiRequest::Recordset(_) = *sub {
                        self.set_data(data)?;
                    }
                }
            },
            Action::SetDnsRecordsetListFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                if self.get_filters().zone_id.is_some() {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        DnsZoneRecordsetApiRequest::List(Box::new(
                            self.get_filters().clone().into(),
                        )),
                    ))));
                } else {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        DnsRecordsetApiRequest::List(Box::new(self.get_filters().clone())),
                    ))));
                }
            }
            //Action::DeleteDnsRecordset => {
            //    // only if we are currently in the right mode
            //    if current_mode == Mode::DnsRecordsets {
            //        // and have command_tx
            //        if let Some(command_tx) = self.get_command_tx() {
            //            // and have a selected entry
            //            if let Some(selected_entry) = self.get_selected() {
            //                // send action to set SecurityGroupRulesListFilters
            //                command_tx.send(Action::Confirm(ApiRequest::DnsRecordsetDelete(
            //                    DnsRecordsetDelete {
            //                        zone_id: selected_entry.id.clone(),
            //                        zone_name: Some(selected_entry.name.clone()),
            //                    },
            //                )))?;
            //            }
            //        }
            //    }
            //}
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
