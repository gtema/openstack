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
    NetworkSecurityGroupList, NetworkSecurityGroupRuleList, NetworkSecurityGroupRuleListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

impl TryFrom<&serde_json::Value> for NetworkSecurityGroupRuleList {
    type Error = crate::cloud_worker::network::v2::NetworkSecurityGroupRuleListBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder = NetworkSecurityGroupRuleListBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.security_group_id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.security_group_name(val.to_string());
        }
        builder.build()
    }
}

pub struct NetworkSecurityGroupsBehaviour;

impl ResourceBehaviour for NetworkSecurityGroupsBehaviour {
    type Filter = NetworkSecurityGroupList;

    fn view_key() -> &'static str {
        super::generated::security_group::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::security_group::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::security_group::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::security_group::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::security_group::Generated::matches_request(request)
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(Vec::from(["name".into()]));
            filter.sort_dir = Some(Vec::from(["asc".into()]));
        }
        filter
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&serde_json::Value>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowResource(key) = action
            && *key == crate::mode::NETWORK_SECURITY_GROUP_RULE
            && let Some(sel) = selected
            && let Ok(list) = NetworkSecurityGroupRuleList::try_from(sel)
        {
            return vec![
                Action::Mode {
                    mode: Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP_RULE),
                    stack: true,
                },
                Action::SetNetworkSecurityGroupRuleListFilters(list),
            ];
        }
        Vec::new()
    }
}

pub type NetworkSecurityGroups = GenericResourceView<'static, NetworkSecurityGroupsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::network::v2::{NetworkApiRequest, NetworkSecurityGroupApiRequest};
    use crate::components::resource_behaviour::ResourceBehaviour;

    fn make_sg(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "description": "test sg",
            "tenant_id": "tenant1",
            "security_group_rules": [],
            "created_at": "2024-01-01T00:00:00",
            "updated_at": "2024-01-01T00:00:00"
        })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            NetworkSecurityGroupsBehaviour::view_key(),
            "network.security_group"
        );
        assert_eq!(NetworkSecurityGroupsBehaviour::title(), "Security Groups");
        assert_eq!(
            NetworkSecurityGroupsBehaviour::mode(),
            Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP)
        );
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = NetworkSecurityGroupList::default();
        let norm = NetworkSecurityGroupsBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(Vec::from(["name".into()])));
        assert_eq!(norm.sort_dir, Some(Vec::from(["asc".into()])));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut f = NetworkSecurityGroupList::default();
        f.sort_key = Some(Vec::from(["id".into()]));
        let norm = NetworkSecurityGroupsBehaviour::normalise_filter(f);
        assert_eq!(norm.sort_key, Some(Vec::from(["id".into()])));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = NetworkSecurityGroupList::default();
        let request = NetworkSecurityGroupsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = NetworkSecurityGroupList::default();
        let request = NetworkSecurityGroupsBehaviour::request_from_filter(&filter);
        assert!(NetworkSecurityGroupsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Network(NetworkApiRequest::Network(Box::new(
            crate::cloud_worker::types::NetworkNetworkApiRequest::List(Box::default()),
        )));
        assert!(!NetworkSecurityGroupsBehaviour::matches_request(&req));
    }

    #[test]
    fn filter_carry_action_show_rules_with_selected() {
        let sg = make_sg("sg-1", "test-sg");
        let actions = NetworkSecurityGroupsBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_SECURITY_GROUP_RULE),
            Some(&sg),
            &NetworkSecurityGroupList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP_RULE),
                stack: true
            }
        ));
        assert!(matches!(
            actions[1],
            Action::SetNetworkSecurityGroupRuleListFilters(_)
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = NetworkSecurityGroupsBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_SECURITY_GROUP_RULE),
            None,
            &NetworkSecurityGroupList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_ignores_show_resource_for_other_key() {
        let sg = make_sg("sg-1", "test-sg");
        let actions = NetworkSecurityGroupsBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::NETWORK_SECURITY_GROUP),
            Some(&sg),
            &NetworkSecurityGroupList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let sg = make_sg("sg-1", "test-sg");
        let actions = NetworkSecurityGroupsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&sg),
            &NetworkSecurityGroupList::default(),
        );
        assert!(actions.is_empty());
    }
}
