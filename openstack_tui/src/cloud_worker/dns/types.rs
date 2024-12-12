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

use crate::cloud_worker::common::ConfirmableRequest;
pub use crate::cloud_worker::dns::recordset::*;
pub use crate::cloud_worker::dns::zone::*;

/// DNS operations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DnsApiRequest {
    Recordset(DnsRecordsetApiRequest),
    Zone(DnsZoneApiRequest),
}

impl From<DnsRecordsetApiRequest> for DnsApiRequest {
    fn from(item: DnsRecordsetApiRequest) -> Self {
        DnsApiRequest::Recordset(item)
    }
}

impl From<DnsZoneApiRequest> for DnsApiRequest {
    fn from(item: DnsZoneApiRequest) -> Self {
        DnsApiRequest::Zone(item)
    }
}

impl ConfirmableRequest for DnsApiRequest {
    fn get_confirm_message(&self) -> Option<String> {
        match &self {
            DnsApiRequest::Zone(req) => req.get_confirm_message(),
            _ => None,
        }
    }
}
