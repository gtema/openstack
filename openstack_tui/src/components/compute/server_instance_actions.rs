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
use crate::cloud_worker::compute::v2::{
    ComputeApiRequest, ComputeServerApiRequest, ComputeServerInstanceActionApiRequest,
    ComputeServerInstanceActionList, ComputeServerInstanceActionShowBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::compute::v2::server::instance_action::response::list_21::InstanceActionResponse;

const VIEW_CONFIG_KEY: &str = "compute.server/instance_action";

impl crate::utils::ResourceKey for InstanceActionResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct ComputeServerInstanceActionsBehaviour;

impl ResourceBehaviour for ComputeServerInstanceActionsBehaviour {
    type Item = InstanceActionResponse;
    type Filter = ComputeServerInstanceActionList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "ServerInstanceAction Actions"
    }
    fn mode() -> Mode {
        Mode::ComputeServerInstanceActions
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ComputeServerInstanceActionApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
                if matches!(&**boxreq, ComputeServerApiRequest::InstanceAction(inner)
                    if matches!(&**inner, ComputeServerInstanceActionApiRequest::List(_))
                )
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetComputeServerInstanceActionListFilters(f) = action {
            Some((**f).clone())
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowComputeServerInstanceActionEvents = action
            && let Some(sel) = selected
        {
            let mut req = ComputeServerInstanceActionShowBuilder::default();
            req.id(sel.request_id.clone());
            req.server_id(sel.instance_uuid.clone());
            if let Some(name) = &filter.server_name {
                req.server_name(name.clone());
            }
            if let Ok(list) = req.build() {
                return vec![
                    Action::Mode {
                        mode: Mode::ComputeServerInstanceActionEvents,
                        stack: true,
                    },
                    Action::SetComputeServerInstanceActionShowFilters(Box::new(list)),
                ];
            }
        }
        Vec::new()
    }
}

pub type ComputeServerInstanceActions =
    GenericResourceView<'static, ComputeServerInstanceActionsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::compute::v2::server::instance_action::response::list_21::InstanceActionResponse;

    fn make_instance_action() -> InstanceActionResponse {
        let json = serde_json::json!({
            "request_id": "req-1",
            "instance_uuid": "server-1",
            "server_name": "test-server",
            "created_at": "2024-01-01T00:00:00",
            "user_id": "user-1",
            "body": {},
            "result": {},
            "method": "POST",
            "status_code": 202,
            "admin_password_required": false,
            "action": "boot_server",
            "start_time": "2024-01-01T00:00:00",
            "end_time": "2024-01-01T00:00:01"
        });
        serde_json::from_value(json).unwrap()
    }

    fn make_filter() -> ComputeServerInstanceActionList {
        let mut f = ComputeServerInstanceActionList::default();
        f.server_id = "server-1".into();
        f
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            ComputeServerInstanceActionsBehaviour::view_key(),
            "compute.server/instance_action"
        );
        assert_eq!(
            ComputeServerInstanceActionsBehaviour::title(),
            "ServerInstanceAction Actions"
        );
        assert_eq!(
            ComputeServerInstanceActionsBehaviour::mode(),
            Mode::ComputeServerInstanceActions
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Server(boxreq))
            if matches!(&*boxreq, ComputeServerApiRequest::InstanceAction(inner)
                if matches!(&**inner, ComputeServerInstanceActionApiRequest::List(_))
            )
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = make_filter();
        let request = ComputeServerInstanceActionsBehaviour::request_from_filter(&filter);
        assert!(ComputeServerInstanceActionsBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Compute(ComputeApiRequest::Hypervisor(Box::new(
            crate::cloud_worker::compute::v2::ComputeHypervisorApiRequest::ListDetailed(Box::new(
                crate::cloud_worker::compute::v2::ComputeHypervisorList::default(),
            )),
        )));
        assert!(!ComputeServerInstanceActionsBehaviour::matches_request(
            &req
        ));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = make_filter();
        let action = Action::SetComputeServerInstanceActionListFilters(Box::new(filter));
        let result = ComputeServerInstanceActionsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = ComputeServerInstanceActionsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_events_with_selected() {
        let action_item = make_instance_action();
        let filter = make_filter();
        let actions = ComputeServerInstanceActionsBehaviour::filter_carry_action(
            &Action::ShowComputeServerInstanceActionEvents,
            Some(&action_item),
            &filter,
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::ComputeServerInstanceActionEvents,
                stack: true
            }
        ));
        assert!(matches!(
            actions[1],
            Action::SetComputeServerInstanceActionShowFilters(_)
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = ComputeServerInstanceActionsBehaviour::filter_carry_action(
            &Action::ShowComputeServerInstanceActionEvents,
            None,
            &make_filter(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let action_item = make_instance_action();
        let actions = ComputeServerInstanceActionsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&action_item),
            &make_filter(),
        );
        assert!(actions.is_empty());
    }
}
