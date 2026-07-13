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

        // --- Region switch: reload data for the new region ---
        if let Action::SwitchToRegion(_) = &action {
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
            // --- Singular API response: delegate to behaviour ---
            if let Some(action) = B::handle_singular_response_data(request, data) {
                return Ok(Some(action));
            }
            return Ok(None);
        }

        // --- ApiResponseData (singular): delegate mutation response handling ---
        if let Action::ApiResponseData { request, data } = &action {
            if let Some(mutations) = B::handle_mutation_response(request, data) {
                for mutation in mutations {
                    match mutation {
                        super::resource_behaviour::Mutation::DeleteRow(id) => {
                            self.base.delete_item_row_by_res_id_mut(&id).ok();
                        }
                        super::resource_behaviour::Mutation::UpdateRow(_id, row_data) => {
                            self.base.update_row_data(row_data)?;
                        }
                        super::resource_behaviour::Mutation::AppendRow(row_data) => {
                            self.base.append_new_row(row_data)?;
                        }
                        super::resource_behaviour::Mutation::Refresh => {
                            return Ok(Some(Action::PerformApiRequest(B::request_from_filter(
                                self.base.get_filters(),
                            ))));
                        }
                    }
                }
                return Ok(None);
            }
            // Check singular request match for describe/data responses
            if B::matches_singular_request(request)
                && let Some(ret_action) =
                    B::handle_singular_response_data(request, std::slice::from_ref(data))
            {
                return Ok(Some(ret_action));
            }
            return Ok(None);
        }

        // --- Action-to-request: resource-specific actions that produce API requests ---
        if let Some(api_request) = B::action_to_request(&action, self.base.get_selected()) {
            return Ok(Some(Action::PerformApiRequest(api_request)));
        }

        // --- Editor create: open editor with YAML template ---
        if let Some((template, api_request)) = B::editor_template(&action, self.base.get_filters())
        {
            return Ok(Some(Action::Edit {
                template,
                original_action: Box::new(Action::PerformApiRequest(api_request)),
            }));
        }

        // --- Editor result: process YAML back into confirm request ---
        if let Action::EditResult {
            result,
            original_action: _,
        } = &action
        {
            if let Some(api_request) = B::deserialize_edit_result(result) {
                self.base.set_loading(true);
                return Ok(Some(Action::Confirm(api_request)));
            }
            return Ok(None);
        }

        // --- Action-to-confirm: actions wrapped in Confirm for confirmation dialog ---
        if let Some(api_request) = B::confirm_request(&action, self.base.get_selected()) {
            return Ok(Some(Action::Confirm(api_request)));
        }

        // --- Singular action-to-request: actions with describe display + API request ---
        if let Some((display_actions, api_request)) =
            B::action_to_singular_request(&action, self.base.get_selected())
        {
            if let Some(tx) = self.base.get_command_tx() {
                for a in display_actions {
                    tx.send(a)?;
                }
            }
            return Ok(Some(Action::PerformApiRequest(api_request)));
        }

        // --- Filter-carry custom actions (with filter access) ---
        if let Some((last, rest)) =
            B::filter_carry_action(&action, self.base.get_selected(), self.base.get_filters())
                .split_last()
        {
            if let Some(tx) = self.base.get_command_tx() {
                for a in rest {
                    tx.send(a.clone())?;
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::{ComputeServerApiRequest, ComputeServerList};
    use crate::cloud_worker::types::{ApiRequest, ComputeApiRequest};
    use crate::components::compute::servers::ComputeServers;
    use crate::mode::Mode;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use serde_json::Value;

    #[test]
    fn tick_returns_none() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let result = comp.update(Action::Tick, Mode::ComputeServers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn render_returns_none() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let result = comp.update(Action::Render, Mode::ComputeServers);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn refresh_returns_perform_api_request() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let result = comp.update(Action::Refresh, Mode::ComputeServers);
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::PerformApiRequest(_))
        ));
    }

    #[test]
    fn mode_switch_sets_loading_when_switching_to_match() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let result = comp.update(
            Action::Mode {
                mode: Mode::ComputeServers,
                stack: false,
            },
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn mode_switch_returns_none_for_other_mode() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let result = comp.update(
            Action::Mode {
                mode: Mode::ComputeFlavors,
                stack: false,
            },
            Mode::ComputeFlavors,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn filter_change_requests_data() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let filter = ComputeServerList {
            sort_key: Some("name".into()),
            ..Default::default()
        };
        let action = Action::SetComputeServerListFilters(Box::new(filter));
        let result = comp.update(action, Mode::ComputeServers);
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::PerformApiRequest(_))
        ));
    }

    #[test]
    fn api_responses_data_sets_data_when_request_matches() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let filter = ComputeServerList::default();
        let request = ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
            ComputeServerApiRequest::ListDetailed(Box::new(filter.clone())),
        )));
        let data: Vec<Value> = Vec::new();
        let result = comp.update(
            Action::ApiResponsesData { request, data },
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn api_responses_data_returns_none_when_no_match() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let request = ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
            ComputeServerApiRequest::Delete(Box::new(
                crate::cloud_worker::compute::v2::ComputeServerDelete {
                    id: "test".into(),
                    name: None,
                },
            )),
        )));
        let data: Vec<Value> = Vec::new();
        let result = comp.update(
            Action::ApiResponsesData { request, data },
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn cloud_change_scope_returns_none() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let scope = openstack_sdk::auth::authtoken::AuthTokenScope::Unscoped;
        let result = comp.update(
            Action::CloudChangeScope(Box::new(scope)),
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn connected_to_cloud_requests_data_when_mode_matches() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        let result = comp.update(
            Action::ConnectedToCloud(Box::new(token_info)),
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::PerformApiRequest(_))
        ));
    }

    #[test]
    fn connected_to_cloud_returns_none_when_mode_does_not_match() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        let result = comp.update(
            Action::ConnectedToCloud(Box::new(token_info)),
            Mode::ComputeFlavors,
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    fn make_server_json() -> Value {
        serde_json::json!({
            "id": "server-1",
            "name": "test-server",
            "status": "ACTIVE",
            "tenant_id": "tenant1",
            "user_id": "user1",
            "created": "2024-01-01T00:00:00Z",
            "updated": "2024-01-01T00:00:00Z",
            "accessIPv4": "",
            "accessIPv6": "",
            "flavor": {"id": "flavor1"},
            "image": "image1",
            "OS-DCF:diskConfig": "AUTO",
            "OS-EXT-AZ:availability_zone": "nova",
            "OS-EXT-STS:power_state": 1,
            "os-extended-volumes:volumes_attached": [],
            "metadata": {},
            "addresses": {},
            "config_drive": "",
            "hostId": "host1",
            "key_name": null,
            "security_groups": []
        })
    }

    fn setup_comp_with_selected_server() -> ComputeServers {
        let mut comp: ComputeServers = GenericResourceView::new();
        let data = vec![make_server_json()];
        let request = ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
            ComputeServerApiRequest::ListDetailed(Box::default()),
        )));
        comp.update(
            Action::ApiResponsesData { request, data },
            Mode::ComputeServers,
        )
        .unwrap();
        comp.handle_key_events(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
            .unwrap();
        comp
    }

    fn setup_comp_with_matching_singular_request_and_data() -> (Value, ApiRequest) {
        let server: openstack_types::compute::v2::server::response::list_detailed_21::ServerResponse =
            serde_json::from_value(make_server_json()).unwrap();
        let (_display_actions, request) =
            crate::components::compute::servers::ComputeServersBehaviour::action_to_singular_request(
                &Action::ShowServerConsoleOutput,
                Some(&server),
            )
            .unwrap();
        (serde_json::json!({ "output": "test console" }), request)
    }

    #[test]
    fn api_responses_data_singular_request_dispatches_action() {
        let mut comp: ComputeServers = GenericResourceView::new();
        let (console_data, request) = setup_comp_with_matching_singular_request_and_data();
        let data = vec![console_data];
        let result = comp.update(
            Action::ApiResponsesData { request, data },
            Mode::ComputeServers,
        );
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::SetDescribeApiResponseData(_))
        ));
    }

    #[test]
    fn confirm_request_dispatches_confirm_action() {
        let mut comp = setup_comp_with_selected_server();
        let result = comp.update(Action::DeleteComputeServer, Mode::ComputeServers);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Some(Action::Confirm(_))));
    }

    #[test]
    fn confirm_request_dispatches_confirm_action_block_storage() {
        use crate::cloud_worker::block_storage::v3::{
            BlockStorageApiRequest, BlockStorageVolumeApiRequest,
        };
        use crate::components::block_storage::volumes::BlockStorageVolumes;

        let mut comp: BlockStorageVolumes = GenericResourceView::new();
        let volume_json = serde_json::json!({
            "id": "vol-1",
            "name": "test-vol",
            "status": "available",
            "size": 10,
            "user_id": "user-1",
            "availability_zone": "nova",
            "created_at": "2024-01-01T00:00:00Z",
            "volume_type": "lvmdriver-1",
            "attachments": [],
            "description": null,
            "snapshot_id": null,
            "source_volid": null,
            "bootable": "true",
            "replicas": [],
            "encrypted": false,
            "consistencygroup_id": null,
            "os-vol-mig-Status.migration_status": null,
            "os-vol-host-attr.host": null,
            "os-vol-tenant-attr.tenant_id": "tenant-1",
            "os-vol-mig-status.migration_status": null,
            "os-vol-host-attr:host": null,
            "os-vol-tenant-attr:tenant_id": "tenant-1"
        });
        let request = ApiRequest::BlockStorage(BlockStorageApiRequest::Volume(Box::new(
            BlockStorageVolumeApiRequest::ListDetailed(Box::default()),
        )));
        comp.update(
            Action::ApiResponsesData {
                request,
                data: vec![volume_json],
            },
            Mode::BlockStorageVolumes,
        )
        .unwrap();
        comp.handle_key_events(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
            .unwrap();
        let result = comp.update(Action::DeleteBlockStorageVolume, Mode::BlockStorageVolumes);
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), Some(Action::Confirm(_))));
    }

    #[test]
    fn action_to_singular_request_dispatches_via_tx_and_returns_perform() {
        let mut comp = setup_comp_with_selected_server();
        let result = comp.update(Action::ShowServerConsoleOutput, Mode::ComputeServers);
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::PerformApiRequest(_))
        ));
    }

    #[test]
    fn filter_carry_action_dispatches_last_action() {
        use crate::cloud_worker::compute::v2::{ComputeApiRequest, ComputeFlavorApiRequest};
        use crate::components::compute::flavors::ComputeFlavors;

        let mut comp: ComputeFlavors = GenericResourceView::new();
        let flavor_json = serde_json::json!({
            "id": "flavor-1",
            "name": "test",
            "vcpus": 1,
            "ram": 512,
            "disk": 10,
            "OS-FLV-DISABLED:disabled": false,
            "rxtx_factor": 1,
            "swap": 0,
            "OS-FLV-EXT-DATA:ephemeral": 0,
            "metadata": {},
            "os-flavor-access:is_public": true
        });
        let request = ApiRequest::Compute(ComputeApiRequest::Flavor(Box::new(
            ComputeFlavorApiRequest::ListDetailed(Box::default()),
        )));
        comp.update(
            Action::ApiResponsesData {
                request,
                data: vec![flavor_json],
            },
            Mode::ComputeFlavors,
        )
        .unwrap();
        comp.handle_key_events(KeyEvent::new(KeyCode::Down, KeyModifiers::empty()))
            .unwrap();
        let result = comp.update(Action::ShowComputeServersWithFlavor, Mode::ComputeFlavors);
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            Some(Action::SetComputeServerListFilters(_))
        ));
    }
}
