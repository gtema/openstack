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
    ComputeApiRequest, ComputeHypervisorApiRequest, ComputeHypervisorList,
};
use crate::cloud_worker::types::ApiRequest;
use crate::components::dynamic_item::{ColumnSpec, impl_dynamic_item};
use crate::components::generic_resource_view::GenericResourceView;
use crate::components::resource_behaviour::ResourceBehaviour;
use crate::mode::Mode;

const VIEW_CONFIG_KEY: &str = "compute.hypervisor";

// Hypervisor's `id` field changed type (i32 -> String) at microversion 2.53 -- no single
// `openstack_types` struct correctly represents every microversion, so `Item` reads columns out
// of the raw response by JSON pointer instead of deserializing into a versioned struct.
static HYPERVISOR_COLUMNS: &[ColumnSpec] = &[
    ColumnSpec {
        title: "ID",
        pointer: "/id",
        wide: false,
        status: false,
    },
    ColumnSpec {
        title: "Hostname",
        pointer: "/hypervisor_hostname",
        wide: false,
        status: false,
    },
    ColumnSpec {
        title: "Type",
        pointer: "/hypervisor_type",
        wide: false,
        status: false,
    },
    ColumnSpec {
        title: "State",
        pointer: "/state",
        wide: false,
        status: false,
    },
    ColumnSpec {
        title: "Status",
        pointer: "/status",
        wide: false,
        status: true,
    },
    ColumnSpec {
        title: "VCPUs",
        pointer: "/vcpus",
        wide: true,
        status: false,
    },
    ColumnSpec {
        title: "Memory MB",
        pointer: "/memory_mb",
        wide: true,
        status: false,
    },
];

impl_dynamic_item!(HypervisorItem, VIEW_CONFIG_KEY, HYPERVISOR_COLUMNS);

/// Behaviour implementation for ComputeHypervisors.
pub struct ComputeHypervisorsBehaviour;

impl ResourceBehaviour for ComputeHypervisorsBehaviour {
    type Item = HypervisorItem;
    type Filter = ComputeHypervisorList;

    fn view_key() -> &'static str {
        VIEW_CONFIG_KEY
    }
    fn title() -> &'static str {
        "Compute Hypervisors"
    }
    fn mode() -> Mode {
        Mode::Resource(Self::view_key())
    }
    fn request_from_filter(filter: &Self::Filter) -> ApiRequest {
        ApiRequest::from(ComputeHypervisorApiRequest::ListDetailed(Box::new(
            filter.clone(),
        )))
    }
    fn matches_request(request: &ApiRequest) -> bool {
        matches!(
            request,
            ApiRequest::Compute(ComputeApiRequest::Hypervisor(boxreq))
            if matches!(**boxreq, ComputeHypervisorApiRequest::ListDetailed(_))
        )
    }
}

/// Public component for ComputeHypervisors using the generic view.
pub type ComputeHypervisors = GenericResourceView<'static, ComputeHypervisorsBehaviour>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::resource_behaviour::ResourceBehaviour;

    #[test]
    fn view_key_and_title() {
        assert_eq!(
            ComputeHypervisorsBehaviour::view_key(),
            "compute.hypervisor"
        );
        assert_eq!(ComputeHypervisorsBehaviour::title(), "Compute Hypervisors");
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
}
