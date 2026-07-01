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
use crate::cloud_worker::load_balancer::v2::{
    LoadBalancerApiRequest, LoadBalancerHealthmonitorList, LoadBalancerHealthmonitorListBuilder,
    LoadBalancerPoolApiRequest, LoadBalancerPoolList, LoadBalancerPoolMemberList,
    LoadBalancerPoolMemberListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::load_balancer::v2::pool::response::list::PoolResponse;

const VIEW_CONFIG_KEY: &str = "load-balancer.pool";

impl crate::utils::ResourceKey for PoolResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&PoolResponse> for LoadBalancerPoolMemberList {
    type Error = crate::cloud_worker::load_balancer::v2::LoadBalancerPoolMemberListBuilderError;
    fn try_from(value: &PoolResponse) -> Result<Self, Self::Error> {
        let mut builder = LoadBalancerPoolMemberListBuilder::default();
        if let Some(val) = &value.id {
            builder.pool_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.pool_name(val.clone());
        }
        builder.build()
    }
}

impl TryFrom<&PoolResponse> for LoadBalancerHealthmonitorList {
    type Error = crate::cloud_worker::load_balancer::v2::LoadBalancerHealthmonitorListBuilderError;
    fn try_from(value: &PoolResponse) -> Result<Self, Self::Error> {
        let mut builder = LoadBalancerHealthmonitorListBuilder::default();
        if let Some(val) = &value.id {
            builder.pool_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.pool_name(val.clone());
        }
        builder.build()
    }
}

pub struct LoadBalancerPoolsBehaviour;

impl ResourceBehaviour for LoadBalancerPoolsBehaviour {
    type Item = PoolResponse;
    type Filter = LoadBalancerPoolList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "LB Pools"
    }
    fn mode() -> Mode {
        Mode::LoadBalancerPools
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(LoadBalancerPoolApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(boxreq))
            if matches!(**boxreq, LoadBalancerPoolApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetLoadBalancerPoolListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowLoadBalancerPoolMembers = action
            && let Some(sel) = selected
            && let Ok(list) = LoadBalancerPoolMemberList::try_from(sel)
        {
            return vec![
                Action::SetLoadBalancerPoolMemberListFilters(list),
                Action::Mode {
                    mode: Mode::LoadBalancerPoolMembers,
                    stack: true,
                },
            ];
        }
        if let Action::ShowLoadBalancerPoolHealthMonitors = action
            && let Some(sel) = selected
            && let Ok(list) = LoadBalancerHealthmonitorList::try_from(sel)
        {
            return vec![
                Action::SetLoadBalancerHealthMonitorListFilters(list),
                Action::Mode {
                    mode: Mode::LoadBalancerHealthMonitors,
                    stack: true,
                },
            ];
        }
        Vec::new()
    }
}

pub type LoadBalancerPools = GenericResourceView<'static, LoadBalancerPoolsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::load_balancer::v2::pool::response::list::PoolResponse;

    fn make_pool(id: &str, name: &str) -> PoolResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "description": "test pool",
            "lb_algorithm": "ROUND_ROBIN",
            "protocol": "HTTP",
            "provisioning_status": "ACTIVE",
            "operating_status": "ONLINE",
            "created_at": "2024-01-01T00:00:00",
            "updated_at": "2024-01-01T00:00:00",
            "project_id": "tenant-1",
            "loadbalancers": [],
            "listeners": [],
            "members": [],
            "health_monitors": [],
            "session_persistence": null,
            "monitor_ports": [],
            "tags": []
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(LoadBalancerPoolsBehaviour::view_key(), "load-balancer.pool");
        assert_eq!(LoadBalancerPoolsBehaviour::title(), "LB Pools");
        assert_eq!(LoadBalancerPoolsBehaviour::mode(), Mode::LoadBalancerPools);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = LoadBalancerPoolList::default();
        let request = LoadBalancerPoolsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(boxreq))
            if matches!(*boxreq, LoadBalancerPoolApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = LoadBalancerPoolList::default();
        let request = LoadBalancerPoolsBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerPoolsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(Box::new(
            crate::cloud_worker::load_balancer::v2::LoadBalancerListenerApiRequest::List(Box::new(
                crate::cloud_worker::load_balancer::v2::LoadBalancerListenerList::default(),
            )),
        )));
        assert!(!LoadBalancerPoolsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = LoadBalancerPoolList::default();
        let action = Action::SetLoadBalancerPoolListFilters(filter);
        let result = LoadBalancerPoolsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = LoadBalancerPoolsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_members_with_selected() {
        let pool = make_pool("pool-1", "test-pool");
        let actions = LoadBalancerPoolsBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerPoolMembers,
            Some(&pool),
            &LoadBalancerPoolList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetLoadBalancerPoolMemberListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::LoadBalancerPoolMembers,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_show_health_monitors_with_selected() {
        let pool = make_pool("pool-1", "test-pool");
        let actions = LoadBalancerPoolsBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerPoolHealthMonitors,
            Some(&pool),
            &LoadBalancerPoolList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetLoadBalancerHealthMonitorListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::LoadBalancerHealthMonitors,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = LoadBalancerPoolsBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerPoolMembers,
            None,
            &LoadBalancerPoolList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let pool = make_pool("pool-1", "test-pool");
        let actions = LoadBalancerPoolsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&pool),
            &LoadBalancerPoolList::default(),
        );
        assert!(actions.is_empty());
    }
}
