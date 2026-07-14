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
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action,
    components::{Component, FuzzySelect, FuzzySelectState},
    config::Config,
    error::TuiError,
    mode::Mode,
};

const TITLE: &str = " Select region to switch to: ";

pub struct RegionSelect {
    config: Config,
    regions: Vec<String>,
    popup_state: FuzzySelectState,
    is_loading: bool,
    action_tx: Option<UnboundedSender<Action>>,
    items_fetched: bool,
}

impl Default for RegionSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl RegionSelect {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            regions: Vec::new(),
            popup_state: FuzzySelectState::new(),
            is_loading: false,
            action_tx: None,
            items_fetched: false,
        }
    }
}

impl Component for RegionSelect {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.action_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config.clone();
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match &action {
            Action::ConnectToCloud(_) | Action::SwitchToRegion(_) | Action::CloudChangeScope(_) => {
                self.items_fetched = false;
                self.is_loading = true;
            }
            Action::ConnectedToCloud(_) => {
                self.is_loading = true;
                self.items_fetched = true;
                self.regions.clear();
                self.popup_state.set_items(Vec::<String>::new());
            }
            Action::SelectRegion => {
                if !self.items_fetched {
                    self.is_loading = true;
                    if let Some(tx) = &self.action_tx {
                        tx.send(Action::ListRegions)?;
                    }
                    self.items_fetched = true;
                }
            }
            Action::Regions(regions) => {
                self.regions = regions.clone();
                self.popup_state.set_items(regions.clone());
                self.is_loading = false;
            }
            _ => {}
        }
        if self.is_loading && self.action_tx.is_some() {
            if let Some(tx) = &self.action_tx {
                tx.send(Action::ListRegions)?;
            }
            self.is_loading = false;
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.popup_state.handle_key(&key.code);
        if key.code == KeyCode::Enter
            && let Some(selected_region) = self.popup_state.selected()
        {
            return Ok(Some(Action::SwitchToRegion(selected_region.clone())));
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        frame.render_stateful_widget(
            FuzzySelect::new(&self.config)
                .as_popup(true)
                .title(TITLE)
                .loading(self.is_loading),
            area,
            &mut self.popup_state,
        );
        Ok(())
    }
}
