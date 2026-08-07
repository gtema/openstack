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
use crate::cloud_worker::compute::v2::ComputeHypervisorList;
use crate::cloud_worker::types::ApiRequest;
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::{GeneratedResourceBehaviour, ResourceBehaviour};
use crate::mode::Mode;

/// Behaviour implementation for ComputeHypervisors.
pub struct ComputeHypervisorsBehaviour;

impl ResourceBehaviour for ComputeHypervisorsBehaviour {
    type Filter = ComputeHypervisorList;

    fn view_key() -> &'static str {
        super::generated::hypervisor::Generated::view_key()
    }
    fn title() -> &'static str {
        super::generated::hypervisor::Generated::title()
    }
    fn mode() -> Mode {
        super::generated::hypervisor::Generated::mode()
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        super::generated::hypervisor::Generated::request_from_filter(filter)
    }
    fn matches_request(request: &ApiRequest) -> bool {
        super::generated::hypervisor::Generated::matches_request(request)
    }
    fn handle_set_filter_action(action: &Action) -> Option<Self::Filter> {
        super::generated::hypervisor::Generated::handle_set_filter_action(action)
    }
}

/// Public component for ComputeHypervisors using the generic view.
pub type ComputeHypervisors = GenericResourceView<'static, ComputeHypervisorsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cloud_worker::compute::v2::{ComputeApiRequest, ComputeHypervisorApiRequest};
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            ComputeHypervisorsBehaviour::view_key(),
            "compute.hypervisor"
        );
        assert_eq!(ComputeHypervisorsBehaviour::title(), "Hypervisors");
        assert_eq!(
            ComputeHypervisorsBehaviour::mode(),
            Mode::Resource(crate::mode::COMPUTE_HYPERVISOR)
        );
    }

    #[test]
    fn request_from_filter_creates_request() {
        let filter = ComputeHypervisorList::default();
        let request = ComputeHypervisorsBehaviour::request_from_filter(&filter);
        assert!(matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Hypervisor(boxreq))
            if matches!(*boxreq, ComputeHypervisorApiRequest::ListDetailed(_))
        ));
    }

    #[test]
    fn matches_request_returns_true_for_matching() {
        let filter = ComputeHypervisorList::default();
        let request = ComputeHypervisorsBehaviour::request_from_filter(&filter);
        assert!(ComputeHypervisorsBehaviour::matches_request(&request));
    }

    #[test]
    fn matches_request_returns_false_for_unrelated() {
        let req = ApiRequest::Compute(ComputeApiRequest::Flavor(Box::new(
            crate::cloud_worker::compute::v2::ComputeFlavorApiRequest::ListDetailed(Box::default()),
        )));
        assert!(!ComputeHypervisorsBehaviour::matches_request(&req));
    }

    #[test]
    fn handle_set_filter_action_returns_filter() {
        let filter = ComputeHypervisorList::default();
        let action = Action::SetComputeHypervisorListFilters(filter);
        let result = ComputeHypervisorsBehaviour::handle_set_filter_action(&action);
        assert!(result.is_some());
    }

    #[test]
    fn handle_set_filter_action_returns_none_for_unrelated() {
        let result = ComputeHypervisorsBehaviour::handle_set_filter_action(&Action::Tick);
        assert!(result.is_none());
    }
}
