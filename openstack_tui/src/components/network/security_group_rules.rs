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
    NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleDelete,
    NetworkSecurityGroupRuleDeleteBuilder, NetworkSecurityGroupRuleList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{Mutation, ResourceBehaviour};
use crate::mode::Mode;
use openstack_types::network::v2::security_group_rule::response::list::SecurityGroupRuleResponse;
use serde_json::Value;

const VIEW_CONFIG_KEY: &str = "network.security_group_rule";

impl crate::utils::ResourceKey for SecurityGroupRuleResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&SecurityGroupRuleResponse> for NetworkSecurityGroupRuleDelete {
    type Error = crate::cloud_worker::network::v2::NetworkSecurityGroupRuleDeleteBuilderError;
    fn try_from(value: &SecurityGroupRuleResponse) -> Result<Self, Self::Error> {
        let mut builder = NetworkSecurityGroupRuleDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        builder.build()
    }
}

pub struct NetworkSecurityGroupRulesBehaviour;

impl ResourceBehaviour for NetworkSecurityGroupRulesBehaviour {
    type Item = SecurityGroupRuleResponse;
    type Filter = NetworkSecurityGroupRuleList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "SecurityGroupRules"
    }
    fn mode() -> Mode {
        Mode::NetworkSecurityGroupRules
    }
    fn normalise_filter(mut filter: Self::Filter) -> Self::Filter {
        if filter.sort_key.is_none() {
            filter.sort_key = Some(vec![
                "ethertype".into(),
                "direction".into(),
                "protocol".into(),
                "port_range_min".into(),
            ]);
            filter.sort_dir = Some(vec!["asc".into(), "asc".into(), "asc".into(), "asc".into()]);
        }
        filter
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(NetworkSecurityGroupRuleApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(**boxreq, NetworkSecurityGroupRuleApiRequest::List(_))
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetNetworkSecurityGroupRuleListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::DeleteNetworkSecurityGroupRule = action {
            let del = NetworkSecurityGroupRuleDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(
                NetworkSecurityGroupRuleApiRequest::Delete(Box::new(del)),
            ))
        } else {
            None
        }
    }
    // TODO: editor_template for CreateNetworkSecurityGroupRule needs filter.security_group_id
    // and deserialize_edit_result needs to deserialize NetworkSecurityGroupRuleCreate.
    // This is left as a manual implementation for now.
    fn handle_mutation_response(request: &ApiRequest, data: &Value) -> Option<Vec<Mutation>> {
        if let ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(req)) = request {
            if let NetworkSecurityGroupRuleApiRequest::Delete(del) = &**req {
                return Some(vec![Mutation::DeleteRow(del.id.clone())]);
            }
            if let NetworkSecurityGroupRuleApiRequest::Create(_) = &**req {
                return Some(vec![Mutation::AppendRow(data.clone())]);
            }
        }
        None
    }
    fn clear_data_on_filter_change() -> bool {
        true
    }
}

pub type NetworkSecurityGroupRules =
    GenericResourceView<'static, NetworkSecurityGroupRulesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::network::v2::security_group_rule::response::list::SecurityGroupRuleResponse;

    fn make_rule(id: &str) -> SecurityGroupRuleResponse {
        let json = serde_json::json!({ "id": id });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::view_key(),
            "network.security_group_rule"
        );
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::title(),
            "SecurityGroupRules"
        );
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::mode(),
            Mode::NetworkSecurityGroupRules
        );
    }

    #[test]
    fn normalise_filter_sets_4_field_sort() {
        let filter = NetworkSecurityGroupRuleList::default();
        let norm = NetworkSecurityGroupRulesBehaviour::normalise_filter(filter);
        assert_eq!(
            norm.sort_key,
            Some(vec![
                "ethertype".into(),
                "direction".into(),
                "protocol".into(),
                "port_range_min".into(),
            ])
        );
        assert_eq!(
            norm.sort_dir,
            Some(vec!["asc".into(), "asc".into(), "asc".into(), "asc".into()])
        );
    }

    #[test]
    fn normalise_filter_preserves_existing_sort_key() {
        let mut filter = NetworkSecurityGroupRuleList::default();
        filter.sort_key = Some(vec!["id".into()]);
        let norm = NetworkSecurityGroupRulesBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some(vec!["id".into()]));
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = NetworkSecurityGroupRuleList::default();
        let request = NetworkSecurityGroupRulesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupRuleApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = NetworkSecurityGroupRuleList::default();
        let request = NetworkSecurityGroupRulesBehaviour::request_from_filter(&filter);
        assert!(NetworkSecurityGroupRulesBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_delete() {
        let del = NetworkSecurityGroupRuleDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let request = ApiRequest::from(NetworkSecurityGroupRuleApiRequest::Delete(Box::new(del)));
        assert!(!NetworkSecurityGroupRulesBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = NetworkSecurityGroupRuleList::default();
        let action = Action::SetNetworkSecurityGroupRuleListFilters(filter);
        let result = NetworkSecurityGroupRulesBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = NetworkSecurityGroupRulesBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let rule = make_rule("rule-1");
        let result = NetworkSecurityGroupRulesBehaviour::confirm_request(
            &Action::DeleteNetworkSecurityGroupRule,
            Some(&rule),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupRuleApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = NetworkSecurityGroupRulesBehaviour::confirm_request(
            &Action::DeleteNetworkSecurityGroupRule,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn handle_mutation_response_delete() {
        let del = NetworkSecurityGroupRuleDeleteBuilder::default()
            .id("rule-1".into())
            .build()
            .unwrap();
        let request = ApiRequest::from(NetworkSecurityGroupRuleApiRequest::Delete(Box::new(del)));
        let data = serde_json::json!({});
        let result = NetworkSecurityGroupRulesBehaviour::handle_mutation_response(&request, &data);
        let muts = result.unwrap();
        assert_eq!(muts.len(), 1);
        if let Mutation::DeleteRow(found_id) = &muts[0] {
            assert_eq!(found_id, "rule-1");
        } else {
            panic!("Expected DeleteRow mutation");
        }
    }

    #[test]
    fn handle_mutation_response_create() {
        let filter = NetworkSecurityGroupRuleList::default();
        let request = NetworkSecurityGroupRulesBehaviour::request_from_filter(&filter);
        let data = serde_json::json!({ "id": "new-rule" });
        let result = NetworkSecurityGroupRulesBehaviour::handle_mutation_response(&request, &data);
        assert!(result.is_none());
    }

    #[test]
    fn handle_mutation_response_list_returns_none() {
        let filter = NetworkSecurityGroupRuleList::default();
        let request = NetworkSecurityGroupRulesBehaviour::request_from_filter(&filter);
        let data = serde_json::json!({});
        let result = NetworkSecurityGroupRulesBehaviour::handle_mutation_response(&request, &data);
        assert!(result.is_none());
    }

    #[test]
    fn clear_data_on_filter_change() {
        assert!(NetworkSecurityGroupRulesBehaviour::clear_data_on_filter_change());
    }
}
