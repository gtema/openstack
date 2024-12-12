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

use serde::{Deserialize, Serialize};

pub use crate::cloud_worker::compute::aggregate::*;
pub use crate::cloud_worker::compute::flavor::*;
pub use crate::cloud_worker::compute::hypervisor::*;
pub use crate::cloud_worker::compute::quota_set::*;
pub use crate::cloud_worker::compute::server::*;
use crate::cloud_worker::ConfirmableRequest;

/// Compute operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeApiRequest {
    /// Aggregate resource
    Aggregate(ComputeAggregateApiRequest),
    /// Flavor resource
    Flavor(ComputeFlavorApiRequest),
    /// Hypervisor
    Hypervisor(ComputeHypervisorApiRequest),
    /// Compute quota
    QuotaSet(ComputeQuotaSetApiRequest),
    /// Compute server
    Server(ComputeServerApiRequest),
}

impl From<ComputeAggregateApiRequest> for ComputeApiRequest {
    fn from(item: ComputeAggregateApiRequest) -> Self {
        ComputeApiRequest::Aggregate(item)
    }
}

impl From<ComputeFlavorApiRequest> for ComputeApiRequest {
    fn from(item: ComputeFlavorApiRequest) -> Self {
        ComputeApiRequest::Flavor(item)
    }
}

impl From<ComputeHypervisorApiRequest> for ComputeApiRequest {
    fn from(item: ComputeHypervisorApiRequest) -> Self {
        ComputeApiRequest::Hypervisor(item)
    }
}

impl From<ComputeQuotaSetApiRequest> for ComputeApiRequest {
    fn from(item: ComputeQuotaSetApiRequest) -> Self {
        ComputeApiRequest::QuotaSet(item)
    }
}

impl From<ComputeServerApiRequest> for ComputeApiRequest {
    fn from(item: ComputeServerApiRequest) -> Self {
        ComputeApiRequest::Server(item)
    }
}

impl ConfirmableRequest for ComputeApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            ComputeApiRequest::Server(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}
