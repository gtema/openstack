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
};

const TITLE: &str = " Select resource to display ";

pub struct ApiRequestSelect {
    config: Config,
    popup_state: FuzzySelectState,
}

impl Default for ApiRequestSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiRequestSelect {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            popup_state: FuzzySelectState::new(),
        }
    }
}

impl Component for ApiRequestSelect {
    fn register_action_handler(&mut self, _tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config.clone();
        let items: Vec<String> = self.config.mode_aliases.keys().cloned().collect();
        self.popup_state.set_items(items);
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.popup_state.handle_key(&key.code);
        if key.code == KeyCode::Enter
            && let Some(selected_name) = self.popup_state.selected()
            && let Some(mode) = self.config.mode_aliases.get(selected_name.as_str())
        {
            self.popup_state.reset_filter();
            return Ok(Some(Action::Mode {
                mode: *mode,
                stack: false,
            }));
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        frame.render_stateful_widget(
            FuzzySelect::new(&self.config).as_popup(true).title(TITLE),
            area,
            &mut self.popup_state,
        );
        Ok(())
    }
}
