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

use super::{Component, Frame};
use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;
use itertools::Itertools;
use ratatui::{
    layout::{
        Constraint::{Length, Min},
        Layout, Rect,
    },
    prelude::*,
    widgets::{block::*, *},
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::{Action, Resource},
    config::Config,
    mode::Mode,
    utils::PALETTES,
};

#[derive(Deserialize, Debug, Default, Clone)]
struct Quota {
    /// The number of allowed members for each server group.
    ///
    in_use: i32,
    limit: i32,
    reserved: i32,
}

#[derive(Deserialize, Debug, Default, Clone)]
struct ComputeQuota {
    instances: Quota,
    cores: Quota,
    ram: Quota,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    compute_quota: ComputeQuota,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
    ratio_compute_servers: f64,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn keymap(mut self, keymap: HashMap<KeyEvent, Action>) -> Self {
        self.keymap = keymap;
        self
    }

    pub fn tick(&mut self) {
        self.last_events.drain(..);
    }

    pub fn render_tick(&mut self) {}

    fn set_compute_data(&mut self, data: Value) -> Result<()> {
        let data: ComputeQuota = serde_json::from_value(data.clone())?;
        self.compute_quota = data;
        Ok(())
    }

    fn refresh_data(&mut self) -> Result<Option<Action>> {
        Ok(Some(Action::RequestCloudResource(Resource::ComputeQuota)))
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, _current_mode: Mode) -> Result<Option<Action>> {
        match action {
            Action::Mode(Mode::Home) => {
                return self.refresh_data();
            }
            Action::Tick => {
                self.tick();
                //if let Mode::Home = current_mode {
                //    return self.refresh_data();
                //}
            }

            Action::ResourceData {
                resource: Resource::ComputeQuota { .. },
                data,
            } => {
                self.set_compute_data(data)?;
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
        //let rects = Layout::vertical([]).split(area);
        let block = Block::default()
            .title(Title::from(" Usage ").alignment(Alignment::Center))
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(PALETTES[0].c900));

        let inner = block.inner(area);
        f.render_widget(block, area);

        let rows = Layout::vertical([
            Length(4),
            Length(4),
            Length(4),
            Length(4),
            Length(4),
            Min(0), // fills remaining space
        ])
        .split(inner);

        let areas = rows
            .iter()
            .flat_map(|area| {
                Layout::horizontal([
                    Length(25),
                    Length(25),
                    Length(25),
                    Length(25),
                    Length(25),
                    Min(0), // fills remaining space
                ])
                .split(*area)
                .iter()
                .copied()
                .take(5) // ignore Min(0)
                .collect_vec()
            })
            .collect_vec();

        render_quota_gauge(
            &self.compute_quota.instances,
            "Server instances",
            f,
            areas[0],
        );
        render_quota_gauge(&self.compute_quota.cores, "CPU", f, areas[1]);
        render_quota_gauge(&self.compute_quota.ram, "RAM (Mb)", f, areas[2]);

        Ok(())
    }
}

fn render_quota_gauge(quota: &Quota, title: &str, f: &mut Frame, area: Rect) {
    if quota.limit > 0 {
        let rate = quota.in_use as f64 / quota.limit as f64;
        let color = match rate {
            0.0..0.5 => Color::Green,
            0.5..0.8 => Color::Yellow,
            _ => Color::Red,
        };
        let gauge = Gauge::default()
            .block(Block::bordered().title(title))
            .label(Span::styled(
                format!("used {}/{}", quota.in_use, quota.limit),
                color,
            ))
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .ratio(rate);
        f.render_widget(gauge, area);
    }
}
