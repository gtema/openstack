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

//! Block Storage API types

pub mod v3;

/// Get OpenAPI spec for the block-storage service used during the codegeneration.
#[cfg(feature = "openapi")]
pub fn get_openapi_spec() -> &'static str {
    include_str!("../data/block-storage/v3.yaml")
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "openapi")]
    #[test]
    fn test_get_openapi_spec() {
        assert!(!super::get_openapi_spec().is_empty());
    }
}
