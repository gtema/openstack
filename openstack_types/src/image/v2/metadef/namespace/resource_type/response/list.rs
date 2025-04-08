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
//! Response type for the get metadefs/namespaces/{namespace_name}/resource_types operation

use serde::{Deserialize, Serialize};

/// ResourceType response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ResourceTypeResponse {
    /// Date and time of resource type association
    ///
    created_at: Option<String>,

    /// Resource type names should be aligned with Heat resource types whenever
    /// possible:
    /// https://docs.openstack.org/heat/latest/template_guide/openstack.html
    ///
    name: String,

    /// Specifies the prefix to use for the given resource type. Any properties
    /// in the namespace should be prefixed with this prefix when being applied
    /// to the specified resource type. Must include prefix separator (e.g. a
    /// colon :).
    ///
    prefix: Option<String>,

    /// Some resource types allow more than one key / value pair per instance.
    /// For example, Cinder allows user and image metadata on volumes. Only the
    /// image properties metadata is evaluated by Nova (scheduling or drivers).
    /// This property allows a namespace target to remove the ambiguity.
    ///
    properties_target: Option<String>,

    /// Date and time of the last resource type association modification
    ///
    updated_at: Option<String>,
}
