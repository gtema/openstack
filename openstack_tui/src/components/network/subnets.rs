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
use openstack_types::network::v2::subnet::response::list::SubnetResponse;

impl crate::utils::ResourceKey for SubnetResponse {
    fn get_key() -> &'static str {
        crate::mode::NETWORK_SUBNET
    }
}

pub struct NetworkSubnetsBehaviour;

impl ResourceBehaviour for NetworkSubnetsBehaviour {
    type Item = SubnetResponse;
    type Filter = cloud_types::NetworkSubnetList;

    fn view_key() -> &'static str {
        super::generated::subnet::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::subnet::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::subnet::Generated::mode()
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::subnet::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::subnet::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::subnet::Generated::handle_set_filter_action(action)
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
        assert_eq!(
            NetworkSubnetsBehaviour::mode(),
            Mode::Resource(crate::mode::NETWORK_SUBNET)
        );
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = cloud_types::NetworkSubnetList::default();
        let norm = NetworkSubnetsBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = cloud_types::NetworkSubnetList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkSubnetsBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::NetworkSubnetList::default();
        let request = NetworkSubnetsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(cloud_types::NetworkApiRequest::Subnet(boxreq))
            if matches!(*boxreq, cloud_types::NetworkSubnetApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::NetworkSubnetList::default();
        let request = NetworkSubnetsBehaviour::request_from_filter(&filter);
        assert!(NetworkSubnetsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(cloud_types::NetworkApiRequest::Network(Box::new(
            crate::cloud_worker::types::NetworkNetworkApiRequest::List(Box::default()),
        )));
        assert!(!NetworkSubnetsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = cloud_types::NetworkSubnetList::default();
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
