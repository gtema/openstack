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
    LoadBalancerApiRequest, LoadBalancerPoolApiRequest, LoadBalancerPoolMemberApiRequest,
    LoadBalancerPoolMemberList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::load_balancer::v2::pool::member::response::list::MemberResponse;

const VIEW_CONFIG_KEY: &str = "load-balancer.pool/member";

impl crate::utils::ResourceKey for MemberResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct LoadBalancerPoolMembersBehaviour;

impl ResourceBehaviour for LoadBalancerPoolMembersBehaviour {
    type Item = MemberResponse;
    type Filter = LoadBalancerPoolMemberList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "LB Pool Members"
    }
    fn mode() -> Mode {
        Mode::LoadBalancerPoolMembers
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(LoadBalancerPoolMemberApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(boxreq))
                if matches!(&**boxreq, LoadBalancerPoolApiRequest::Member(inner)
                    if matches!(&**inner, LoadBalancerPoolMemberApiRequest::List(_))
                )
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetLoadBalancerPoolMemberListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}

pub type LoadBalancerPoolMembers = GenericResourceView<'static, LoadBalancerPoolMembersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            LoadBalancerPoolMembersBehaviour::view_key(),
            "load-balancer.pool/member"
        );
        assert_eq!(LoadBalancerPoolMembersBehaviour::title(), "LB Pool Members");
        assert_eq!(
            LoadBalancerPoolMembersBehaviour::mode(),
            Mode::LoadBalancerPoolMembers
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = LoadBalancerPoolMemberList::default();
        let request = LoadBalancerPoolMembersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::LoadBalancer(LoadBalancerApiRequest::Pool(boxreq))
            if matches!(&*boxreq, LoadBalancerPoolApiRequest::Member(inner)
                if matches!(&**inner, LoadBalancerPoolMemberApiRequest::List(_))
            )
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = LoadBalancerPoolMemberList::default();
        let request = LoadBalancerPoolMembersBehaviour::request_from_filter(&filter);
        assert!(LoadBalancerPoolMembersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::LoadBalancer(LoadBalancerApiRequest::Listener(Box::new(
            crate::cloud_worker::load_balancer::v2::LoadBalancerListenerApiRequest::List(
                Box::default(),
            ),
        )));
        assert!(!LoadBalancerPoolMembersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = LoadBalancerPoolMemberList::default();
        let action = Action::SetLoadBalancerPoolMemberListFilters(filter);
        let result = LoadBalancerPoolMembersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = LoadBalancerPoolMembersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
