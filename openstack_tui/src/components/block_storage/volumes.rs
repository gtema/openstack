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
use tracing::debug;

use crate::{
    action::Action,
    cloud_worker::block_storage::v3::{
        BlockStorageApiRequest, BlockStorageVolume, BlockStorageVolumeApiRequest,
        BlockStorageVolumeDelete, BlockStorageVolumeDeleteBuilder,
        BlockStorageVolumeDeleteBuilderError, BlockStorageVolumeList,
    },
    cloud_worker::types::ApiRequest,
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    error::TuiError,
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "Volumes";
const VIEW_CONFIG_KEY: &str = "block_storage.volume";

impl ResourceKey for BlockStorageVolume {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub type BlockStorageVolumes<'a> =
    TableViewComponentBase<'a, BlockStorageVolume, BlockStorageVolumeList>;

impl TryFrom<&BlockStorageVolume> for BlockStorageVolumeDelete {
    type Error = BlockStorageVolumeDeleteBuilderError;
    fn try_from(value: &BlockStorageVolume) -> Result<Self, Self::Error> {
        let mut builder = BlockStorageVolumeDeleteBuilder::default();
        builder.id(value.id.clone());
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

impl Component for BlockStorageVolumes<'_> {
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
                if let Mode::BlockStorageVolumes = current_mode {
                    return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                        BlockStorageVolumeApiRequest::ListDetailed(Box::new(
                            self.get_filters().clone(),
                        )),
                    ))));
                }
            }
            Action::Mode {
                mode: Mode::BlockStorageVolumes,
                ..
            }
            | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(ApiRequest::from(
                    BlockStorageVolumeApiRequest::ListDetailed(Box::new(
                        self.get_filters().clone(),
                    )),
                ))));
            }
            Action::DescribeApiResponse => self.describe_selected_entry()?,
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ApiResponsesData {
                request: ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(req)),
                data,
            } => {
                if let BlockStorageVolumeApiRequest::ListDetailed(_) = *req {
                    debug!("Data is {:?}", data);
                    self.set_data(data)?;
                }
            }
            Action::DeleteBlockStorageVolume => {
                // only if we are currently in the Volumes mode
                if current_mode == Mode::BlockStorageVolumes {
                    // and have command_tx
                    if let Some(command_tx) = self.get_command_tx() {
                        // and have a selected entry
                        if let Some(selected_entry) = self.get_selected() {
                            // send action to set SecurityGroupRulesList
                            command_tx.send(Action::Confirm(ApiRequest::from(
                                BlockStorageVolumeApiRequest::Delete(Box::new(
                                    BlockStorageVolumeDelete::try_from(selected_entry)
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
