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
    IdentityApiRequest, IdentityUserApiRequest, IdentityUserApplicationCredentialApiRequest,
    IdentityUserApplicationCredentialList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;

const VIEW_CONFIG_KEY: &str = "identity.user/application_credential";

pub struct IdentityApplicationCredentialsBehaviour;

impl ResourceBehaviour for IdentityApplicationCredentialsBehaviour {
    type Filter = IdentityUserApplicationCredentialList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Application Credentials"
    }
    fn mode() -> Mode {
        Mode::Resource(Self::view_key())
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(IdentityUserApplicationCredentialApiRequest::List(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
                if matches!(&**boxreq, IdentityUserApiRequest::ApplicationCredential(inner)
                    if matches!(&**inner, IdentityUserApplicationCredentialApiRequest::List(_))
                )
        )
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        if let Action::SetIdentityApplicationCredentialListFilters(f) = action {
            Some(f.clone())
        } else {
            None
        }
    }

    fn seed_filter_from_current_user(
        mut filter: Self::Filter,
        token: &openstack_sdk::types::identity::v3::TokenInfo,
    ) -> Self::Filter {
        if filter.user_id.is_empty() {
            filter.user_id = token.user.id.clone();
        }
        filter
    }

    fn reset_filter_on_cloud_switch(mut filter: Self::Filter) -> Self::Filter {
        filter.user_id = String::new();
        filter
    }

    fn is_filter_ready(filter: &Self::Filter) -> bool {
        !filter.user_id.is_empty()
    }
}

pub type IdentityApplicationCredentials =
    GenericResourceView<'static, IdentityApplicationCredentialsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            IdentityApplicationCredentialsBehaviour::view_key(),
            "identity.user/application_credential"
        );
        assert_eq!(
            IdentityApplicationCredentialsBehaviour::title(),
            "Application Credentials"
        );
        assert_eq!(
            IdentityApplicationCredentialsBehaviour::mode(),
            Mode::Resource(crate::mode::IDENTITY_APPLICATION_CREDENTIAL)
        );
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = IdentityUserApplicationCredentialList::default();
        let request = IdentityApplicationCredentialsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::User(boxreq))
            if matches!(&*boxreq, IdentityUserApiRequest::ApplicationCredential(inner)
                if matches!(&**inner, IdentityUserApplicationCredentialApiRequest::List(_))
            )
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = IdentityUserApplicationCredentialList::default();
        let request = IdentityApplicationCredentialsBehaviour::request_from_filter(&filter);
        assert!(IdentityApplicationCredentialsBehaviour::matches_request(
            &request
        ));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Identity(IdentityApiRequest::Group(Box::new(
            crate::cloud_worker::identity::v3::IdentityGroupApiRequest::List(Box::default()),
        )));
        assert!(!IdentityApplicationCredentialsBehaviour::matches_request(
            &req
        ));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = IdentityUserApplicationCredentialList::default();
        let action = Action::SetIdentityApplicationCredentialListFilters(filter);
        let result = IdentityApplicationCredentialsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result =
            IdentityApplicationCredentialsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }

    #[test]
    fn seed_filter_from_current_user_sets_empty_user_id() {
        let filter = IdentityUserApplicationCredentialList::default();
        assert_eq!(filter.user_id, "");
        let mut token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        token_info.user.id = "current-user-id".to_string();

        let result = IdentityApplicationCredentialsBehaviour::seed_filter_from_current_user(
            filter,
            &token_info,
        );

        assert_eq!(result.user_id, "current-user-id");
    }

    #[test]
    fn seed_filter_from_current_user_does_not_override_existing_user_id() {
        let filter = IdentityUserApplicationCredentialList {
            user_id: "explicitly-selected-user".to_string(),
            ..Default::default()
        };
        let mut token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        token_info.user.id = "current-user-id".to_string();

        let result = IdentityApplicationCredentialsBehaviour::seed_filter_from_current_user(
            filter,
            &token_info,
        );

        assert_eq!(result.user_id, "explicitly-selected-user");
    }

    #[test]
    fn reset_filter_on_cloud_switch_clears_user_id() {
        let filter = IdentityUserApplicationCredentialList {
            user_id: "stale-user-from-previous-cloud".to_string(),
            ..Default::default()
        };

        let result = IdentityApplicationCredentialsBehaviour::reset_filter_on_cloud_switch(filter);

        assert_eq!(result.user_id, "");
    }

    #[test]
    fn reset_then_reseed_picks_up_new_cloud_user() {
        // Regression test: switching clouds must not leave the previous cloud's user_id
        // stuck in the filter forever (seed_filter_from_current_user's "don't clobber an
        // explicit selection" guard would otherwise treat it as already-scoped).
        let filter = IdentityUserApplicationCredentialList {
            user_id: "cloud-a-user".to_string(),
            ..Default::default()
        };

        let filter = IdentityApplicationCredentialsBehaviour::reset_filter_on_cloud_switch(filter);

        let mut token_info = openstack_sdk::types::identity::v3::TokenInfo::default();
        token_info.user.id = "cloud-b-user".to_string();
        let result = IdentityApplicationCredentialsBehaviour::seed_filter_from_current_user(
            filter,
            &token_info,
        );

        assert_eq!(result.user_id, "cloud-b-user");
    }

    #[test]
    fn is_filter_ready_false_when_user_id_empty() {
        let filter = IdentityUserApplicationCredentialList::default();
        assert_eq!(filter.user_id, "");
        assert!(!IdentityApplicationCredentialsBehaviour::is_filter_ready(
            &filter
        ));
    }

    #[test]
    fn is_filter_ready_true_when_user_id_set() {
        let filter = IdentityUserApplicationCredentialList {
            user_id: "user-42".to_string(),
            ..Default::default()
        };
        assert!(IdentityApplicationCredentialsBehaviour::is_filter_ready(
            &filter
        ));
    }
}
