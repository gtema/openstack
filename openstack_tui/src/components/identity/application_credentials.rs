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
use openstack_types::identity::v3::user::application_credential::response::list::ApplicationCredentialResponse;

const VIEW_CONFIG_KEY: &str = "identity.user/application_credential";

impl crate::utils::ResourceKey for ApplicationCredentialResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct IdentityApplicationCredentialsBehaviour;

impl ResourceBehaviour for IdentityApplicationCredentialsBehaviour {
    type Item = ApplicationCredentialResponse;
    type Filter = IdentityUserApplicationCredentialList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Application Credentials"
    }
    fn mode() -> Mode {
        Mode::IdentityApplicationCredentials
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
            Mode::IdentityApplicationCredentials
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
            crate::cloud_worker::identity::v3::IdentityGroupApiRequest::List(Box::new(
                crate::cloud_worker::identity::v3::IdentityGroupList::default(),
            )),
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
}
