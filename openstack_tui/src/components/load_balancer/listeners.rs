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
use crate::cloud_worker::types::{self as cloud_types, ApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

pub struct LoadBalancerListenersBehaviour;

impl ResourceBehaviour for LoadBalancerListenersBehaviour {
    type Filter = cloud_types::LoadBalancerListenerList;

    fn view_key() -> &'static str {
        super::generated::listener::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::listener::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::listener::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::listener::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::listener::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::listener::Generated::handle_set_filter_action(action)
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
        assert_eq!(LoadBalancerListenersBehaviour::title(), "Listeners");
        assert_eq!(
            LoadBalancerListenersBehaviour::mode(),
            Mode::Resource(crate::mode::LB_LISTENER)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::LoadBalancerListenerList::default();
        let request = LoadBalancerListenersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(cloud_types::LoadBalancerApiRequest::Listener(boxreq))
            if matches!(*boxreq, cloud_types::LoadBalancerListenerApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::LoadBalancerListenerList::default();
        let request = LoadBalancerListenersBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerListenersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(cloud_types::LoadBalancerApiRequest::Pool(Box::new(
            cloud_types::LoadBalancerPoolApiRequest::List(Box::default()),
        )));
        assert!(!LoadBalancerListenersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = cloud_types::LoadBalancerListenerList::default();
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
