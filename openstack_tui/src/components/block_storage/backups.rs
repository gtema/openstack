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
    cloud_worker::block_storage::v3::{
        BlockStorageApiRequest, BlockStorageBackupApiRequest, BlockStorageBackupList,
    },
    cloud_worker::types::ApiRequest,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Backups";

#[derive(Deserialize, StructTable)]
pub struct BackupData {
    #[structable(title = "Id", wide)]
    id: String,
    #[structable(title = "Name", optional)]
    #[serde(default)]
    name: Option<String>,
    #[structable(title = "AZ", optional)]
    #[serde(default)]
    availability_zone: Option<String>,
    #[structable(title = "Size")]
    #[serde(default)]
    size: u32,
    #[structable(title = "Status")]
    #[serde(default)]
    status: String,
    #[structable(title = "Created")]
    #[serde(default)]
    created_at: String,
}

pub type BlockStorageBackups<'a> = TableViewComponentBase<'a, BackupData, BlockStorageBackupList>;

impl Component for BlockStorageBackups<'_> {
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
                if let Mode::BlockStorageBackups = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        BlockStorageBackupApiRequest::ListDetailed(Box::new(
                            self.get_filters().clone(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::BlockStorageBackups,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    BlockStorageBackupApiRequest::ListDetailed(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::BlockStorage(BlockStorageApiRequest::Backup(req)),
                data,
            } => {
                if let BlockStorageBackupApiRequest::ListDetailed(_) = *req {
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
