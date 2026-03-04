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

//! Identity v3 data types
pub mod version;

pub use openstack_sdk_auth_core::types::*;

/// Build Domain type if id or name are given
#[inline]
pub(crate) fn get_domain(id: Option<String>, name: Option<String>) -> Option<Domain> {
    if id.is_some() || name.is_some() {
        Some(Domain {
            id: id.clone(),
            name: name.clone(),
        })
    } else {
        None
    }
}
