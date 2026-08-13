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
    NetworkApiRequest, NetworkSecurityGroupApiRequest, NetworkSecurityGroupDelete,
    NetworkSecurityGroupDeleteBuilder, NetworkSecurityGroupList, NetworkSecurityGroupRuleList,
    NetworkSecurityGroupRuleListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{
    GeneratedResourceBehaviour, Mutation, ResourceBehaviour,
};
use crate::mode::Mode;
use serde_json::Value;

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

impl TryFrom<&Value> for NetworkSecurityGroupDelete {
    type Error = crate::cloud_worker::network::v2::NetworkSecurityGroupDeleteBuilderError;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let mut builder = NetworkSecurityGroupDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
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
    fn confirm_request(action: &Action, selected: Option<&Value>) -> Option<ApiRequest> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Delete,
        } = action
            && *key == Self::view_key()
        {
            let del = NetworkSecurityGroupDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(NetworkSecurityGroupApiRequest::Delete(
                Box::new(del),
            )))
        } else {
            None
        }
    }
    fn editor_template(
        action: &Action,
        filter: &Self::Filter,
        selected: Option<&Value>,
    ) -> Option<(String, ApiRequest)> {
        super::generated::security_group::Generated::editor_template(action, filter, selected)
    }
    fn deserialize_edit_result(data: &Value, original_action: &Action) -> Option<ApiRequest> {
        super::generated::security_group::Generated::deserialize_edit_result(data, original_action)
    }
    fn editor_schema(action: &Action) -> Option<&'static str> {
        super::generated::security_group::Generated::editor_schema(action)
    }
    fn handle_mutation_response(request: &ApiRequest, data: &Value) -> Option<Vec<Mutation>> {
        if let ApiRequest::Network(NetworkApiRequest::SecurityGroup(req)) = request {
            match &**req {
                NetworkSecurityGroupApiRequest::Create(_) => {
                    return Some(vec![Mutation::AppendRow(data.clone())]);
                }
                NetworkSecurityGroupApiRequest::Set(_) => {
                    if let Some(id) = data.get("id").and_then(|v| v.as_str()) {
                        return Some(vec![Mutation::UpdateRow(id.to_string(), data.clone())]);
                    }
                }
                NetworkSecurityGroupApiRequest::Delete(del) => {
                    return Some(vec![Mutation::DeleteRow(del.id.clone())]);
                }
                _ => {}
            }
        }
        None
    }
}

pub type NetworkSecurityGroups = GenericResourceView<'static, NetworkSecurityGroupsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::network::v2::{
        NetworkApiRequest, NetworkSecurityGroupApiRequest, NetworkSecurityGroupCreate,
    };
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

    #[test]
    fn editor_schema_returns_body_schema_for_create() {
        let schema = NetworkSecurityGroupsBehaviour::editor_schema(&Action::ResourceOp {
            key: crate::mode::NETWORK_SECURITY_GROUP,
            op: crate::action::ResourceOp::Create,
        });
        assert_eq!(schema, Some(NetworkSecurityGroupCreate::BODY_SCHEMA));
    }

    #[test]
    fn editor_schema_ignores_other_actions() {
        assert!(NetworkSecurityGroupsBehaviour::editor_schema(&Action::Tick).is_none());
    }

    #[test]
    fn editor_template_ignores_other_actions() {
        let filter = NetworkSecurityGroupList::default();
        let result = NetworkSecurityGroupsBehaviour::editor_template(&Action::Tick, &filter, None);
        assert!(result.is_none());
    }

    #[test]
    fn editor_template_ignores_create_for_other_resource() {
        let filter = NetworkSecurityGroupList::default();
        let result = NetworkSecurityGroupsBehaviour::editor_template(
            &Action::ResourceOp {
                key: "network.security_group_rule",
                op: crate::action::ResourceOp::Create,
            },
            &filter,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn editor_template_returns_template_for_create() {
        let filter = NetworkSecurityGroupList::default();
        let result = NetworkSecurityGroupsBehaviour::editor_template(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Create,
            },
            &filter,
            None,
        );
        let (template, request) = result.expect("editor_template should return a template");
        assert!(template.contains("security_group:"));
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupApiRequest::Create(_))
        ));
    }

    #[test]
    fn editor_template_returns_template_for_update_with_live_values() {
        let filter = NetworkSecurityGroupList::default();
        let sg = make_sg("sg-1", "web-servers");
        let result = NetworkSecurityGroupsBehaviour::editor_template(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Update,
            },
            &filter,
            Some(&sg),
        );
        let (template, request) = result.expect("editor_template should return a template");
        assert!(template.contains("name: web-servers"));
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupApiRequest::Set(_))
        ));
    }

    #[test]
    fn editor_template_update_without_selected_returns_none() {
        let filter = NetworkSecurityGroupList::default();
        let result = NetworkSecurityGroupsBehaviour::editor_template(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Update,
            },
            &filter,
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn deserialize_edit_result_builds_create_request() {
        let edited = serde_yaml::from_str::<serde_json::Value>(
            r#"
security_group:
  name: web-servers
  description: allow web traffic
  stateful: true
"#,
        )
        .unwrap();
        let original_action = Action::PerformApiRequest(ApiRequest::from(
            NetworkSecurityGroupApiRequest::Create(Box::default()),
        ));
        let request =
            NetworkSecurityGroupsBehaviour::deserialize_edit_result(&edited, &original_action)
                .expect("valid YAML should deserialize");
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupApiRequest::Create(_))
        ));
    }

    #[test]
    fn deserialize_edit_result_builds_set_request_with_id() {
        let filter = NetworkSecurityGroupList::default();
        let sg = make_sg("sg-1", "web-servers");
        let (template, original_request) = NetworkSecurityGroupsBehaviour::editor_template(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Update,
            },
            &filter,
            Some(&sg),
        )
        .expect("editor_template should return a template");
        let original_action = Action::PerformApiRequest(original_request);

        let edited = serde_yaml::from_str::<serde_json::Value>(&template).unwrap();
        let request =
            NetworkSecurityGroupsBehaviour::deserialize_edit_result(&edited, &original_action)
                .expect("valid YAML should deserialize");
        match request {
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq)) => match *boxreq {
                NetworkSecurityGroupApiRequest::Set(set) => {
                    assert_eq!(set.id, "sg-1");
                }
                other => panic!("expected Set request, got {other:?}"),
            },
            other => panic!("expected SecurityGroup request, got {other:?}"),
        }
    }

    #[test]
    fn deserialize_edit_result_rejects_unknown_fields() {
        let edited = serde_json::json!({ "not_a_security_group": {} });
        let original_action = Action::PerformApiRequest(ApiRequest::from(
            NetworkSecurityGroupApiRequest::Create(Box::default()),
        ));
        let request =
            NetworkSecurityGroupsBehaviour::deserialize_edit_result(&edited, &original_action);
        assert!(request.is_none());
    }

    #[test]
    fn handle_mutation_response_create() {
        let request = ApiRequest::from(NetworkSecurityGroupApiRequest::Create(Box::default()));
        let data = serde_json::json!({ "id": "new-sg" });
        let result = NetworkSecurityGroupsBehaviour::handle_mutation_response(&request, &data);
        let muts = result.expect("create response should produce a mutation");
        assert_eq!(muts.len(), 1);
        assert!(matches!(&muts[0], Mutation::AppendRow(v) if v == &data));
    }

    #[test]
    fn handle_mutation_response_set() {
        let request = ApiRequest::from(NetworkSecurityGroupApiRequest::Set(Box::default()));
        let data = serde_json::json!({ "id": "sg-1", "name": "updated" });
        let result = NetworkSecurityGroupsBehaviour::handle_mutation_response(&request, &data);
        let muts = result.expect("set response should produce a mutation");
        assert_eq!(muts.len(), 1);
        assert!(matches!(
            &muts[0],
            Mutation::UpdateRow(id, v) if id == "sg-1" && v == &data
        ));
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let sg = make_sg("sg-1", "web-servers");
        let result = NetworkSecurityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&sg),
        );
        let request = result.expect("confirm_request should return a delete request");
        assert!(matches!(
            request,
            ApiRequest::Network(NetworkApiRequest::SecurityGroup(boxreq))
            if matches!(*boxreq, NetworkSecurityGroupApiRequest::Delete(ref del) if del.id == "sg-1")
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = NetworkSecurityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::NETWORK_SECURITY_GROUP,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let sg = make_sg("sg-1", "web-servers");
        let result = NetworkSecurityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: "network.security_group_rule",
                op: crate::action::ResourceOp::Delete,
            },
            Some(&sg),
        );
        assert!(result.is_none());
    }

    #[test]
    fn handle_mutation_response_delete() {
        let del = NetworkSecurityGroupDeleteBuilder::default()
            .id("sg-1".to_string())
            .build()
            .unwrap();
        let request = ApiRequest::from(NetworkSecurityGroupApiRequest::Delete(Box::new(del)));
        let data = serde_json::json!({});
        let result = NetworkSecurityGroupsBehaviour::handle_mutation_response(&request, &data);
        let muts = result.expect("delete response should produce a mutation");
        assert_eq!(muts.len(), 1);
        assert!(matches!(&muts[0], Mutation::DeleteRow(id) if id == "sg-1"));
    }

    #[test]
    fn handle_mutation_response_list_returns_none() {
        let filter = NetworkSecurityGroupList::default();
        let request = NetworkSecurityGroupsBehaviour::request_from_filter(&filter);
        let data = serde_json::json!({});
        let result = NetworkSecurityGroupsBehaviour::handle_mutation_response(&request, &data);
        assert!(result.is_none());
    }
}
