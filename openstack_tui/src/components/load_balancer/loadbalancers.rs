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
    LoadBalancerApiRequest, LoadBalancerListenerList, LoadBalancerListenerListBuilder,
    LoadBalancerLoadbalancerApiRequest, LoadBalancerLoadbalancerList, LoadBalancerPoolList,
    LoadBalancerPoolListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::load_balancer::v2::loadbalancer::response::list::LoadbalancerResponse;

const VIEW_CONFIG_KEY: &str = "load-balancer.loadbalancer";

impl crate::utils::ResourceKey for LoadbalancerResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&LoadbalancerResponse> for LoadBalancerListenerList {
    type Error = crate::cloud_worker::load_balancer::v2::LoadBalancerListenerListBuilderError;
    fn try_from(value: &LoadbalancerResponse) -> Result<Self, Self::Error> {
        let mut builder = LoadBalancerListenerListBuilder::default();
        if let Some(val) = &value.id {
            builder.load_balancer_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.load_balancer_name(val.clone());
        }
        builder.build()
    }
}

impl TryFrom<&LoadbalancerResponse> for LoadBalancerPoolList {
    type Error = crate::cloud_worker::load_balancer::v2::LoadBalancerPoolListBuilderError;
    fn try_from(value: &LoadbalancerResponse) -> Result<Self, Self::Error> {
        let mut builder = LoadBalancerPoolListBuilder::default();
        if let Some(val) = &value.id {
            builder.loadbalancer_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.loadbalancer_name(val.clone());
        }
        builder.build()
    }
}

pub struct LoadBalancersBehaviour;

impl ResourceBehaviour for LoadBalancersBehaviour {
    type Item = LoadbalancerResponse;
    type Filter = LoadBalancerLoadbalancerList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "LoadBalancers"
    }
    fn mode() -> Mode {
        Mode::LoadBalancers
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(LoadBalancerLoadbalancerApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Loadbalancer(boxreq))
            if matches!(**boxreq, LoadBalancerLoadbalancerApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetLoadBalancerListFilters(f) = action {
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
        if let Action::ShowLoadBalancerListeners = action
            && let Some(sel) = selected
            && let Ok(list) = LoadBalancerListenerList::try_from(sel)
        {
            return vec![
                Action::SetLoadBalancerListenerListFilters(list),
                Action::Mode {
                    mode: Mode::LoadBalancerListeners,
                    stack: true,
                },
            ];
        }
        if let Action::ShowLoadBalancerPools = action
            && let Some(sel) = selected
            && let Ok(list) = LoadBalancerPoolList::try_from(sel)
        {
            return vec![
                Action::SetLoadBalancerPoolListFilters(list),
                Action::Mode {
                    mode: Mode::LoadBalancerPools,
                    stack: true,
                },
            ];
        }
        Vec::new()
    }
}

pub type LoadBalancers = GenericResourceView<'static, LoadBalancersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::load_balancer::v2::loadbalancer::response::list::LoadbalancerResponse;

    fn make_lb(id: &str, name: &str) -> LoadbalancerResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "provisioning_status": "ACTIVE",
            "operating_status": "ONLINE",
            "description": "test lb",
            "provider": "haproxy",
            "created_at": "2024-01-01T00:00:00",
            "updated_at": "2024-01-01T00:00:00",
            "listeners": [],
            "pools": [],
            "vip_address": "10.0.0.1",
            "vip_port_id": "port-1",
            "vip_subnet_id": "subnet-1",
            "project_id": "tenant-1",
            "flavor_id": null,
            "tags": []
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            LoadBalancersBehaviour::view_key(),
            "load-balancer.loadbalancer"
        );
        assert_eq!(LoadBalancersBehaviour::title(), "LoadBalancers");
        assert_eq!(LoadBalancersBehaviour::mode(), Mode::LoadBalancers);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = LoadBalancerLoadbalancerList::default();
        let request = LoadBalancersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Loadbalancer(boxreq))
            if matches!(*boxreq, LoadBalancerLoadbalancerApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = LoadBalancerLoadbalancerList::default();
        let request = LoadBalancersBehaviour::request_from_filter(&filter);
        assert!(LoadBalancersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(Box::new(
            crate::cloud_worker::load_balancer::v2::LoadBalancerListenerApiRequest::List(Box::new(
                crate::cloud_worker::load_balancer::v2::LoadBalancerListenerList::default(),
            )),
        )));
        assert!(!LoadBalancersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = LoadBalancerLoadbalancerList::default();
        let action = Action::SetLoadBalancerListFilters(filter);
        let result = LoadBalancersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = LoadBalancersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_listeners_with_selected() {
        let lb = make_lb("lb-1", "test-lb");
        let actions = LoadBalancersBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerListeners,
            Some(&lb),
            &LoadBalancerLoadbalancerList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetLoadBalancerListenerListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::LoadBalancerListeners,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_show_pools_with_selected() {
        let lb = make_lb("lb-1", "test-lb");
        let actions = LoadBalancersBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerPools,
            Some(&lb),
            &LoadBalancerLoadbalancerList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetLoadBalancerPoolListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::LoadBalancerPools,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = LoadBalancersBehaviour::filter_carry_action(
            &Action::ShowLoadBalancerListeners,
            None,
            &LoadBalancerLoadbalancerList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let lb = make_lb("lb-1", "test-lb");
        let actions = LoadBalancersBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&lb),
            &LoadBalancerLoadbalancerList::default(),
        );
        assert!(actions.is_empty());
    }
}
