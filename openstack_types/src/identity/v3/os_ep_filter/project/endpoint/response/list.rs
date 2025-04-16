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
//! Response type for the GET `OS-EP-FILTER/projects/{project_id}/endpoints` operation

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use structable::{StructTable, StructTableOptions};

/// Response data as HashMap type
#[derive(Deserialize, Serialize)]
pub struct EndpointResponse(BTreeMap<String, Value>);

impl StructTable for EndpointResponse {
    fn instance_headers<O: StructTableOptions>(&self, _options: &O) -> Option<Vec<String>> {
        Some(self.0.keys().map(Into::into).collect())
    }

    fn data<O: StructTableOptions>(&self, _options: &O) -> Vec<Option<String>> {
        Vec::from_iter(self.0.values().map(|v| serde_json::to_string(&v).ok()))
    }
}

impl StructTable for &EndpointResponse {
    fn instance_headers<O: StructTableOptions>(&self, _options: &O) -> Option<Vec<String>> {
        Some(self.0.keys().map(Into::into).collect())
    }

    fn data<O: StructTableOptions>(&self, _options: &O) -> Vec<Option<String>> {
        Vec::from_iter(self.0.values().map(|v| serde_json::to_string(&v).ok()))
    }
}
