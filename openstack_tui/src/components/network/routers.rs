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

use crate::cloud_worker::types::{
    ApiRequest, NetworkApiRequest, NetworkRouterApiRequest, NetworkRouterList,
};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::network::v2::router::response::list::RouterResponse;

const VIEW_CONFIG_KEY: &str = "network.router";

impl crate::utils::ResourceKey for RouterResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct NetworkRoutersBehaviour;

impl ResourceBehaviour for NetworkRoutersBehaviour {
    type Item = RouterResponse;
    type Filter = NetworkRouterList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Routers"
    }
    fn mode() -> Mode {
        Mode::NetworkRouters
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(NetworkRouterApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Router(boxreq))
            if matches!(**boxreq, NetworkRouterApiRequest::List(_))
        )
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
        assert_eq!(NetworkRoutersBehaviour::mode(), Mode::NetworkRouters);
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = NetworkRouterList::default();
        let norm = NetworkRoutersBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = NetworkRouterList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkRoutersBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = NetworkRouterList::default();
        let request = NetworkRoutersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Router(boxreq))
            if matches!(*boxreq, NetworkRouterApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = NetworkRouterList::default();
        let request = NetworkRoutersBehaviour::request_from_filter(&filter);
        assert!(NetworkRoutersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(NetworkApiRequest::Subnet(Box::new(
            crate::cloud_worker::network::v2::NetworkSubnetApiRequest::List(Box::new(
                crate::cloud_worker::network::v2::NetworkSubnetList::default(),
            )),
        )));
        assert!(!NetworkRoutersBehaviour::matches_request(&req));
    }
}
