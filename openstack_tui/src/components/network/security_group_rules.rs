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
    NetworkApiRequest, NetworkSecurityGroupRuleApiRequest, NetworkSecurityGroupRuleCreate,
    NetworkSecurityGroupRuleDelete, NetworkSecurityGroupRuleDeleteBuilder,
    NetworkSecurityGroupRuleList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{
    GeneratedResourceBehaviour, Mutation, ResourceBehaviour,
};
use crate::mode::Mode;
use serde_json::Value;

impl TryFrom<&Value> for NetworkSecurityGroupRuleDelete {
    type Error = crate::cloud_worker::network::v2::NetworkSecurityGroupRuleDeleteBuilderError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut builder = NetworkSecurityGroupRuleDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
        }
        builder.build()
    }
}

pub struct NetworkSecurityGroupRulesBehaviour;

impl ResourceBehaviour for NetworkSecurityGroupRulesBehaviour {
    type Filter = NetworkSecurityGroupRuleList;

    fn view_key() -> &'static str {
        super::generated::security_group_rule::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::security_group_rule::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::security_group_rule::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::security_group_rule::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::security_group_rule::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::security_group_rule::Generated::handle_set_filter_action(action)
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
    fn confirm_request(action: &Action, selected: Option<&Value>) -> Option<ApiRequest> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Delete,
        } = action
            && *key == Self::view_key()
        {
            let del = NetworkSecurityGroupRuleDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(
                NetworkSecurityGroupRuleApiRequest::Delete(Box::new(del)),
            ))
        } else {
            None
        }
    }
    fn editor_template(action: &Action, filter: &Self::Filter) -> Option<(String, ApiRequest)> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Create,
        } = action
            && *key == Self::view_key()
        {
            let security_group_id = filter.security_group_id.clone().unwrap_or_default();
            let template = format!(
                r#"# Create a security group rule.
# direction: ingress | egress
# ethertype: IPv4 | IPv6
# protocol: e.g. tcp, udp, icmp, or leave empty to match any protocol
# Specify either remote_group_id or remote_ip_prefix, not both.
security_group_rule:
  security_group_id: {security_group_id}
  direction: ingress
  ethertype: IPv4
  protocol:
  port_range_min:
  port_range_max:
  remote_ip_prefix:
  remote_group_id:
  remote_address_group_id:
  description:
  tenant_id:
"#
            );
            let request =
                ApiRequest::from(NetworkSecurityGroupRuleApiRequest::Create(Box::default()));
            return Some((template, request));
        }
        None
    }
    fn deserialize_edit_result(data: &Value) -> Option<ApiRequest> {
        let create: NetworkSecurityGroupRuleCreate = serde_json::from_value(data.clone()).ok()?;
        Some(ApiRequest::from(
            NetworkSecurityGroupRuleApiRequest::Create(Box::new(create)),
        ))
    }
    fn editor_schema(action: &Action) -> Option<&'static str> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Create,
        } = action
            && *key == Self::view_key()
        {
            return Some(NetworkSecurityGroupRuleCreate::BODY_SCHEMA);
        }
        None
    }
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

    fn make_rule(id: &str) -> serde_json::Value {
        serde_json::json!({ "id": id })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::view_key(),
            "network.security_group_rule"
        );
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::title(),
            "Security Group Rules"
        );
        assert_eq!(
            NetworkSecurityGroupRulesBehaviour::mode(),
            Mode::Resource(crate::mode::NETWORK_SECURITY_GROUP_RULE)
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
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP_RULE,
                op: crate::action::ResourceOp::Delete,
            },
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
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP_RULE,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let rule = make_rule("rule-1");
        let result = NetworkSecurityGroupRulesBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&rule),
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

    #[test]
    fn editor_schema_returns_body_schema_for_create() {
        let schema = NetworkSecurityGroupRulesBehaviour::editor_schema(&Action::ResourceOp {
            key: crate::mode::NETWORK_SECURITY_GROUP_RULE,
            op: crate::action::ResourceOp::Create,
        });
        assert_eq!(schema, Some(NetworkSecurityGroupRuleCreate::BODY_SCHEMA));
        assert!(schema.unwrap().contains("\"IPv4\""));
    }

    #[test]
    fn editor_schema_ignores_other_actions() {
        assert!(NetworkSecurityGroupRulesBehaviour::editor_schema(&Action::Tick).is_none());
    }

    #[test]
    fn editor_template_ignores_other_actions() {
        let filter = NetworkSecurityGroupRuleList::default();
        let result = NetworkSecurityGroupRulesBehaviour::editor_template(&Action::Tick, &filter);
        assert!(result.is_none());
    }

    #[test]
    fn editor_template_ignores_create_for_other_resource() {
        let filter = NetworkSecurityGroupRuleList::default();
        let result = NetworkSecurityGroupRulesBehaviour::editor_template(
            &Action::ResourceOp {
                key: "network.security_group",
                op: crate::action::ResourceOp::Create,
            },
            &filter,
        );
        assert!(result.is_none());
    }

    #[test]
    fn editor_template_prefills_security_group_id_from_filter() {
        let filter = NetworkSecurityGroupRuleList {
            security_group_id: Some("sg-1".into()),
            ..Default::default()
        };
        let result = NetworkSecurityGroupRulesBehaviour::editor_template(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP_RULE,
                op: crate::action::ResourceOp::Create,
            },
            &filter,
        );
        let (template, request) = result.expect("editor_template should return a template");
        assert!(template.contains("security_group_id: sg-1"));
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupRuleApiRequest::Create(_))
        ));
    }

    #[test]
    fn deserialize_edit_result_builds_create_request() {
        let edited = serde_yaml::from_str::<serde_json::Value>(
            r#"
security_group_rule:
  security_group_id: sg-1
  direction: ingress
  ethertype: IPv4
  protocol: tcp
  port_range_min: 22
  port_range_max: 22
  remote_ip_prefix: 0.0.0.0/0
"#,
        )
        .unwrap();
        let request = NetworkSecurityGroupRulesBehaviour::deserialize_edit_result(&edited)
            .expect("valid YAML should deserialize");
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroupRule(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupRuleApiRequest::Create(_))
        ));
    }

    #[test]
    fn deserialize_edit_result_rejects_unknown_fields() {
        let edited = serde_json::json!({ "not_a_security_group_rule": {} });
        let request = NetworkSecurityGroupRulesBehaviour::deserialize_edit_result(&edited);
        assert!(request.is_none());
    }
}
