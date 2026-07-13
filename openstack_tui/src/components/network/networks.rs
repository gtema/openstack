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
    NetworkApiRequest, NetworkNetworkApiRequest, NetworkNetworkList, NetworkSubnetList,
    NetworkSubnetListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::network::v2::network::response::list::NetworkResponse;

const VIEW_CONFIG_KEY: &str = "network.network";

impl crate::utils::ResourceKey for NetworkResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&NetworkResponse> for NetworkSubnetList {
    type Error = crate::cloud_worker::network::v2::NetworkSubnetListBuilderError;
    fn try_from(value: &NetworkResponse) -> Result<Self, Self::Error> {
        let mut builder = NetworkSubnetListBuilder::default();
        if let Some(val) = &value.id {
            builder.network_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.network_name(val.clone());
        }
        builder.build()
    }
}

pub struct NetworkNetworksBehaviour;

impl ResourceBehaviour for NetworkNetworksBehaviour {
    type Item = NetworkResponse;
    type Filter = NetworkNetworkList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Networks"
    }
    fn mode() -> Mode {
        Mode::NetworkNetworks
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(NetworkNetworkApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Network(boxreq))
            if matches!(**boxreq, NetworkNetworkApiRequest::List(_))
        )
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowNetworkSubnets = action
            && let Some(sel) = selected
            && let Ok(list) = NetworkSubnetList::try_from(sel)
        {
            return vec![
                Action::Mode {
                    mode: Mode::NetworkSubnets,
                    stack: true,
                },
                Action::SetNetworkSubnetListFilters(list),
            ];
        }
        Vec::new()
    }
}

pub type NetworkNetworks = GenericResourceView<'static, NetworkNetworksBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::network::v2::network::response::list::NetworkResponse;

    fn make_network(id: &str, name: &str) -> NetworkResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "tenant_id": "tenant1",
            "subnets": [],
            "network_type": "vxlan",
            "segments": [],
            "status": "ACTIVE",
            "admin_state_up": true,
            "shared": false
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(NetworkNetworksBehaviour::view_key(), "network.network");
        assert_eq!(NetworkNetworksBehaviour::title(), "Networks");
        assert_eq!(NetworkNetworksBehaviour::mode(), Mode::NetworkNetworks);
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = NetworkNetworkList::default();
        let norm = NetworkNetworksBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = NetworkNetworkList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkNetworksBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = NetworkNetworkList::default();
        let request = NetworkNetworksBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::Network(boxreq))
            if matches!(*boxreq, NetworkNetworkApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = NetworkNetworkList::default();
        let request = NetworkNetworksBehaviour::request_from_filter(&filter);
        assert!(NetworkNetworksBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(NetworkApiRequest::Subnet(Box::new(
            crate::cloud_worker::network::v2::NetworkSubnetApiRequest::List(Box::default()),
        )));
        assert!(!NetworkNetworksBehaviour::matches_request(&req));
    }

    #[test]
    fn filter_carry_action_show_subnets_with_selected() {
        let net = make_network("net-1", "test-net");
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::ShowNetworkSubnets,
            Some(&net),
            &NetworkNetworkList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::NetworkSubnets,
                stack: true
            }
        ));
        assert!(matches!(actions[1], Action::SetNetworkSubnetListFilters(_)));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::ShowNetworkSubnets,
            None,
            &NetworkNetworkList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let net = make_network("net-1", "test-net");
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&net),
            &NetworkNetworkList::default(),
        );
        assert!(actions.is_empty());
    }
}
