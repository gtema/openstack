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
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info, instrument};

use crate::cli::Cli;
use crate::{
    action::Action,
    cloud_worker::{AuthAction, Cloud},
    components::{
        Component, auth_helper::AuthHelper, cloud_select_popup::CloudSelect,
        confirm_popup::ConfirmPopup, describe::Describe, error_popup::ErrorPopup, header::Header,
        home::Home, project_select_popup::ProjectSelect, region_select_popup::RegionSelect,
        resource_select_popup::ApiRequestSelect,
    },
    config::Config,
    error::TuiError,
    mode::Mode,
    tui,
    tui::{Event, Tui},
};

#[derive(Debug, Eq, Hash, PartialEq)]
enum Popup {
    AuthHelper,
    Error,
    SelectApiRequest,
    SwitchCloud,
    SwitchProject,
    SwitchRegion,
    Confirm,
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
    /// Stack of the mode changes so that <Esc> brings user back in connected modes
    mode_switch_stack: Vec<Mode>,
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
    pub fn new(args: &Cli) -> Result<Self> {
        let config = Config::new()?;
        let mode = Mode::Home;
        let (action_tx, action_rx) = mpsc::unbounded_channel();

        let (cloud_worker, mut cloud_worker_receiver) = mpsc::unbounded_channel();
        let cloud_worker_app_tx = action_tx.clone();
        let client_config = args.os_client_config_file.clone();
        let client_secure_config = args.os_client_secure_file.clone();

        // Is there a way to initialize HashMap with Box<dyn Foo> as keys in one operation?
        let mut components: HashMap<Mode, Box<dyn Component>> = HashMap::new();
        components.insert(Mode::Home, Box::new(Home::new()));
        components.insert(Mode::Describe, Box::new(Describe::new()));

        // BEGIN GENERATED rust-tui-view app-insert block_storage.backup
        components.insert(
            Mode::Resource(crate::mode::BLOCK_STORAGE_BACKUP),
            Box::new(crate::components::block_storage::backups::BlockStorageBackups::new()),
        );
        // END GENERATED rust-tui-view app-insert block_storage.backup
        // BEGIN GENERATED rust-tui-view app-insert block_storage.snapshot
        components.insert(
            Mode::Resource(crate::mode::BLOCK_STORAGE_SNAPSHOT),
            Box::new(crate::components::block_storage::snapshots::BlockStorageSnapshots::new()),
        );
        // END GENERATED rust-tui-view app-insert block_storage.snapshot
        // BEGIN GENERATED rust-tui-view app-insert block_storage.volume
        components.insert(
            Mode::Resource(crate::mode::BLOCK_STORAGE_VOLUME),
            Box::new(crate::components::block_storage::volumes::BlockStorageVolumes::new()),
        );
        // END GENERATED rust-tui-view app-insert block_storage.volume

        // BEGIN GENERATED rust-tui-view app-insert compute.server
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_SERVER),
            Box::new(crate::components::compute::servers::ComputeServers::new()),
        );
        // END GENERATED rust-tui-view app-insert compute.server
        // BEGIN GENERATED rust-tui-view app-insert compute.server/instance_action
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION),
            Box::new(
                crate::components::compute::server_instance_actions::ComputeServerInstanceActions::new(),
            ),
        );
        // END GENERATED rust-tui-view app-insert compute.server/instance_action
        // BEGIN GENERATED rust-tui-view app-insert compute.server/instance_action/event
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_SERVER_INSTANCE_ACTION_EVENT),
            Box::new(
                crate::components::compute::server_instance_action_events::ComputeServerInstanceActionEvents::new(),
            ),
        );
        // END GENERATED rust-tui-view app-insert compute.server/instance_action/event
        // BEGIN GENERATED rust-tui-view app-insert compute.flavor
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_FLAVOR),
            Box::new(crate::components::compute::flavors::ComputeFlavors::new()),
        );
        // END GENERATED rust-tui-view app-insert compute.flavor
        // BEGIN GENERATED rust-tui-view app-insert compute.aggregate
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_AGGREGATE),
            Box::new(crate::components::compute::aggregates::ComputeAggregates::new()),
        );
        // END GENERATED rust-tui-view app-insert compute.aggregate
        // BEGIN GENERATED rust-tui-view app-insert compute.hypervisor
        components.insert(
            Mode::Resource(crate::mode::COMPUTE_HYPERVISOR),
            Box::new(crate::components::compute::hypervisors::ComputeHypervisors::new()),
        );
        // END GENERATED rust-tui-view app-insert compute.hypervisor

        // BEGIN GENERATED rust-tui-view app-insert dns.recordset
        components.insert(
            Mode::Resource(crate::mode::DNS_RECORDSET),
            Box::new(crate::components::dns::recordsets::DnsRecordsets::new()),
        );
        // END GENERATED rust-tui-view app-insert dns.recordset
        // BEGIN GENERATED rust-tui-view app-insert dns.zone
        components.insert(
            Mode::Resource(crate::mode::DNS_ZONE),
            Box::new(crate::components::dns::zones::DnsZones::new()),
        );
        // END GENERATED rust-tui-view app-insert dns.zone

        // BEGIN GENERATED rust-tui-view app-insert identity.user/application_credential
        components.insert(
            Mode::Resource(crate::mode::IDENTITY_APPLICATION_CREDENTIAL),
            Box::new(
                crate::components::identity::application_credentials::IdentityApplicationCredentials::new(),
            ),
        );
        // END GENERATED rust-tui-view app-insert identity.user/application_credential
        // BEGIN GENERATED rust-tui-view app-insert identity.group
        components.insert(
            Mode::Resource(crate::mode::IDENTITY_GROUP),
            Box::new(crate::components::identity::groups::IdentityGroups::new()),
        );
        // END GENERATED rust-tui-view app-insert identity.group
        // BEGIN GENERATED rust-tui-view app-insert identity.group_user
        components.insert(
            Mode::Resource(crate::mode::IDENTITY_GROUP_USER),
            Box::new(crate::components::identity::group_users::IdentityGroupUsers::new()),
        );
        // END GENERATED rust-tui-view app-insert identity.group_user
        // BEGIN GENERATED rust-tui-view app-insert identity.project
        components.insert(
            Mode::Resource(crate::mode::IDENTITY_PROJECT),
            Box::new(crate::components::identity::projects::IdentityProjects::new()),
        );
        // END GENERATED rust-tui-view app-insert identity.project
        // BEGIN GENERATED rust-tui-view app-insert identity.user
        components.insert(
            Mode::Resource(crate::mode::IDENTITY_USER),
            Box::new(crate::components::identity::users::IdentityUsers::new()),
        );
        // END GENERATED rust-tui-view app-insert identity.user

        // BEGIN GENERATED rust-tui-view app-insert image.image
        components.insert(
            Mode::Resource(crate::mode::IMAGE_IMAGE),
            Box::new(crate::components::image::images::Images::new()),
        );
        // END GENERATED rust-tui-view app-insert image.image

        // BEGIN GENERATED rust-tui-view app-insert load-balancer.loadbalancer
        components.insert(
            Mode::Resource(crate::mode::LB_LOADBALANCER),
            Box::new(crate::components::load_balancer::loadbalancers::LoadBalancers::new()),
        );
        // END GENERATED rust-tui-view app-insert load-balancer.loadbalancer
        // BEGIN GENERATED rust-tui-view app-insert load-balancer.listener
        components.insert(
            Mode::Resource(crate::mode::LB_LISTENER),
            Box::new(crate::components::load_balancer::listeners::LoadBalancerListeners::new()),
        );
        // END GENERATED rust-tui-view app-insert load-balancer.listener
        // BEGIN GENERATED rust-tui-view app-insert load-balancer.pool
        components.insert(
            Mode::Resource(crate::mode::LB_POOL),
            Box::new(crate::components::load_balancer::pools::LoadBalancerPools::new()),
        );
        // END GENERATED rust-tui-view app-insert load-balancer.pool
        // BEGIN GENERATED rust-tui-view app-insert load-balancer.pool/member
        components.insert(
            Mode::Resource(crate::mode::LB_POOL_MEMBER),
            Box::new(
                crate::components::load_balancer::pool_members::LoadBalancerPoolMembers::new(),
            ),
        );
        // END GENERATED rust-tui-view app-insert load-balancer.pool/member
        // BEGIN GENERATED rust-tui-view app-insert load-balancer.healthmonitor
        components.insert(
            Mode::Resource(crate::mode::LB_HEALTHMONITOR),
            Box::new(
                crate::components::load_balancer::health_monitors::LoadBalancerHealthMonitors::new(
                ),
            ),
        );
        // END GENERATED rust-tui-view app-insert load-balancer.healthmonitor

        // BEGIN GENERATED rust-tui-view app-insert network.network
        components.insert(
            Mode::Resource(crate::mode::NETWORK_NETWORK),
            Box::new(crate::components::network::networks::NetworkNetworks::new()),
        );
        // END GENERATED rust-tui-view app-insert network.network
        // BEGIN GENERATED rust-tui-view app-insert network.router
        components.insert(
            Mode::Resource(crate::mode::NETWORK_ROUTER),
            Box::new(crate::components::network::routers::NetworkRouters::new()),
        );
        // END GENERATED rust-tui-view app-insert network.router
        // BEGIN GENERATED rust-tui-view app-insert network.security_group
        components.insert(
            Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP),
            Box::new(crate::components::network::security_groups::NetworkSecurityGroups::new()),
        );
        // END GENERATED rust-tui-view app-insert network.security_group
        // BEGIN GENERATED rust-tui-view app-insert network.security_group_rule
        components.insert(
            Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP_RULE),
            Box::new(
                crate::components::network::security_group_rules::NetworkSecurityGroupRules::new(),
            ),
        );
        // END GENERATED rust-tui-view app-insert network.security_group_rule
        // GENERATED-ANCHOR: component insert
        // BEGIN GENERATED rust-tui-view app-insert network.subnet
        components.insert(
            Mode::Resource(crate::mode::NETWORK_SUBNET),
            Box::new(crate::components::network::subnets::NetworkSubnets::new()),
        );
        // END GENERATED rust-tui-view app-insert network.subnet

        let (auth_helper_control_channel_tx, auth_helper_control_channel_rx) =
            mpsc::channel::<oneshot::Sender<AuthAction>>(10);

        let mut popups: HashMap<Popup, Box<dyn Component>> = HashMap::new();
        popups.insert(Popup::SwitchProject, Box::new(ProjectSelect::new()));
        popups.insert(Popup::SwitchRegion, Box::new(RegionSelect::new()));
        popups.insert(Popup::Error, Box::new(ErrorPopup::new()));
        popups.insert(Popup::SwitchCloud, Box::new(CloudSelect::new()));
        popups.insert(Popup::SelectApiRequest, Box::new(ApiRequestSelect::new()));
        popups.insert(
            Popup::AuthHelper,
            Box::new(AuthHelper::new(auth_helper_control_channel_rx)),
        );

        let auth_helper_control_channel_tx_clone = auth_helper_control_channel_tx.clone();
        let mut cloud = Cloud::new(
            client_config,
            client_secure_config,
            auth_helper_control_channel_tx_clone,
        )?;
        tokio::spawn(async move {
            if let Err(err) = cloud
                .run(cloud_worker_app_tx, &mut cloud_worker_receiver)
                .await
            {
                info!("Error in the cloud worker: {}", err);
            }
        });
        Ok(Self {
            tick_rate: args.tick_rate,
            frame_rate: args.frame_rate,
            components,
            header: Box::new(Header::new()),
            should_quit: false,
            should_suspend: false,
            config,
            mode,
            mode_switch_stack: Vec::new(),
            action_tx,
            action_rx,
            cloud_worker_tx: cloud_worker,
            last_tick_key_events: Vec::new(),
            cloud_name: args.os_cloud.clone(),
            cloud_connected: false,
            active_popup: None,
            popups,
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
        action_tx.send(Action::Mode {
            mode: Mode::Home,
            stack: false,
        })?;
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
            if let Some(popup) = self.popups.get_mut(popup_type)
                && let Some(action) = popup.handle_events(Some(event.clone()))?
            {
                action_tx.send(action)?;
            }
        } else if let Some(component) = self.components.get_mut(&self.mode)
            && let Some(action) = component.handle_events(Some(event.clone()))?
        {
            action_tx.send(action)?;
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
        if self.active_popup.is_none()
            && let Some(action) = self.config.global_keybindings.get(&vec![key])
        {
            // Normal global keybinding
            action_tx.send(action.action.clone())?;
        } else if key == KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL) {
            action_tx.send(Action::Quit)?;
        } else if key.code == KeyCode::Esc && self.active_popup.is_some() {
            if let Some(popup_type) = &self.active_popup
                && let Some(popup) = self.popups.get_mut(popup_type)
            {
                popup.handle_key_events(key)?;
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
                    } else if key.code == KeyCode::Esc && self.mode_switch_stack.len() > 1 {
                        // remove the current mode from the stack
                        self.mode_switch_stack.pop();
                        if let Some(prev_mode) = self.mode_switch_stack.last() {
                            debug!("Switching to the previous mode {:?}", prev_mode);
                            self.mode = *prev_mode;
                            self.action_tx.send(Action::PrevMode)?;
                        }
                    }
                }
            }
        } else if key.code == KeyCode::Esc && self.mode_switch_stack.len() > 1 {
            // remove the current mode from the stack
            self.mode_switch_stack.pop();
            if let Some(prev_mode) = self.mode_switch_stack.last() {
                debug!("Switching to the previous mode {:?}", prev_mode);
                self.mode = *prev_mode;
                self.action_tx.send(Action::PrevMode)?;
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
                Action::Clouds(_)
                    // Started without any cloud selected - switch to CloudSelect mode
                    if self.cloud_name.is_none() => {
                        self.action_tx.send(Action::CloudSelect)?;
                    }

                Action::ConnectedToCloud(_) => {
                    if let Some(popup) = &self.active_popup
                        && (popup == &Popup::SwitchProject || popup == &Popup::SwitchRegion)
                    {
                        // Hide popup
                        self.active_popup = None;
                    }
                    self.cloud_connected = true;
                    self.render(tui)?;
                }
                Action::CloudChangeScope(ref scope) => {
                    if let Some(popup) = &self.active_popup
                        && (popup == &Popup::SwitchProject || popup == &Popup::SwitchRegion)
                    {
                        // Hide popup
                        self.active_popup = None;
                    }
                    // Broadcast to all popups so they can clear stale data
                    for popup in self.popups.values_mut() {
                        let _ = popup.update(action.clone(), self.mode);
                    }
                    self.render(tui)?;
                    self.cloud_worker_tx
                        .send(Action::CloudChangeScope(scope.clone()))?;
                }
                Action::SwitchToRegion(ref region) => {
                    if let Some(popup) = &self.active_popup
                        && popup == &Popup::SwitchRegion
                    {
                        // Hide popup
                        self.active_popup = None;
                    }
                    // Broadcast to all popups so they can clear stale data
                    for popup in self.popups.values_mut() {
                        let _ = popup.update(action.clone(), self.mode);
                    }
                    self.render(tui)?;
                    self.cloud_worker_tx
                        .send(Action::SwitchToRegion(region.clone()))?;
                }
                Action::ApiRequestSelect => {
                    self.active_popup = Some(Popup::SelectApiRequest);
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
                Action::ListClouds => {
                    self.cloud_worker_tx.send(Action::ListClouds)?;
                }
                Action::ListRegions => {
                    self.cloud_worker_tx.send(Action::ListRegions)?;
                }
                Action::SelectRegion => {
                    self.active_popup = Some(Popup::SwitchRegion);
                    self.render(tui)?;
                }
                Action::AuthDataRequired { .. } => {
                    self.active_popup = Some(Popup::AuthHelper);
                    self.render(tui)?;
                }
                Action::Edit {
                    ref template,
                    ref original_action,
                } => {
                    tui.exit()?;
                    let edited = edit::edit(template)?;
                    tracing::debug!("after editing: '{}'", edited);
                    tui.enter()?;
                    tui.terminal.clear()?;
                    self.action_tx.send(Action::EditResult {
                        result: serde_json::from_value(serde_yaml::from_str(&edited)?)?,
                        original_action: original_action.clone(),
                    })?;
                    self.render(tui)?;
                }
                Action::AuthHelperCompleted => {
                    self.active_popup = None;
                    self.render(tui)?;
                }
                Action::Mode { mode, stack }
                    if self.mode != mode => {
                        debug!("Switching from {:?} to {:?}", self.mode, mode);
                        // Hide popup
                        self.active_popup = None;
                        if !stack {
                            self.mode_switch_stack.clear();
                        }
                        self.mode_switch_stack.push(mode);
                        self.mode = mode;
                        self.action_tx.send(Action::Refresh)?;
                        self.render(tui)?;
                    }
                Action::PerformApiRequest(ref request) => {
                    self.cloud_worker_tx
                        .send(Action::PerformApiRequest(request.clone()))?;
                    self.render(tui)?;
                }
                Action::ConnectToCloud(ref cloud) => {
                    self.cloud_worker_tx
                        .send(Action::ConnectToCloud(cloud.clone()))?;
                    self.active_popup = None;
                    self.cloud_connected = false;
                    // Broadcast to all popups so they can clear stale data
                    for popup in self.popups.values_mut() {
                        let _ = popup.update(action.clone(), self.mode);
                    }
                    // Let the active mode component know to refresh
                    self.action_tx.send(Action::Refresh)?;
                    self.render(tui)?;
                }
                Action::Confirm(ref request) => {
                    debug!("Need to confirm {:?}", request);
                    self.active_popup = Some(Popup::Confirm);
                    if let Some(popup) = self.popups.get_mut(&Popup::Confirm) {
                        // ConfirmPopup already exists, reuse by downcasting
                        if let Some(confirm_popup) = popup.as_any_mut().downcast_mut::<ConfirmPopup>() {
                            confirm_popup.update_request(request);
                        }
                    } else {
                        let mut confirm_popup = ConfirmPopup::new(request);
                        confirm_popup.register_action_handler(self.action_tx.clone())?;
                        self.popups.insert(Popup::Confirm, Box::new(confirm_popup));
                    }
                    self.render(tui)?;
                }
                Action::ConfirmRejected(ref _request) => {
                    debug!("Action not confirmed");
                    self.active_popup = None;
                    self.render(tui)?;
                }
                Action::ConfirmAccepted(ref request) => {
                    debug!("Action confirmed");
                    self.cloud_worker_tx
                        .send(Action::PerformApiRequest(request.clone()))?;

                    self.active_popup = None;
                    self.render(tui)?;
                }
                Action::Error { .. } => {
                    //if self.mode != Mode::Home {
                    self.active_popup = Some(Popup::Error);
                    self.render(tui)?;
                    //}
                }
                _ => {}
            }

            // Dispatch to global components
            if let Some(action) = self.header.update(action.clone(), self.mode)? {
                self.action_tx.send(action)?
            };

            // Only the active popup receives actions
            if let Some(popup_type) = self.active_popup.as_ref()
                && let Some(popup) = self.popups.get_mut(popup_type)
                && let Some(action) = popup.update(action.clone(), self.mode)?
            {
                self.action_tx.send(action)?;
            }

            // Only the active mode component receives the action
            if let Some(component) = self.components.get_mut(&self.mode) {
                match component.update(action.clone(), self.mode) {
                    Ok(Some(action)) => self.action_tx.send(action)?,
                    Err(err @ TuiError::JsonError { .. }) => {
                        self.action_tx.send(Action::Error {
                            msg: err.to_string(),
                            action: Some(Box::new(action.clone())),
                        })?
                    }
                    _ => {}
                }
            }
            self.render(tui)?;
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
            let draw_header = f.area().height > 30;
            let rects = Layout::default()
                .constraints(
                    [
                        Constraint::Min(if draw_header { 6 } else { 0 }),
                        Constraint::Percentage(100),
                    ]
                    .as_ref(),
                )
                .split(f.area());
            if draw_header && let Err(e) = self.header.draw(f, rects[0]) {
                self.action_tx
                    .send(Action::Error {
                        msg: format!("Failed to draw: {e:?}"),
                        action: None,
                    })
                    .ok();
            }

            if let Some(component) = self.components.get_mut(&self.mode)
                && let Err(e) = component.draw(f, rects[1])
            {
                error!("Error {:?}", e);
                self.action_tx
                    .send(Action::Error {
                        msg: format!("Failed to draw: {e:?}"),
                        action: None,
                    })
                    .ok();
            }
            if let Some(popup_type) = &self.active_popup
                && let Some(popup) = self.popups.get_mut(popup_type)
                && let Err(e) = popup.draw(f, f.area())
            {
                self.action_tx
                    .send(Action::Error {
                        msg: format!("Failed to draw: {e:?}"),
                        action: None,
                    })
                    .ok();
            }
        })?;
        Ok(())
    }
}
