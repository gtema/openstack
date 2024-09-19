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

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use eyre::Result;
use ratatui::prelude::*;
use serde::Deserialize;
use structable_derive::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::{Action, ComputeServerFilters, Resource},
    components::{table_view::TableViewComponentBase, Component},
    config::Config,
    mode::Mode,
    utils::{OutputConfig, StructTable},
};

const TITLE: &str = "Compute Servers";

#[derive(Deserialize, StructTable)]
pub struct ServerData {
    #[structable(title = "Name")]
    name: String,
    #[structable(title = "Status")]
    status: String,
    #[structable(title = "Created")]
    created: String,
    #[structable(title = "Updated")]
    updated: String,
}

pub type ComputeServers<'a> = TableViewComponentBase<'a, ServerData, ComputeServerFilters>;

impl<'a> Component for ComputeServers<'a> {
    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.set_config(config)?;
        Ok(())
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.set_command_tx(tx);
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
                if let Mode::ComputeServers = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServers(self.get_filters().clone()),
                    )));
                }
            }
            Action::Mode(Mode::ComputeServers) | Action::Refresh => {
                self.set_loading(true);
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeServers(self.get_filters().clone()),
                )));
            }
            Action::Tick => {
                self.app_tick()?;
                if let Mode::ComputeServers = current_mode {
                    return Ok(Some(Action::RequestCloudResource(
                        Resource::ComputeServers(self.get_filters().clone()),
                    )));
                }
            }
            Action::Render => self.render_tick()?,
            Action::ResourcesData {
                resource: Resource::ComputeServers(_),
                data,
            } => {
                self.set_data(data)?;
            }
            Action::ResourceData {
                resource: Resource::ComputeServerConsoleOutput(id),
                data,
            } => {
                let server_id = self.get_selected_resource_id()?;
                if server_id == id {
                    return Ok(Some(Action::Describe(data)));
                }
            }
            Action::ServerConsoleOutput => {
                let server_id = self.get_selected_resource_id()?;
                if let Some(command_tx) = self.get_command_tx() {
                    command_tx.send(Action::Mode(Mode::Describe))?;
                }
                return Ok(Some(Action::RequestCloudResource(
                    Resource::ComputeServerConsoleOutput(server_id),
                )));
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
