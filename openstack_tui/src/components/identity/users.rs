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
use crate::cloud_worker::identity::v3::{
    IdentityApiRequest, IdentityUserApiRequest, IdentityUserApplicationCredentialList,
    IdentityUserApplicationCredentialListBuilder, IdentityUserDelete, IdentityUserDeleteBuilder,
    IdentityUserList, IdentityUserSetBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{Mutation, ResourceBehaviour};
use crate::mode::Mode;
use openstack_types::identity::v3::user::response::list::UserResponse;
use serde_json::Value;

const VIEW_CONFIG_KEY: &str = "identity.user";

impl crate::utils::ResourceKey for UserResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&UserResponse> for IdentityUserDelete {
    type Error = crate::cloud_worker::identity::v3::IdentityUserDeleteBuilderError;
    fn try_from(value: &UserResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityUserDeleteBuilder::default();
        builder.id(value.id.clone());
        builder.name(value.name.clone());
        builder.build()
    }
}

impl TryFrom<&UserResponse> for IdentityUserApplicationCredentialList {
    type Error =
        crate::cloud_worker::identity::v3::IdentityUserApplicationCredentialListBuilderError;
    fn try_from(value: &UserResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityUserApplicationCredentialListBuilder::default();
        builder.user_id(value.id.clone());
        builder.user_name(value.name.clone());
        builder.build()
    }
}

pub struct IdentityUsersBehaviour;

impl ResourceBehaviour for IdentityUsersBehaviour {
    type Item = UserResponse;
    type Filter = IdentityUserList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Identity Users"
    }
    fn mode() -> Mode {
        Mode::IdentityUsers
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(IdentityUserApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
            if matches!(**boxreq, IdentityUserApiRequest::List(_))
        )
    }
    fn action_to_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::IdentityUserFlipEnable = action {
            let sel = selected?;
            let req: crate::cloud_worker::identity::v3::user::set::User =
                crate::cloud_worker::identity::v3::user::set::UserBuilder::default()
                    .enabled(!sel.enabled)
                    .build()
                    .ok()?;
            let set_req = IdentityUserSetBuilder::default()
                .id(sel.id.clone())
                .user(req)
                .build()
                .ok()?;
            Some(ApiRequest::from(IdentityUserApiRequest::Set(Box::new(
                set_req,
            ))))
        } else {
            None
        }
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::IdentityUserDelete = action {
            let del = IdentityUserDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(IdentityUserApiRequest::Delete(Box::new(
                del,
            ))))
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowIdentityUserApplicationCredentials = action
            && let Some(sel) = selected
            && let Ok(list) = IdentityUserApplicationCredentialList::try_from(sel)
        {
            return vec![
                Action::SetIdentityApplicationCredentialListFilters(list),
                Action::Mode {
                    mode: Mode::IdentityApplicationCredentials,
                    stack: true,
                },
            ];
        }
        Vec::new()
    }
    fn handle_mutation_response(request: &ApiRequest, data: &Value) -> Option<Vec<Mutation>> {
        if let ApiRequest::Identity(IdentityApiRequest::User(req)) = request {
            if let IdentityUserApiRequest::Delete(del) = &**req {
                return Some(vec![Mutation::DeleteRow(del.id.clone())]);
            }
            if let IdentityUserApiRequest::Set(_) = &**req
                && let Some(id) = data.get("id").and_then(|v| v.as_str().map(String::from))
            {
                return Some(vec![Mutation::UpdateRow(id, data.clone())]);
            }
        }
        None
    }
}

pub type IdentityUsers = GenericResourceView<'static, IdentityUsersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::identity::v3::user::response::list::UserResponse;

    fn make_user(id: &str, name: &str, enabled: bool) -> UserResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "enabled": enabled,
            "domain_id": "default"
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(IdentityUsersBehaviour::view_key(), "identity.user");
        assert_eq!(IdentityUsersBehaviour::title(), "Identity Users");
        assert_eq!(IdentityUsersBehaviour::mode(), Mode::IdentityUsers);
    }

    #[test]
    fn normalise_filter_passthrough() {
        let filter = IdentityUserList::default();
        let filter_clone = filter.clone();
        let norm = IdentityUsersBehaviour::normalise_filter(filter);
        assert_eq!(norm, filter_clone);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = IdentityUserList::default();
        let request = IdentityUsersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
            if matches!(*boxreq, IdentityUserApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = IdentityUserList::default();
        let request = IdentityUsersBehaviour::request_from_filter(&filter);
        assert!(IdentityUsersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_delete() {
        let del = IdentityUserDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let request = ApiRequest::from(IdentityUserApiRequest::Delete(Box::new(del)));
        assert!(!IdentityUsersBehaviour::matches_request(&request));
    }

    #[test]
    fn handle_set_filter_action_returns_none() {
        let result = IdentityUsersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn action_to_request_flip_enable_with_selected() {
        let user = make_user("user-1", "test-user", true);
        let result =
            IdentityUsersBehaviour::action_to_request(&Action::IdentityUserFlipEnable, Some(&user));
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
            if matches!(*boxreq, IdentityUserApiRequest::Set(_))
        ));
    }

    #[test]
    fn action_to_request_flip_enable_without_selected() {
        let result =
            IdentityUsersBehaviour::action_to_request(&Action::IdentityUserFlipEnable, None);
        assert!(result.is_none());
    }

    #[test]
    fn action_to_request_returns_none_for_unrelated_action() {
        let user = make_user("user-1", "test-user", true);
        let result = IdentityUsersBehaviour::action_to_request(&Action::Tick, Some(&user));
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let user = make_user("user-1", "test-user", true);
        let result =
            IdentityUsersBehaviour::confirm_request(&Action::IdentityUserDelete, Some(&user));
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
            if matches!(*boxreq, IdentityUserApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = IdentityUsersBehaviour::confirm_request(&Action::IdentityUserDelete, None);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated_action() {
        let user = make_user("user-1", "test-user", true);
        let result = IdentityUsersBehaviour::confirm_request(&Action::Tick, Some(&user));
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_credentials_with_selected() {
        let user = make_user("user-1", "test-user", true);
        let actions = IdentityUsersBehaviour::filter_carry_action(
            &Action::ShowIdentityUserApplicationCredentials,
            Some(&user),
            &IdentityUserList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetIdentityApplicationCredentialListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::IdentityApplicationCredentials,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_show_credentials_without_selected() {
        let actions = IdentityUsersBehaviour::filter_carry_action(
            &Action::ShowIdentityUserApplicationCredentials,
            None,
            &IdentityUserList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let user = make_user("user-1", "test-user", true);
        let actions = IdentityUsersBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&user),
            &IdentityUserList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn handle_mutation_response_delete() {
        let del = IdentityUserDeleteBuilder::default()
            .id("user-1".into())
            .build()
            .unwrap();
        let request = ApiRequest::from(IdentityUserApiRequest::Delete(Box::new(del)));
        let data = serde_json::json!({});
        let result = IdentityUsersBehaviour::handle_mutation_response(&request, &data);
        let muts = result.unwrap();
        assert_eq!(muts.len(), 1);
        if let Mutation::DeleteRow(found_id) = &muts[0] {
            assert_eq!(found_id, "user-1");
        } else {
            panic!("Expected DeleteRow mutation");
        }
    }

    #[test]
    fn handle_mutation_response_set() {
        let set_req = IdentityUserSetBuilder::default()
            .id("user-1".into())
            .user(
                crate::cloud_worker::identity::v3::user::set::UserBuilder::default()
                    .enabled(false)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let request = ApiRequest::from(IdentityUserApiRequest::Set(Box::new(set_req)));
        let data = serde_json::json!({ "id": "user-1", "name": "test" });
        let result = IdentityUsersBehaviour::handle_mutation_response(&request, &data);
        assert!(matches!(result, Some(ref muts) if muts.len() == 1));
        if let Some(ref muts) = result {
            if let Mutation::UpdateRow(found_id, _) = &muts[0] {
                assert_eq!(found_id, "user-1");
            } else {
                panic!("Expected UpdateRow mutation");
            }
        }
    }

    #[test]
    fn handle_mutation_response_non_matching() {
        let filter = IdentityUserList::default();
        let request = IdentityUsersBehaviour::request_from_filter(&filter);
        let data = serde_json::json!({ "id": "user-1" });
        let result = IdentityUsersBehaviour::handle_mutation_response(&request, &data);
        assert!(result.is_none());
    }
}
