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

impl TryFrom<&serde_json::Value> for cloud_types::IdentityGroupUserList {
    type Error = crate::cloud_worker::identity::v3::IdentityGroupUserListBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder =
            crate::cloud_worker::identity::v3::IdentityGroupUserListBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.group_id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.group_name(val.to_string());
        }
        builder.build()
    }
}

impl TryFrom<&serde_json::Value> for cloud_types::IdentityGroupDelete {
    type Error = crate::cloud_worker::identity::v3::IdentityGroupDeleteBuilderError;
    fn try_from(value: &serde_json::Value) -> Result<Self, Self::Error> {
        let mut builder = crate::cloud_worker::identity::v3::IdentityGroupDeleteBuilder::default();
        if let Some(val) = crate::components::view_render::get_str(value, "/id") {
            builder.id(val.to_string());
        }
        if let Some(val) = crate::components::view_render::get_str(value, "/name") {
            builder.name(val.to_string());
        }
        builder.build()
    }
}

pub struct IdentityGroupsBehaviour;

impl ResourceBehaviour for IdentityGroupsBehaviour {
    type Filter = cloud_types::IdentityGroupList;

    fn view_key() -> &'static str {
        super::generated::group::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::group::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::group::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::group::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::group::Generated::matches_request(request)
    }
    fn confirm_request(
        action: &Action,
        selected: Option<&serde_json::Value>,
    ) -> Option<ApiRequest> {
        if let Action::ResourceOp {
            key,
            op: crate::action::ResourceOp::Delete,
        } = action
            && *key == Self::view_key()
        {
            let del = cloud_types::IdentityGroupDelete::try_from(selected?).ok()?;
            Some(ApiRequest::from(
                cloud_types::IdentityGroupApiRequest::Delete(Box::new(del)),
            ))
        } else {
            None
        }
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&serde_json::Value>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowResource(key) = action
            && *key == crate::mode::IDENTITY_GROUP_USER
            && let Some(sel) = selected
            && let Ok(list) = cloud_types::IdentityGroupUserList::try_from(sel)
        {
            return vec![
                Action::Mode {
                    mode: Mode::Resource(crate::mode::IDENTITY_GROUP_USER),
                    stack: true,
                },
                Action::SetIdentityGroupUserListFilters(list),
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

    fn make_group(id: &str, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "domain_id": "default",
            "description": "test group"
        })
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(IdentityGroupsBehaviour::view_key(), "identity.group");
        assert_eq!(IdentityGroupsBehaviour::title(), "Groups");
        assert_eq!(
            IdentityGroupsBehaviour::mode(),
            Mode::Resource(crate::mode::IDENTITY_GROUP)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = cloud_types::IdentityGroupList::default();
        let request = IdentityGroupsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(cloud_types::IdentityApiRequest::Group(boxreq))
            if matches!(*boxreq, cloud_types::IdentityGroupApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = cloud_types::IdentityGroupList::default();
        let request = IdentityGroupsBehaviour::request_from_filter(&filter);
        assert!(IdentityGroupsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let del = crate::cloud_worker::identity::v3::IdentityGroupDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let req = ApiRequest::from(cloud_types::IdentityGroupApiRequest::Delete(Box::new(del)));
        assert!(!IdentityGroupsBehaviour::matches_request(&req));
    }

    #[test]
    fn confirm_request_delete_with_selected() {
        let group = make_group("group-1", "test-group");
        let result = IdentityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::IDENTITY_GROUP,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&group),
        );
        assert!(result.is_some());
        let request = result.unwrap();
        assert!(matches!(
            request,
            ApiRequest::Identity(cloud_types::IdentityApiRequest::Group(boxreq))
            if matches!(*boxreq, cloud_types::IdentityGroupApiRequest::Delete(_))
        ));
    }

    #[test]
    fn confirm_request_delete_without_selected() {
        let result = IdentityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::IDENTITY_GROUP,
                op: crate::action::ResourceOp::Delete,
            },
            None,
        );
        assert!(result.is_none());
    }

    #[test]
    fn confirm_request_ignores_delete_for_other_resource() {
        let group = make_group("group-1", "test-group");
        let result = IdentityGroupsBehaviour::confirm_request(
            &Action::ResourceOp {
                key: crate::mode::IDENTITY_PROJECT,
                op: crate::action::ResourceOp::Delete,
            },
            Some(&group),
        );
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
            &Action::ShowResource(crate::mode::IDENTITY_GROUP_USER),
            Some(&group),
            &cloud_types::IdentityGroupList::default(),
        );
        assert_eq!(actions.len(), 2);
        assert!(matches!(
            actions[0],
            Action::Mode {
                mode: Mode::Resource(crate::mode::IDENTITY_GROUP_USER),
                stack: true
            }
        ));
        assert!(matches!(
            actions[1],
            Action::SetIdentityGroupUserListFilters(_)
        ));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::IDENTITY_GROUP_USER),
            None,
            &cloud_types::IdentityGroupList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_ignores_show_resource_for_other_key() {
        let group = make_group("group-1", "test-group");
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::ShowResource(crate::mode::IDENTITY_GROUP),
            Some(&group),
            &cloud_types::IdentityGroupList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let group = make_group("group-1", "test-group");
        let actions = IdentityGroupsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&group),
            &cloud_types::IdentityGroupList::default(),
        );
        assert!(actions.is_empty());
    }
}
