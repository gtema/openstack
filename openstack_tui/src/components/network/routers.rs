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
use openstack_types::network::v2::router::response::list::RouterResponse;

impl crate::utils::ResourceKey for RouterResponse {
    fn get_key() -> &'static str {
        crate::mode::NETWORK_ROUTER
    }
}

pub struct NetworkRoutersBehaviour;

impl ResourceBehaviour for NetworkRoutersBehaviour {
    type Item = RouterResponse;
    type Filter = cloud_types::NetworkRouterList;

    fn view_key() -> &'static str {
        super::generated::router::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::router::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::router::Generated::mode()
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::router::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::router::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::router::Generated::handle_set_filter_action(action)
    }
}

pub type NetworkRouters = GenericResourceView<'static, NetworkRoutersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(NetworkRoutersBehaviour::view_key(), "network.router");
        assert_eq!(NetworkRoutersBehaviour::title(), "Routers");
        assert_eq!(
            NetworkRoutersBehaviour::mode(),
            Mode::Resource(crate::mode::NETWORK_ROUTER)
        );
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = cloud_types::NetworkRouterList::default();
        let norm = NetworkRoutersBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = cloud_types::NetworkRouterList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkRoutersBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::NetworkRouterList::default();
        let request = NetworkRoutersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(cloud_types::NetworkApiRequest::Router(boxreq))
            if matches!(*boxreq, cloud_types::NetworkRouterApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::NetworkRouterList::default();
        let request = NetworkRoutersBehaviour::request_from_filter(&filter);
        assert!(NetworkRoutersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(cloud_types::NetworkApiRequest::Subnet(Box::new(
            crate::cloud_worker::network::v2::NetworkSubnetApiRequest::List(Box::default()),
        )));
        assert!(!NetworkRoutersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = cloud_types::NetworkRouterList::default();
        let action = Action::SetNetworkRouterListFilters(filter);
        let result = NetworkRoutersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = NetworkRoutersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
