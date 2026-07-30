// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use crate::action::Action;
use crate::cloud_worker::compute::v2::ComputeAggregateList;
use crate::cloud_worker::types::ApiRequest;
use crate::components::compute::generated::aggregate::AggregateItem;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

impl crate::utils::ResourceKey for AggregateItem {
    fn get_key() -> &'static str {
        crate::mode::COMPUTE_AGGREGATE
    }
}

/// Behaviour implementation for ComputeAggregates.
pub struct ComputeAggregatesBehaviour;

impl ResourceBehaviour for ComputeAggregatesBehaviour {
    type Item = AggregateItem;
    type Filter = ComputeAggregateList;

    fn view_key() -> &'static str {
        super::generated::aggregate::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::aggregate::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::aggregate::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::aggregate::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::aggregate::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::aggregate::Generated::handle_set_filter_action(action)
    }
    fn deserialize_items(
        data: &[serde_json::Value],
        negotiated_version: Option<openstack_sdk::types::ApiVersion>,
    ) -> serde_json::Result<Vec<Self::Item>> {
        super::generated::aggregate::Generated::deserialize_items(data, negotiated_version)
    }
}

/// Public component for ComputeAggregates using the generic view.
pub type ComputeAggregates = GenericResourceView<'static, ComputeAggregatesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::{ComputeAggregateApiRequest, ComputeApiRequest};
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(ComputeAggregatesBehaviour::view_key(), "compute.aggregate");
        assert_eq!(ComputeAggregatesBehaviour::title(), "Aggregates");
        assert_eq!(
            ComputeAggregatesBehaviour::mode(),
            Mode::Resource(crate::mode::COMPUTE_AGGREGATE)
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = ComputeAggregateList::default();
        let request = ComputeAggregatesBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Aggregate(boxreq))
            if matches!(*boxreq, ComputeAggregateApiRequest::List(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = ComputeAggregateList::default();
        let request = ComputeAggregatesBehaviour::request_from_filter(&filter);
        assert!(ComputeAggregatesBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Compute(ComputeApiRequest::Hypervisor(Box::new(
            crate::cloud_worker::types::ComputeHypervisorApiRequest::ListDetailed(Box::default()),
        )));
        assert!(!ComputeAggregatesBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = ComputeAggregateList::default();
        let action = Action::SetComputeAggregateListFilters(filter);
        let result = ComputeAggregatesBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = ComputeAggregatesBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
