// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use crate::action::Action;
use crate::cloud_worker::compute::v2::{
    ComputeServerApiRequest, ComputeServerDelete, ComputeServerGetConsoleOutputBuilder,
    ComputeServerInstanceActionList, ComputeServerList,
};
use crate::cloud_worker::types::{ApiRequest, ComputeApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::compute::v2::server::response::list_detailed_21::ServerResponse;

/// Behaviour implementation for ComputeServers.
pub struct ComputeServersBehaviour;

impl ResourceBehaviour for ComputeServersBehaviour {
    type Item = ServerResponse;
    type Filter = ComputeServerList;

    fn view_key() -> &'static str {
        "compute.server"
    }
    fn title() -> &'static str {
        "Compute Servers"
    }
    fn mode() -> Mode {
        Mode::ComputeServers
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some("display_name".into());
            filter.sort_dir = Some("asc".into());
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
            ComputeServerApiRequest::ListDetailed(Box::new(filter.clone())),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
            if matches!(**boxreq, ComputeServerApiRequest::ListDetailed(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetComputeServerListFilters(f) = action {
            Some((**f).clone())
        } else {
            None
        }
    }
    fn action_to_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        match action {
            Action::ShowServerConsoleOutput => {
                let server_id = selected?.id.clone();
                let req = ComputeServerApiRequest::GetConsoleOutput(Box::new(
                    ComputeServerGetConsoleOutputBuilder::default()
                        .id(server_id)
                        .os_get_console_output(crate::cloud_worker::compute::v2::server::get_console_output::OsGetConsoleOutputBuilder::default().build().ok()?)
                        .build()
                        .ok()?,
                ));
                Some(ApiRequest::from(req))
            }
            Action::DeleteComputeServer => {
                let del = ComputeServerDelete::try_from(selected?).ok()?;
                Some(ApiRequest::from(ComputeServerApiRequest::Delete(Box::new(
                    del,
                ))))
            }
            _ => None,
        }
    }
    fn custom_action(action: &Action, selected: Option<&Self::Item>) -> Vec<Action> {
        if let Action::ShowComputeServerInstanceActions = action
            && let Some(sel) = selected
        {
            let sel = sel.clone();
            if let Ok(list) = ComputeServerInstanceActionList::try_from(&sel) {
                return vec![
                    Action::SetComputeServerInstanceActionListFilters(Box::new(list)),
                    Action::Mode {
                        mode: Mode::ComputeServerInstanceActions,
                        stack: true,
                    },
                ];
            }
        }
        Vec::new()
    }
}

/// Public component for ComputeServers using the generic view.
pub type ComputeServers = GenericResourceView<'static, ComputeServersBehaviour>;
