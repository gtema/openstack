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
    IdentityApiRequest, IdentityProjectApiRequest, IdentityProjectList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::identity::v3::project::response::list::ProjectResponse;

const VIEW_CONFIG_KEY: &str = "identity.project";

impl crate::utils::ResourceKey for ProjectResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct IdentityProjectsBehaviour;

impl ResourceBehaviour for IdentityProjectsBehaviour {
    type Item = ProjectResponse;
    type Filter = IdentityProjectList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Identity Projects"
    }
    fn mode() -> Mode {
        Mode::IdentityProjects
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(IdentityProjectApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Project(boxreq))
            if matches!(**boxreq, IdentityProjectApiRequest::List(_))
        )
    }
    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::SwitchToProject = action
            && let Some(sel) = selected
        {
            let scope = openstack_sdk::types::identity::v3::Project {
                id: sel.id.clone(),
                name: sel.name.clone(),
                domain: Some(openstack_sdk::types::identity::v3::Domain {
                    id: sel.domain_id.clone(),
                    name: None,
                }),
            };
            return vec![Action::CloudChangeScope(Box::new(
                openstack_sdk::auth::authtoken::AuthTokenScope::Project(scope),
            ))];
        }
        Vec::new()
    }
}

pub type IdentityProjects = GenericResourceView<'static, IdentityProjectsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::identity::v3::project::response::list::ProjectResponse;

    fn make_project(id: &str, name: &str, domain_id: &str) -> ProjectResponse {
        let json = serde_json::json!({
            "id": id,
            "name": name,
            "domain_id": domain_id,
            "enabled": true,
            "description": "test project"
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(IdentityProjectsBehaviour::view_key(), "identity.project");
        assert_eq!(IdentityProjectsBehaviour::title(), "Identity Projects");
        assert_eq!(IdentityProjectsBehaviour::mode(), Mode::IdentityProjects);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = IdentityProjectList::default();
        let request = IdentityProjectsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Identity(IdentityApiRequest::Project(boxreq))
            if matches!(*boxreq, IdentityProjectApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list() {
        let filter = IdentityProjectList::default();
        let request = IdentityProjectsBehaviour::request_from_filter(&filter);
        assert!(IdentityProjectsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let del = crate::cloud_worker::identity::v3::IdentityProjectDeleteBuilder::default()
            .id("test".into())
            .build()
            .unwrap();
        let req = ApiRequest::from(IdentityProjectApiRequest::Delete(Box::new(del)));
        assert!(!IdentityProjectsBehaviour::matches_request(&req));
    }

    #[test]
    fn filter_carry_action_switch_project_with_selected() {
        let project = make_project("proj-1", "test-proj", "domain-1");
        let actions = IdentityProjectsBehaviour::filter_carry_action(
            &Action::SwitchToProject,
            Some(&project),
            &IdentityProjectList::default(),
        );
        assert_eq!(actions.len(), 1);
        assert!(matches!(actions[0], Action::CloudChangeScope(_)));
    }

    #[test]
    fn filter_carry_action_without_selected() {
        let actions = IdentityProjectsBehaviour::filter_carry_action(
            &Action::SwitchToProject,
            None,
            &IdentityProjectList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_returns_empty_for_unrelated() {
        let project = make_project("proj-1", "test-proj", "domain-1");
        let actions = IdentityProjectsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&project),
            &IdentityProjectList::default(),
        );
        assert!(actions.is_empty());
    }
}
