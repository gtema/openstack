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
use crate::cloud_worker::network::v2::{
    NetworkApiRequest, NetworkSubnetApiRequest, NetworkSubnetList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::network::v2::subnet::response::list::SubnetResponse;

const VIEW_CONFIG_KEY: &str = "network.subnet";

impl crate::utils::ResourceKey for SubnetResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct NetworkSubnetsBehaviour;

impl ResourceBehaviour for NetworkSubnetsBehaviour {
    type Item = SubnetResponse;
    type Filter = NetworkSubnetList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Subnets"
    }
    fn mode() -> Mode {
        Mode::NetworkSubnets
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(NetworkSubnetApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Subnet(boxreq))
            if matches!(**boxreq, NetworkSubnetApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetNetworkSubnetListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}

pub type NetworkSubnets = GenericResourceView<'static, NetworkSubnetsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(NetworkSubnetsBehaviour::view_key(), "network.subnet");
        assert_eq!(NetworkSubnetsBehaviour::title(), "Subnets");
        assert_eq!(NetworkSubnetsBehaviour::mode(), Mode::NetworkSubnets);
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = NetworkSubnetList::default();
        let norm = NetworkSubnetsBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = NetworkSubnetList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkSubnetsBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = NetworkSubnetList::default();
        let request = NetworkSubnetsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Subnet(boxreq))
            if matches!(*boxreq, NetworkSubnetApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = NetworkSubnetList::default();
        let request = NetworkSubnetsBehaviour::request_from_filter(&filter);
        assert!(NetworkSubnetsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(NetworkApiRequest::Network(Box::new(
            crate::cloud_worker::types::NetworkNetworkApiRequest::List(Box::default()),
        )));
        assert!(!NetworkSubnetsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = NetworkSubnetList::default();
        let action = Action::SetNetworkSubnetListFilters(filter);
        let result = NetworkSubnetsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = NetworkSubnetsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
