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
use itertools::Itertools;
use ratatui::{
    layout::{
        Constraint::{Length, Min},
        Flex, Layout, Rect,
    },
    prelude::*,
    widgets::{block::*, *},
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::UnboundedSender;

use crate::{
    action::Action, cloud_worker::types::ApiRequest, components::Component, config::Config,
    error::TuiError, mode::Mode,
};

/// Single resource quota details
#[derive(Deserialize, Debug, Default, Clone)]
struct Quota {
    #[serde(alias = "in_use")]
    used: i32,
    limit: i32,
}

/// Copmute quota details
#[derive(Deserialize, Debug, Default, Clone)]
struct ComputeQuota {
    instances: Option<Quota>,
    cores: Option<Quota>,
    ram: Option<Quota>,
}

/// Network quota details
#[derive(Deserialize, Debug, Default, Clone)]
struct NetworkQuota {
    floatingip: Option<Quota>,
    network: Option<Quota>,
    subnet: Option<Quota>,
    port: Option<Quota>,
    router: Option<Quota>,
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    is_loading: bool,
    is_error: bool,
    compute_quota: Option<ComputeQuota>,
    network_quota: Option<NetworkQuota>,
    pub keymap: HashMap<KeyEvent, Action>,
    pub last_events: Vec<KeyEvent>,
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

    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }

    fn set_compute_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            let data: ComputeQuota = serde_json::from_value(data.clone())?;
            self.compute_quota = Some(data);
        }
        Ok(())
    }

    fn set_network_data(&mut self, data: Value) -> Result<(), TuiError> {
        if !data.is_null() {
            let data: NetworkQuota = serde_json::from_value(data.clone())?;
            self.network_quota = Some(data);
        }
        Ok(())
    }

    fn refresh_data(&mut self) -> Result<Option<Action>, TuiError> {
        if let Some(command_tx) = &self.command_tx {
            command_tx.send(Action::PerformApiRequest(ApiRequest::ComputeQuota))?;
            command_tx.send(Action::PerformApiRequest(ApiRequest::NetworkQuota))?;
        }
        Ok(None)
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<(), TuiError> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> Result<(), TuiError> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action, current_mode: Mode) -> Result<Option<Action>, TuiError> {
        match action {
            Action::CloudChangeScope(_) => {
                self.is_error = false;
                self.set_loading(true);
            }
            Action::ConnectedToCloud(_) => {
                self.is_error = false;
                self.set_loading(true);
                self.compute_quota = None;
                self.network_quota = None;
                if let Mode::Home = current_mode {
                    return self.refresh_data();
                }
            }
            Action::Mode {
                mode: Mode::Home, ..
            } => {
                if !self.is_loading {
                    self.set_loading(true);
                    return self.refresh_data();
                }
            }
            Action::Tick => {
                self.tick();
            }

            Action::ApiResponseData {
                request: ApiRequest::ComputeQuota { .. },
                data,
            } => {
                self.set_compute_data(data)?;
                self.set_loading(false);
            }
            Action::ApiResponseData {
                request: ApiRequest::NetworkQuota { .. },
                data,
            } => {
                self.set_network_data(data)?;
                self.set_loading(false);
            }

            Action::Error(_) => {
                if let Mode::Home = current_mode {
                    self.is_error = true;
                    self.set_loading(false);
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<(), TuiError> {
        //let rects = Layout::vertical([]).split(area);
        let mut title = vec![" Usage ".white()];
        if self.is_loading {
            title.push(Span::styled(
                " ...Loading... ",
                self.config.styles.title_loading_fg,
            ));
        } else if self.is_error {
            title.push(Span::styled(
                " (Cannot be fetched) ",
                self.config.styles.title_loading_fg,
            ));
        }

        let block = Block::default()
            .title(title)
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1))
            .border_style(Style::default().fg(self.config.styles.border_fg));

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

        if let Some(compute_quota) = &self.compute_quota {
            render_quota_gauge(
                compute_quota.instances.as_ref(),
                "Server instances",
                f,
                areas[0],
            );
            render_quota_gauge(compute_quota.cores.as_ref(), "CPU", f, areas[1]);
            render_quota_gauge(compute_quota.ram.as_ref(), "RAM (Mb)", f, areas[2]);
        }
        if let Some(network_quota) = &self.network_quota {
            render_quota_gauge(network_quota.floatingip.as_ref(), "IP", f, areas[3]);
            render_quota_gauge(network_quota.router.as_ref(), "Routers", f, areas[4]);
            render_quota_gauge(network_quota.network.as_ref(), "Networks", f, areas[5]);
            render_quota_gauge(network_quota.subnet.as_ref(), "Subnets", f, areas[6]);
            render_quota_gauge(network_quota.port.as_ref(), "Ports", f, areas[7]);
        }
        if self.compute_quota.is_none() && self.network_quota.is_none() {
            let layout = Layout::vertical([5]).flex(Flex::Center);
            let [area] = layout.areas(inner);
            let paragraph = Paragraph::new("Not available")
                .style(Style::new().red().on_black())
                .alignment(Alignment::Center);
            f.render_widget(paragraph, area);
        }

        Ok(())
    }
}

fn render_quota_gauge(quota: Option<&Quota>, title: &str, f: &mut Frame, area: Rect) {
    let mut rate: f64 = 1.0;
    let mut used: i32 = 0;
    let mut limit: i32 = 0;
    if let Some(quota) = &quota {
        used = quota.used;
        limit = quota.limit;
        if limit > 0 {
            rate = used as f64 / limit as f64;
        }
    }
    let color = match rate {
        0.0..0.5 => Color::Green,
        0.5..0.8 => Color::Yellow,
        _ => Color::Red,
    };
    let gauge = Gauge::default()
        .block(Block::bordered().title(title))
        .label(Span::styled(
            if quota.is_some() {
                if limit > 0 {
                    format!("used {}/{}", used, limit)
                } else {
                    format!("used {}/∞", used)
                }
            } else {
                String::from("N/A")
            },
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
