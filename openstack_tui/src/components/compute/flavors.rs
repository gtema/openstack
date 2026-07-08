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

use openstack_types::compute::v2::flavor::response::list_detailed_255::FlavorResponse;

use crate::{
    action::Action,
    cloud_worker::compute::v2::{
        ComputeApiRequest, ComputeFlavorApiRequest, ComputeFlavorList, ComputeServerListBuilder,
    },
    cloud_worker::types::ApiRequest,
    components::generic_resource_view::GenericResourceView,
    components::resource_behaviour::{Mutation, ResourceBehaviour},
    mode::Mode,
    utils::ResourceKey,
};

const TITLE: &str = "Compute Flavors";
const VIEW_CONFIG_KEY: &str = "compute.flavor";

impl ResourceKey for FlavorResponse {
    fn get_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
}

pub struct ComputeFlavorsBehaviour;

impl ResourceBehaviour for ComputeFlavorsBehaviour {
    type Item = FlavorResponse;
    type Filter = ComputeFlavorList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }

    fn title() -> &'static str {
        TITLE
    }

    fn mode() -> Mode {
        Mode::ComputeFlavors
    }

    fn normalise_filter(filter: Self::Filter) -> Self::Filter {
        let mut f = filter;
        if f.sort_key.is_none() {
            f.sort_key = Some("name".into());
            f.sort_dir = Some("asc".into());
        }
        f
    }

    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ComputeFlavorApiRequest::ListDetailed(Box::new(
            filter.clone(),
        )))
    }

    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Flavor(inner))
                if matches!(&**inner, ComputeFlavorApiRequest::ListDetailed(_))
        )
    }

    fn filter_carry_action(
        action: &Action,
        selected: Option<&Self::Item>,
        _filter: &Self::Filter,
    ) -> Vec<Action> {
        if let Action::ShowComputeServersWithFlavor = action
            && let Some(sel) = selected
            && let Ok(server_list) = ComputeServerListBuilder::default()
                .flavor(sel.id.clone())
                .build()
        {
            return vec![
                Action::Mode {
                    mode: Mode::ComputeServers,
                    stack: true,
                },
                Action::SetComputeServerListFilters(Box::new(server_list)),
            ];
        }
        Vec::new()
    }

    fn handle_mutation_response(
        request: &ApiRequest,
        data: &serde_json::Value,
    ) -> Option<Vec<Mutation>> {
        let _ = (request, data);
        None
    }
}

pub type ComputeFlavors = GenericResourceView<'static, ComputeFlavorsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::ComputeServerApiRequest;
    use crate::components::resource_behaviour::ResourceBehaviour;
    use openstack_types::compute::v2::flavor::response::list_detailed_255::FlavorResponse;

    fn make_flavor(id: &str) -> FlavorResponse {
        let json = serde_json::json!({
            "id": id,
            "name": "test",
            "vcpus": 1,
            "ram": 512,
            "disk": 10,
            "OS-FLV-DISABLED:disabled": false,
            "rxtx_factor": 1,
            "swap": 0,
            "OS-FLV-EXT-DATA:ephemeral": 0,
            "metadata": {},
            "os-flavor-access:is_public": true,
        });
        serde_json::from_value(json).unwrap()
    }

    #[test]
    fn normalise_filter_sets_defaults() {
        let filter = ComputeFlavorList::default();
        let norm = ComputeFlavorsBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some("name".into()));
        assert_eq!(norm.sort_dir, Some("asc".into()));
    }

    #[test]
    fn normalise_filter_preserves_existing() {
        let mut filter = ComputeFlavorList::default();
        filter.sort_key = Some("id".into());
        let norm = ComputeFlavorsBehaviour::normalise_filter(filter);
        assert_eq!(norm.sort_key, Some("id".into()));
    }

    #[test]
    fn view_key_and_title() {
        assert_eq!(ComputeFlavorsBehaviour::view_key(), "compute.flavor");
        assert_eq!(ComputeFlavorsBehaviour::title(), "Compute Flavors");
        assert_eq!(ComputeFlavorsBehaviour::mode(), Mode::ComputeFlavors);
    }

    #[test]
    fn request_from_filter_creates_list_request() {
        let filter = ComputeFlavorList::default();
        let request = ComputeFlavorsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Flavor(boxreq))
                if matches!(*boxreq, ComputeFlavorApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_list_detailed() {
        let filter = ComputeFlavorList::default();
        let request = ComputeFlavorsBehaviour::request_from_filter(&filter);
        assert!(ComputeFlavorsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_non_matching() {
        let filter = ComputeFlavorList::default();
        let request = ComputeFlavorsBehaviour::request_from_filter(&filter);
        assert!(!ComputeFlavorsBehaviour::matches_request(
            &ApiRequest::Compute(ComputeApiRequest::Server(Box::new(
                ComputeServerApiRequest::ListDetailed(Box::new(
                    crate::cloud_worker::compute::v2::ComputeServerList::default(),
                ))
            )))
        ));
    }

    #[test]
    fn filter_carry_action_show_servers_with_flavor() {
        let flavor = make_flavor("flavor-123");
        let actions = ComputeFlavorsBehaviour::filter_carry_action(
            &Action::ShowComputeServersWithFlavor,
            Some(&flavor),
            &ComputeFlavorList::default(),
        );
        assert_eq!(actions.len(), 2);
        match &actions[0] {
            Action::Mode { mode, stack } => {
                assert_eq!(*mode, crate::mode::Mode::ComputeServers);
                assert!(*stack);
            }
            _ => panic!("Expected Mode switch, got {:?}", actions[0]),
        }
        match &actions[1] {
            Action::SetComputeServerListFilters(f) => {
                assert_eq!(f.flavor, Some("flavor-123".into()));
            }
            _ => panic!("Expected SetComputeServerListFilters, got {:?}", actions[1]),
        }
    }

    #[test]
    fn filter_carry_action_no_selected() {
        let actions = ComputeFlavorsBehaviour::filter_carry_action(
            &Action::ShowComputeServersWithFlavor,
            None,
            &ComputeFlavorList::default(),
        );
        assert!(actions.is_empty());
    }

    #[test]
    fn filter_carry_action_unrelated_action() {
        let flavor = make_flavor("f");
        let actions = ComputeFlavorsBehaviour::filter_carry_action(
            &Action::Tick,
            Some(&flavor),
            &ComputeFlavorList::default(),
        );
        assert!(actions.is_empty());
    }
}
