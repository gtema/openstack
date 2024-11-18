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

use crate::{
    action::Action,
    cloud_worker::types::{ImageFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Images";

#[derive(Deserialize, StructTable)]
pub struct ImageData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Distro")]
    #[serde(rename = "os_distro", default)]
    distro: String,
    #[structable(title = "Version")]
    #[serde(rename = "os_version", default)]
    version: String,
    #[structable(title = "Arch")]
    #[serde(rename = "architecture", default)]
    arch: String,
    #[structable(title = "Visibility")]
    #[serde(rename = "visibility", default)]
    visibility: String,
}

pub type Images<'a> = TableViewComponentBase<'a, ImageData, ImageFilters>;

impl Component for Images<'_> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::CloudChangeScope(_) => {
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.set_loading(true);
                self.set_data(Vec::new())?;
                if let Mode::ImageImages = current_mode {
                    return Ok(Some(Action::RequestCloudResource(Resource::ImageImages(
                        self.get_filters().clone(),
                    ))));
                }
            }
            Action::Mode(Mode::ImageImages) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(Resource::ImageImages(
                    self.get_filters().clone(),
                ))));
            }
            Action::Tick => self.app_tick()?,
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::ImageImages(_),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::SetImageFilters(filters) => {
                self.set_filters(filters);
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(Resource::ImageImages(
                    self.get_filters().clone(),
                ))));
            }
            _ => {}
        };
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
        self.handle_key_events(key)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        self.draw(f, area, TITLE)
    }
}
