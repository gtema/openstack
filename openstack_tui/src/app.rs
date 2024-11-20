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

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use ratatui::prelude::{Rect, *};
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::{debug, error, instrument};

use crate::{
    action::Action,
    cloud_worker::Cloud,
    components::{
        block_storage::{
            backups::BlockStorageBackups, snapshots::BlockStorageSnapshots,
            volumes::BlockStorageVolumes,
        },
        cloud_select_popup::CloudSelect,
        compute::{
            aggregates::ComputeAggregates, flavors::ComputeFlavors,
            hypervisors::ComputeHypervisors,
            server_instance_action_events::ComputeServerInstanceActionEvents,
            server_instance_actions::ComputeServerInstanceActions, servers::ComputeServers,
        },
        describe::Describe,
        error_popup::ErrorPopup,
        header::Header,
        home::Home,
        identity::{
            application_credentials::IdentityApplicationCredentials,
            group_users::IdentityGroupUsers, groups::IdentityGroups, projects::IdentityProjects,
            users::IdentityUsers,
        },
        image::images::Images,
        network::{
            networks::NetworkNetworks, routers::NetworkRouters,
            security_group_rules::NetworkSecurityGroupRules,
            security_groups::NetworkSecurityGroups, subnets::NetworkSubnets,
        },
        project_select_popup::ProjectSelect,
        resource_select_popup::ResourceSelect,
        Component,
    },
    config::Config,
    mode::Mode,
    tui,
    tui::{Event, Tui},
};

#[derive(Eq, Hash, PartialEq)]
enum Popup {
    Error,
    SelectResource,
    SwitchCloud,
    SwitchProject,
}

pub struct App {
    config: Config,
    tick_rate: f64,
    frame_rate: f64,
    components: HashMap<Mode, Box<dyn Component>>,
    header: Box<dyn Component>,
    should_quit: bool,
    should_suspend: bool,
    mode: Mode,
    prev_mode: Option<Mode>,
    action_tx: mpsc::UnboundedSender<Action>,
    action_rx: mpsc::UnboundedReceiver<Action>,
    cloud_worker_tx: mpsc::UnboundedSender<Action>,
    last_tick_key_events: Vec<KeyEvent>,
    cloud_name: Option<String>,
    cloud_connected: bool,
    /// Currently visible popup
    active_popup: Option<Popup>,
    /// Initialized popup components
    popups: HashMap<Popup, Box<dyn Component>>,
}

impl App {
    pub fn new(tick_rate: f64, frame_rate: f64, cloud_name: Option<String>) -> Result<Self> {
        let config = Config::new()?;
        let mode = Mode::Home;
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let (cloud_worker, mut cloud_worker_receiver) = mpsc::unbounded_channel();
        let cloud_worker_app_tx = action_tx.clone();
        tokio::spawn(async move {
            let mut cloud = Cloud::new();
            cloud
                .run(cloud_worker_app_tx, &mut cloud_worker_receiver)
                .await
                .unwrap();
        });

        // Is there a way to initialize HashMap with Box<dyn Foo> as keys in one operation?
        let mut components: HashMap<Mode, Box<dyn Component>> = HashMap::new();
        components.insert(Mode::Home, Box::new(Home::new()));
        components.insert(Mode::Describe, Box::new(Describe::new()));

        components.insert(
            Mode::BlockStorageBackups,
            Box::new(BlockStorageBackups::new()),
        );
        components.insert(
            Mode::BlockStorageSnapshots,
            Box::new(BlockStorageSnapshots::new()),
        );
        components.insert(
            Mode::BlockStorageVolumes,
            Box::new(BlockStorageVolumes::new()),
        );

        components.insert(Mode::ComputeServers, Box::new(ComputeServers::new()));
        components.insert(
            Mode::ComputeServerInstanceActions,
            Box::new(ComputeServerInstanceActions::new()),
        );
        components.insert(
            Mode::ComputeServerInstanceActionEvents,
            Box::new(ComputeServerInstanceActionEvents::new()),
        );
        components.insert(Mode::ComputeFlavors, Box::new(ComputeFlavors::new()));
        components.insert(Mode::ComputeAggregates, Box::new(ComputeAggregates::new()));
        components.insert(
            Mode::ComputeHypervisors,
            Box::new(ComputeHypervisors::new()),
        );

        components.insert(
            Mode::IdentityApplicationCredentials,
            Box::new(IdentityApplicationCredentials::new()),
        );
        components.insert(Mode::IdentityGroups, Box::new(IdentityGroups::new()));
        components.insert(
            Mode::IdentityGroupUsers,
            Box::new(IdentityGroupUsers::new()),
        );
        components.insert(Mode::IdentityProjects, Box::new(IdentityProjects::new()));
        components.insert(Mode::IdentityUsers, Box::new(IdentityUsers::new()));

        components.insert(Mode::ImageImages, Box::new(Images::new()));

        components.insert(Mode::NetworkNetworks, Box::new(NetworkNetworks::new()));
        components.insert(Mode::NetworkRouters, Box::new(NetworkRouters::new()));
        components.insert(
            Mode::NetworkSecurityGroups,
            Box::new(NetworkSecurityGroups::new()),
        );
        components.insert(
            Mode::NetworkSecurityGroupRules,
            Box::new(NetworkSecurityGroupRules::new()),
        );
        components.insert(Mode::NetworkSubnets, Box::new(NetworkSubnets::new()));

        Ok(Self {
            tick_rate,
            frame_rate,
            components,
            header: Box::new(Header::new()),
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            prev_mode: None,
            action_tx,
            action_rx,
            cloud_worker_tx: cloud_worker,
            last_tick_key_events: Vec::new(),
            cloud_name,
            cloud_connected: false,
            active_popup: None,
            popups: HashMap::from([
                (
                    Popup::SwitchProject,
                    Box::new(ProjectSelect::new()) as Box<dyn Component>,
                ),
                (Popup::Error, Box::new(ErrorPopup::new())),
                (Popup::SwitchCloud, Box::new(CloudSelect::new())),
                (Popup::SelectResource, Box::new(ResourceSelect::new())),
            ]),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut tui = tui::Tui::new()?
            .tick_rate(self.tick_rate)
            .frame_rate(self.frame_rate);
        // tui.mouse(true);
        tui.enter()?;

        self.header.register_config_handler(self.config.clone())?;

        for component in self.components.values_mut().chain(self.popups.values_mut()) {
            component.register_action_handler(self.action_tx.clone())?;
        }

        for component in self.components.values_mut().chain(self.popups.values_mut()) {
            component.register_config_handler(self.config.clone())?;
        }

        for component in self.components.values_mut().chain(self.popups.values_mut()) {
            component.init(tui.size()?)?;
        }

        let action_tx = self.action_tx.clone();

        self.cloud_worker_tx.send(Action::ListClouds)?;
        if let Some(cloud_name) = &self.cloud_name {
            action_tx.send(Action::ConnectToCloud(cloud_name.clone()))?;
        }
        action_tx.send(Action::Mode(Mode::Home))?;
        loop {
            self.handle_events(&mut tui).await?;
            self.handle_actions(&mut tui)?;

            if self.should_suspend {
                tui.suspend()?;
                action_tx.send(Action::Resume)?;
                action_tx.send(Action::ClearScreen)?;
                // tui.mouse(true);
                tui.enter()?;
            } else if self.should_quit {
                tui.stop()?;
                break;
            }
        }
        tui.exit()?;
        Ok(())
    }

    async fn handle_events(&mut self, tui: &mut Tui) -> Result<()> {
        let Some(event) = tui.next_event().await else {
            return Ok(());
        };
        let action_tx = self.action_tx.clone();
        match event {
            Event::Quit => action_tx.send(Action::Quit)?,
            Event::Tick => action_tx.send(Action::Tick)?,
            Event::Render => action_tx.send(Action::Render)?,
            Event::Resize(x, y) => action_tx.send(Action::Resize(x, y))?,
            Event::Key(key) => self.handle_key_event(key)?,
            _ => {}
        }
        if let Some(popup_type) = &self.active_popup {
            if let Some(popup) = self.popups.get_mut(popup_type) {
                if let Some(action) = popup.handle_events(Some(event.clone()))? {
                    action_tx.send(action)?;
                }
            }
        } else if let Some(component) = self.components.get_mut(&self.mode) {
            if let Some(action) = component.handle_events(Some(event.clone()))? {
                action_tx.send(action)?;
            }
        }
        if let Some(action) = self.header.handle_events(Some(event.clone()))? {
            action_tx.send(action)?;
        }

        Ok(())
    }

    #[instrument(level = "debug", skip(self))]
    fn handle_key_event(&mut self, key: KeyEvent) -> Result<()> {
        debug!("Key event received");
        let action_tx = self.action_tx.clone();
        if let Some(action) = self.config.global_keybindings.get(&vec![key]) {
            // Normal global keybinding
            action_tx.send(action.action.clone())?;
        } else if key.code == KeyCode::Esc && self.active_popup.is_some() {
            if let Some(popup_type) = &self.active_popup {
                if let Some(popup) = self.popups.get_mut(popup_type) {
                    popup.handle_key_events(key)?;
                }
            }
            // Close the popup
            self.active_popup = None;
            if !self.cloud_connected {
                self.action_tx.send(Action::CloudSelect)?;
            }
        } else if let Some(keymap) = self.config.mode_keybindings.get(&self.mode) {
            // If there is popup open do not try to convert key event to actions (since any
            // keypress may collide with shortcuts of the current mode)
            if self.active_popup.is_none() {
                if let Some(action) = keymap.get(&vec![key]) {
                    action_tx.send(action.action.clone())?;
                } else {
                    // If the key was not handled as a single key action,
                    // then consider it r multi-key combinations.
                    self.last_tick_key_events.push(key);

                    // Check for multi-key combinations
                    if let Some(action) = keymap.get(&self.last_tick_key_events) {
                        self.action_tx.send(action.action.clone())?;
                    } else if key.code == KeyCode::Esc {
                        if let Some(prev_mode) = self.prev_mode {
                            debug!("Switching to the previous mode {:?}", prev_mode);
                            self.mode = prev_mode;
                        }
                    }
                }
            }
        } else if key == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            action_tx.send(Action::Quit)?;
        } else if key.code == KeyCode::Esc {
            if let Some(prev_mode) = self.prev_mode {
                debug!("Switching to the previous mode {:?}", prev_mode);
                self.mode = prev_mode;
            }
        }
        action_tx.send(Action::Render)?;
        Ok(())
    }

    fn handle_actions(&mut self, tui: &mut Tui) -> Result<()> {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::Tick => {
                    self.last_tick_key_events.drain(..);
                }
                Action::Quit => {
                    self.should_quit = true;
                    self.cloud_worker_tx.send(Action::Quit)?;
                }
                Action::Suspend => self.should_suspend = true,
                Action::Resume => self.should_suspend = false,
                Action::ClearScreen => tui.terminal.clear()?,
                Action::Resize(w, h) => self.handle_resize(tui, w, h)?,
                Action::Render => self.render(tui)?,
                Action::Clouds(_) => {
                    // Started without any cloud selected - switch to CloudSelect mode
                    if self.cloud_name.is_none() {
                        self.action_tx.send(Action::CloudSelect)?;
                    }
                }

                Action::ConnectedToCloud(_) => {
                    if let Some(popup) = &self.active_popup {
                        if popup == &Popup::SwitchProject {
                            // Hide popup
                            self.active_popup = None;
                        }
                    }
                    self.cloud_connected = true;
                    self.render(tui)?;
                }
                Action::CloudChangeScope(ref scope) => {
                    if let Some(popup) = &self.active_popup {
                        if popup == &Popup::SwitchProject {
                            // Hide popup
                            self.active_popup = None;
                        }
                    }
                    self.render(tui)?;
                    self.cloud_worker_tx
                        .send(Action::CloudChangeScope(scope.clone()))?;
                }
                Action::ResourceSelect => {
                    self.active_popup = Some(Popup::SelectResource);
                    self.render(tui)?;
                }
                Action::CloudSelect => {
                    self.active_popup = Some(Popup::SwitchCloud);
                    self.render(tui)?;
                }
                Action::SelectProject => {
                    self.active_popup = Some(Popup::SwitchProject);
                    self.render(tui)?;
                }
                Action::Mode(mode) => {
                    if self.mode != mode {
                        debug!("Switching from {:?} to {:?}", self.mode, mode);
                        // Hide popup
                        self.active_popup = None;
                        if self.prev_mode != Some(mode) {
                            self.prev_mode = Some(self.mode);
                        }
                        self.mode = mode;
                        self.render(tui)?;
                    }
                }
                Action::RequestCloudResource(ref resource) => {
                    self.cloud_worker_tx
                        .send(Action::RequestCloudResource(resource.clone()))?;
                    self.render(tui)?;
                }
                Action::ConnectToCloud(ref cloud) => {
                    self.cloud_worker_tx
                        .send(Action::ConnectToCloud(cloud.clone()))?;
                    // Hide popup
                    self.active_popup = None;
                    self.cloud_connected = false;
                    self.render(tui)?;
                }
                Action::Error(_) => {
                    if self.mode != Mode::Home {
                        self.active_popup = Some(Popup::Error);
                        self.render(tui)?;
                    }
                }
                _ => {}
            }

            for popup in self.popups.values_mut() {
                if let Some(action) = popup.update(action.clone(), self.mode)? {
                    self.action_tx.send(action)?
                };
            }
            for (mode, component) in self.components.iter_mut() {
                // only update component if it belongs to the current mode or it is not refresh
                // event
                if *mode == self.mode
                    || (action != Action::Refresh && action != Action::DescribeResource)
                {
                    if let Some(action) = component.update(action.clone(), self.mode)? {
                        self.action_tx.send(action)?
                    };
                }
            }
            if let Some(action) = self.header.update(action.clone(), self.mode)? {
                self.action_tx.send(action)?
            };
        }
        Ok(())
    }

    #[instrument(level = "trace", skip(self, tui))]
    fn handle_resize(&mut self, tui: &mut Tui, w: u16, h: u16) -> Result<()> {
        tui.resize(Rect::new(0, 0, w, h))?;
        self.render(tui)?;
        Ok(())
    }

    fn render(&mut self, tui: &mut Tui) -> Result<()> {
        tui.draw(|f| {
            let rects = Layout::default()
                .constraints([Constraint::Min(6), Constraint::Percentage(100)].as_ref())
                .split(f.area());
            if let Err(e) = self.header.draw(f, rects[0]) {
                self.action_tx
                    .send(Action::Error(format!("Failed to draw: {:?}", e)))
                    .unwrap();
            }

            if let Some(component) = self.components.get_mut(&self.mode) {
                if let Err(e) = component.draw(f, rects[1]) {
                    error!("Error {:?}", e);
                    self.action_tx
                        .send(Action::Error(format!("Failed to draw: {:?}", e)))
                        .unwrap();
                }
            }
            if let Some(popup_type) = &self.active_popup {
                if let Some(popup) = self.popups.get_mut(popup_type) {
                    if let Err(e) = popup.draw(f, f.area()) {
                        self.action_tx
                            .send(Action::Error(format!("Failed to draw: {:?}", e)))
                            .unwrap();
                    }
                }
            }
        })?;
        Ok(())
    }
}
