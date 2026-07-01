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
    IdentityApiRequest, IdentityGroupApiRequest, IdentityGroupDelete, IdentityGroupDeleteBuilder,
    IdentityGroupList, IdentityGroupUserList, IdentityGroupUserListBuilder,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::identity::v3::group::response::list::GroupResponse;

const VIEW_CONFIG_KEY: &str = "identity.group";

impl crate::utils::ResourceKey for GroupResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

impl TryFrom<&GroupResponse> for IdentityGroupUserList {
    type Error = crate::cloud_worker::identity::v3::IdentityGroupUserListBuilderError;
    fn try_from(value: &GroupResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityGroupUserListBuilder::default();
        if let Some(val) = &value.id {
            builder.group_id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.group_name(val.clone());
        }
        builder.build()
    }
}

impl TryFrom<&GroupResponse> for IdentityGroupDelete {
    type Error = crate::cloud_worker::identity::v3::IdentityGroupDeleteBuilderError;
    fn try_from(value: &GroupResponse) -> Result<Self, Self::Error> {
        let mut builder = IdentityGroupDeleteBuilder::default();
        if let Some(val) = &value.id {
            builder.id(val.clone());
        }
        if let Some(val) = &value.name {
            builder.name(val.clone());
        }
        builder.build()
    }
}

pub struct IdentityGroupsBehaviour;

impl ResourceBehaviour for IdentityGroupsBehaviour {
    type Item = GroupResponse;
    type Filter = IdentityGroupList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Identity Groups"
    }
    fn mode() -> Mode {
        Mode::IdentityGroups
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(IdentityGroupApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Group(boxreq))
            if matches!(**boxreq, IdentityGroupApiRequest::List(_))
        )
    }
    fn confirm_request(action: &Action, selected: Option<&Self::Item>) -> Option<ApiRequest> {
        if let Action::IdentityGroupDelete = action {
            let del = IdentityGroupDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(IdentityGroupApiRequest::Delete(Box::new(
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
        if let Action::ShowIdentityGroupUsers = action
            && let Some(sel) = selected
            && let Ok(list) = IdentityGroupUserList::try_from(sel)
        {
            return vec![
                Action::SetIdentityGroupUserListFilters(list),
                Action::Mode {
                    mode: Mode::IdentityGroupUsers,
                    stack: true,
                },
            ];
        }
        Vec::new()
    }
}

pub type IdentityGroups = GenericResourceView<'static, IdentityGroupsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::identity::v3::group::response::list::GroupResponse;

    fn make_group(id: &str, name: &str) -> GroupResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "domain_id": "default",
            "description": "test group"
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(IdentityGroupsBehaviour::view_key(), "identity.group");
        assert_eq!(IdentityGroupsBehaviour::title(), "Identity Groups");
        assert_eq!(IdentityGroupsBehaviour::mode(), Mode::IdentityGroups);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = IdentityGroupList::default();
        let request = IdentityGroupsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Group(boxreq))
            if matches!(*boxreq, IdentityGroupApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = IdentityGroupList::default();
        let request = IdentityGroupsBehaviour::request_from_filter(&filter);
        assert!(IdentityGroupsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let del = IdentityGroupDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let req = ApiRequest::from(IdentityGroupApiRequest::Delete(Box::new(del)));
        assert!(!IdentityGroupsBehaviour::matches_request(&req));
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let group = make_group("group-1", "test-group");
        let result =
            IdentityGroupsBehaviour::confirm_request(&Action::IdentityGroupDelete, Some(&group));
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Group(boxreq))
            if matches!(*boxreq, IdentityGroupApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = IdentityGroupsBehaviour::confirm_request(&Action::IdentityGroupDelete, None);
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_returns_none_for_unrelated() {
        let group = make_group("group-1", "test-group");
        let result = IdentityGroupsBehaviour::confirm_request(&Action::Tick, Some(&group));
        assert!(result.is_none());
    }

    #[test]
    fn filter_carry_action_show_group_users_with_selected() {
        let group = make_group("group-1", "test-group");
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::ShowIdentityGroupUsers,
            Some(&group),
            &IdentityGroupList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::SetIdentityGroupUserListFilters(_)
        ));
        assert!(matches!(
            actions[1],
            Action::Mode {
                mode: Mode::IdentityGroupUsers,
                stack: true
            }
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::ShowIdentityGroupUsers,
            None,
            &IdentityGroupList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let group = make_group("group-1", "test-group");
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&group),
            &IdentityGroupList::default(),
        );
        assert!(actions.is_empty());
    }
}
