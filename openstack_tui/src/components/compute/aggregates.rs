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

use crate::cloud_worker::compute::v2::{
    ComputeAggregateApiRequest, ComputeAggregateList, ComputeApiRequest,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;
use openstack_types::compute::v2::aggregate::response::list_241::AggregateResponse;

/// Behaviour implementation for ComputeAggregates.
pub struct ComputeAggregatesBehaviour;

impl ResourceBehaviour for ComputeAggregatesBehaviour {
    type Item = AggregateResponse;
    type Filter = ComputeAggregateList;

    fn view_key() -> &'static str {
        "compute.aggregate"
    }
    fn title() -> &'static str {
        "Compute Aggregates"
    }
    fn mode() -> Mode {
        Mode::ComputeAggregates
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ComputeAggregateApiRequest::List(Box::new(filter.clone())))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Aggregate(boxreq))
            if matches!(**boxreq, ComputeAggregateApiRequest::List(_))
        )
    }
}

/// Public component for ComputeAggregates using the generic view.
pub type ComputeAggregates = GenericResourceView<'static, ComputeAggregatesBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(ComputeAggregatesBehaviour::view_key(), "compute.aggregate");
        assert_eq!(ComputeAggregatesBehaviour::title(), "Compute Aggregates");
        assert_eq!(ComputeAggregatesBehaviour::mode(), Mode::ComputeAggregates);
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
            crate::cloud_worker::types::ComputeHypervisorApiRequest::ListDetailed(Box::new(
                crate::cloud_worker::types::ComputeHypervisorList::default(),
            )),
        )));
        assert!(!ComputeAggregatesBehaviour::matches_request(&req));
    }
}
