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
    IdentityApiRequest, IdentityGroupApiRequest, IdentityGroupUserApiRequest, IdentityGroupUserList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::identity::v3::group::user::response::list::UserResponse;

const VIEW_CONFIG_KEY: &str = "identity.user";

impl crate::utils::ResourceKey for UserResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct IdentityGroupUsersBehaviour;

impl ResourceBehaviour for IdentityGroupUsersBehaviour {
    type Item = UserResponse;
    type Filter = IdentityGroupUserList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Identity Group Users"
    }
    fn mode() -> Mode {
        Mode::IdentityGroupUsers
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(IdentityGroupUserApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Group(boxreq))
                if matches!(&**boxreq, IdentityGroupApiRequest::User(inner)
                    if matches!(&**inner, IdentityGroupUserApiRequest::List(_))
                )
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetIdentityGroupUserListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }
}

pub type IdentityGroupUsers = GenericResourceView<'static, IdentityGroupUsersBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(IdentityGroupUsersBehaviour::view_key(), "identity.user");
        assert_eq!(IdentityGroupUsersBehaviour::title(), "Identity Group Users");
        assert_eq!(
            IdentityGroupUsersBehaviour::mode(),
            Mode::IdentityGroupUsers
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = IdentityGroupUserList::default();
        let request = IdentityGroupUsersBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Group(boxreq))
            if matches!(&*boxreq, IdentityGroupApiRequest::User(inner)
                if matches!(&**inner, IdentityGroupUserApiRequest::List(_))
            )
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = IdentityGroupUserList::default();
        let request = IdentityGroupUsersBehaviour::request_from_filter(&filter);
        assert!(IdentityGroupUsersBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Identity(IdentityApiRequest::User(Box::new(
            crate::cloud_worker::identity::v3::IdentityUserApiRequest::List(Box::new(
                crate::cloud_worker::identity::v3::IdentityUserList::default(),
            )),
        )));
        assert!(!IdentityGroupUsersBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = IdentityGroupUserList::default();
        let action = Action::SetIdentityGroupUserListFilters(filter);
        let result = IdentityGroupUsersBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = IdentityGroupUsersBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
