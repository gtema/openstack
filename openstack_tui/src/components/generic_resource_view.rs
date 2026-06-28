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

use crate::action::Action;
use crate::components::{Component, Frame};
use crate::config::Config;
use crate::error::TuiError;
use crate::mode::Mode;
use crossterm::event::KeyEvent;
use eyre::Result;
use ratatui::prelude::*;
use structable::StructTable;
use tokio::sync::mpsc::UnboundedSender;

use super::resource_behaviour::ResourceBehaviour;

/// Generic component that manages state for any resource defined by `ResourceBehaviour`.
/// It re‑uses `TableViewComponentBase` internals for data handling but delegates rendering
/// to the stateless `ResourceTable` widget.
pub struct GenericResourceView<'a, B>
where
    B: ResourceBehaviour,
    B::Item: StructTable + 'static,
    for<'b> &'b B::Item: StructTable,
{
    base: super::table_view::TableViewComponentBase<'a, B::Item, B::Filter>,
    behaviour: std::marker::PhantomData<B>,
}

impl<'a, B> GenericResourceView<'a, B>
where
    B: ResourceBehaviour,
    B::Item: StructTable + 'static,
    for<'b> &'b B::Item: StructTable,
{
    pub fn new() -> Self {
        Self {
            base: super::table_view::TableViewComponentBase::new(),
            behaviour: std::marker::PhantomData,
        }
    }
}

impl<'a, B> Default for GenericResourceView<'a, B>
where
    B: ResourceBehaviour,
    B::Item: StructTable + 'static,
    for<'b> &'b B::Item: StructTable,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, B> Component for GenericResourceView<'a, B>
where
    B: ResourceBehaviour,
    B::Item: StructTable,
    for<'b> &'b B::Item: StructTable,
{
    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.base.set_config(config)
    }

    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.base.set_command_tx(tx)
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>, TuiError> {
        self.base.handle_key_events(key)
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        // --- Generic UI actions ---
        match &action {
            Action::Tick => {
                self.base.app_tick()?;
                return Ok(None);
            }
            Action::Render => {
                self.base.render_tick()?;
                return Ok(None);
            }
            Action::DescribeApiResponse => {
                self.base.describe_selected_entry()?;
                return Ok(None);
            }
            _ => {}
        }

        // --- Cloud change scope: reset loading state ---
        if let Action::CloudChangeScope(_) = &action {
            self.base.set_loading(true);
            return Ok(None);
        }

        // --- Connected to cloud: only request data if we are the current mode ---
        if let Action::ConnectedToCloud(_) = &action {
            self.base.set_loading(true);
            self.base.set_data(Vec::new())?;
            if B::mode() == current_mode {
                let filter = B::normalise_filter(self.base.get_filters().clone());
                self.base.set_filters(filter);
                return Ok(Some(Action::PerformApiRequest(B::request_from_filter(
                    self.base.get_filters(),
                ))));
            }
            return Ok(None);
        }

        // --- Refresh or mode switch to us: request data ---
        if let Action::Refresh = &action {
            self.base.set_loading(true);
            return Ok(Some(Action::PerformApiRequest(B::request_from_filter(
                self.base.get_filters(),
            ))));
        }
        if let Action::Mode { mode, .. } = &action {
            if *mode == B::mode() {
                self.base.set_loading(true);
                return Ok(Some(Action::PerformApiRequest(B::request_from_filter(
                    self.base.get_filters(),
                ))));
            }
            return Ok(None);
        }

        // --- Filter change actions ---
        if let Some(new_filter) = B::handle_set_filter_action(&action) {
            self.base.set_loading(true);
            let filter = B::normalise_filter(new_filter);
            self.base.set_filters(filter);
            return Ok(Some(Action::PerformApiRequest(B::request_from_filter(
                self.base.get_filters(),
            ))));
        }

        // --- ApiResponsesData: only accept if request matches ---
        if let Action::ApiResponsesData { request, data } = &action {
            if B::matches_request(request) {
                self.base.set_data(data.clone())?;
                return Ok(None);
            }
            return Ok(None);
        }

        // --- Custom resource-specific actions ---
        if let Some((last, rest)) = B::custom_action(&action, self.base.get_selected()).split_last()
        {
            if let Some(tx) = self.base.get_command_tx() {
                for a in rest {
                    tx.send(a.clone())?;
                }
            }
            // Return last custom action for re-dispatch by the app loop.
            return Ok(Some(last.clone()));
        }

        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        // Reuse the existing TableViewComponentBase drawing logic which handles the
        // table, description pane and footer correctly.
        self.base.draw(f, area, B::title())
    }
}
