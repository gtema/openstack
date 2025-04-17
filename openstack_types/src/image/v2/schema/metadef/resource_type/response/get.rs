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
//
// WARNING: This file is automatically generated from OpenAPI schema using
// `openstack-codegenerator`.
//! Response type for the GET `schemas/metadefs/resource_type` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// ResourceType response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ResourceTypeResponse(String);

impl StructTable for ResourceTypeResponse {
    fn class_headers<O: StructTableOptions>(_options: &O) -> Option<Vec<String>> {
        Some(Vec::from(["Value".to_string()]))
    }

    fn data<O: StructTableOptions>(
        &self,
        _options: &O,
    ) -> ::std::vec::Vec<Option<::std::string::String>> {
        Vec::from([Some(self.0.to_string())])
    }
}

impl StructTable for &ResourceTypeResponse {
    fn class_headers<O: StructTableOptions>(_options: &O) -> Option<Vec<String>> {
        Some(Vec::from(["Value".to_string()]))
    }

    fn data<O: StructTableOptions>(
        &self,
        _options: &O,
    ) -> ::std::vec::Vec<Option<::std::string::String>> {
        Vec::from([Some(self.0.to_string())])
    }
}
