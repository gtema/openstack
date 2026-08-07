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

//! Hand-written [`HasStatusPointer`] impls for server endpoints. Not generated — codegen support
//! for emitting these is tracked separately; this file is safe from being overwritten by a
//! codegen run.

use openstack_sdk_core::api::HasStatusPointer;
use openstack_sdk_core::types::ApiVersion;

use super::get;

impl HasStatusPointer for get::Request<'_> {
    fn status_pointer(_negotiated: Option<ApiVersion>) -> &'static str {
        "/server/status"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_get_status_pointer_is_server_status() {
        assert_eq!(get::Request::status_pointer(None), "/server/status");
    }
}
