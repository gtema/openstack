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

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;

use crate::{
    action::{Action, ComputeFlavorFilters, Resource},
    components::{table_view::TableViewComponentBase, Component, Frame},
    config::Config,
    mode::Mode,
    utils::{as_string, OutputConfig, StructTable},
};

const TITLE: &str = "Compute Flavors";

#[derive(Deserialize, StructTable)]
pub struct FlavorData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "vCPU")]
    #[serde(deserialize_with = "as_string")]
    vcpus: String,
    #[serde(deserialize_with = "as_string")]
    ram: String,
    #[serde(deserialize_with = "as_string")]
    disk: String,
    #[serde(rename = "OS-FLV-DISABLED:disabled", deserialize_with = "as_string")]
    disabled: String,
}

pub type ComputeFlavors<'a> = TableViewComponentBase<'a, FlavorData, ComputeFlavorFilters>;

impl<'a> Component for ComputeFlavors<'a> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)?;
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::Mode(Mode::ComputeFlavors) | Action::Refresh | Action::ConnectToCloud(_) => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeFlavors(self.get_filters().clone()),
                )));
            }
            Action::Tick => {
                self.app_tick()?;
                if let Mode::ComputeFlavors = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeFlavors(self.get_filters().clone()),
                    )));
                }
            }
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::ComputeFlavors(_),
                data,
            } => {
                self.set_data(data)?;
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
        if key.kind == KeyEventKind::Press && key.code == KeyCode::Enter {
            return Ok(Some(Action::Describe(self.get_selected_raw().clone())));
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
