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
    LoadBalancerApiRequest, LoadBalancerListenerApiRequest, LoadBalancerListenerList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::load_balancer::v2::listener::response::list::ListenerResponse;

const VIEW_CONFIG_KEY: &str = "load-balancer.listener";

impl crate::utils::ResourceKey for ListenerResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct LoadBalancerListenersBehaviour;

impl ResourceBehaviour for LoadBalancerListenersBehaviour {
    type Item = ListenerResponse;
    type Filter = LoadBalancerListenerList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "LB Listeners"
    }
    fn mode() -> Mode {
        Mode::LoadBalancerListeners
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(LoadBalancerListenerApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(boxreq))
            if matches!(**boxreq, LoadBalancerListenerApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetLoadBalancerListenerListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}

pub type LoadBalancerListeners = GenericResourceView<'static, LoadBalancerListenersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            LoadBalancerListenersBehaviour::view_key(),
            "load-balancer.listener"
        );
        assert_eq!(LoadBalancerListenersBehaviour::title(), "LB Listeners");
        assert_eq!(
            LoadBalancerListenersBehaviour::mode(),
            Mode::LoadBalancerListeners
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = LoadBalancerListenerList::default();
        let request = LoadBalancerListenersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(boxreq))
            if matches!(*boxreq, LoadBalancerListenerApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = LoadBalancerListenerList::default();
        let request = LoadBalancerListenersBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerListenersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(Box::new(
            crate::cloud_worker::load_balancer::v2::LoadBalancerPoolApiRequest::List(Box::default()),
        )));
        assert!(!LoadBalancerListenersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = LoadBalancerListenerList::default();
        let action = Action::SetLoadBalancerListenerListFilters(filter);
        let result = LoadBalancerListenersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = LoadBalancerListenersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
