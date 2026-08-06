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
use crate::cloud_worker::network::v2::{NetworkSubnetList, NetworkSubnetListBuilder};
use crate::cloud_worker::types::{self as cloud_types, ApiRequest};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

impl TryFrom<&serde_json::Value> for NetworkSubnetList {
    type Error = crate::cloud_worker::network::v2::NetworkSubnetListBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder = NetworkSubnetListBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.network_id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.network_name(val.to_string());
        }
        builder.build()
    }
}

pub struct NetworkNetworksBehaviour;

impl ResourceBehaviour for NetworkNetworksBehaviour {
    type Filter = cloud_types::NetworkNetworkList;

    fn view_key() -> &'static str {
        super::generated::network::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::network::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::network::Generated::mode()
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::network::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::network::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::network::Generated::handle_set_filter_action(action)
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&serde_json::Value>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowResource(key) = action
            && *key == crate::mode::NETWORK_SUBNET
            && let Some(sel) = selected
            && let Ok(list) = NetworkSubnetList::try_from(sel)
        {
            return vec![
                Action::Mode {
                    mode: Mode::Resource(crate::mode::NETWORK_SUBNET),
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

    fn make_network(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "tenant_id": "tenant1",
            "subnets": [],
            "network_type": "vxlan",
            "segments": [],
            "status": "ACTIVE",
            "admin_state_up": true,
            "shared": false
        })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(NetworkNetworksBehaviour::view_key(), "network.network");
        assert_eq!(NetworkNetworksBehaviour::title(), "Networks");
        assert_eq!(
            NetworkNetworksBehaviour::mode(),
            Mode::Resource(crate::mode::NETWORK_NETWORK)
        );
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = cloud_types::NetworkNetworkList::default();
        let norm = NetworkNetworksBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = cloud_types::NetworkNetworkList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkNetworksBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::NetworkNetworkList::default();
        let request = NetworkNetworksBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(cloud_types::NetworkApiRequest::Network(boxreq))
            if matches!(*boxreq, cloud_types::NetworkNetworkApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::NetworkNetworkList::default();
        let request = NetworkNetworksBehaviour::request_from_filter(&filter);
        assert!(NetworkNetworksBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(cloud_types::NetworkApiRequest::Subnet(Box::new(
            crate::cloud_worker::network::v2::NetworkSubnetApiRequest::List(Box::default()),
        )));
        assert!(!NetworkNetworksBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = cloud_types::NetworkNetworkList::default();
        let action = Action::SetNetworkNetworkListFilters(filter);
        let result = NetworkNetworksBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = NetworkNetworksBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_subnets_with_selected() {
        let net = make_network("net-1", "test-net");
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_SUBNET),
            Some(&net),
            &cloud_types::NetworkNetworkList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::Resource(crate::mode::NETWORK_SUBNET),
                stack: true
            }
        ));
        assert!(matches!(actions[1], Action::SetNetworkSubnetListFilters(_)));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_SUBNET),
            None,
            &cloud_types::NetworkNetworkList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_ignores_show_resource_for_other_key() {
        let net = make_network("net-1", "test-net");
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_NETWORK),
            Some(&net),
            &cloud_types::NetworkNetworkList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let net = make_network("net-1", "test-net");
        let actions = NetworkNetworksBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&net),
            &cloud_types::NetworkNetworkList::default(),
        );
        assert!(actions.is_empty());
    }
}
