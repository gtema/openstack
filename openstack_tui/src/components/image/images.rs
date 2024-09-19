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

use crossterm::event::{KeyCode, KeyEvent};
use eyre::Result;
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;

use crate::{
    action::{Action, ImageFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Images";
const INFO_TEXT: &str = "(Esc) quit | (↑) move up | (↓) move down";
const EMPTY_VAL: &str = "";
const EMPTY_VAL_STRING: String = String::new();

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

impl<'a> Component for Images<'a> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)?;
        Ok(())
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
            Action::Tick => {
                self.app_tick()?;
                // if let Mode::ComputeFlavors = current_mode {
                //     return Ok(Some(Action::RequestCloudResource(Resource::ImageImages(
                //         self.get_filters().clone(),
                //     ))));
                // }
            }
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::ImageImages(_),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::ImageFilter(filters) => {
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
        match key.code {
            KeyCode::Down => self.cursor_down()?,
            KeyCode::Up => self.cursor_up()?,
            KeyCode::Home => self.cursor_first()?,
            KeyCode::End => self.cursor_last()?,
            KeyCode::PageUp => self.cursor_page_up()?,
            KeyCode::PageDown => self.cursor_page_down()?,
            KeyCode::Left => self.cursor_left()?,
            KeyCode::Right => self.cursor_right()?,
            KeyCode::Tab => self.key_tab()?,
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        let areas = Layout::vertical([Constraint::Min(5), Constraint::Length(3)]).split(area);

        self.render_content(TITLE, f, areas[0])?;
        self.render_footer(f, areas[1]);
        Ok(())
    }
}
