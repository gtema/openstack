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

use crate::{
    action::Action,
    components::{Component, FuzzySelect, FuzzySelectState},
    config::Config,
    error::TuiError,
    mode::Mode,
};

const TITLE: &str = " Select cloud to connect: ";

pub struct CloudSelect {
    config: Config,
    popup_state: FuzzySelectState,
}

impl Default for CloudSelect {
    fn default() -> Self {
        Self::new()
    }
}

impl CloudSelect {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            popup_state: FuzzySelectState::new(),
        }
    }
}

impl Component for CloudSelect {
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config.clone();
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>, TuiError> {
        if let Action::Clouds(clouds) = action {
            let mut items: Vec<String> = clouds.to_vec();
            items.sort_by_key(|a| a.to_lowercase());
            self.popup_state.set_items(items);
        }
        Ok(None)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.popup_state.handle_key(&key.code);
        if key.code == KeyCode::Enter
            && let Some(cloud) = self.popup_state.selected().cloned()
        {
            return Ok(Some(Action::ConnectToCloud(cloud)));
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
