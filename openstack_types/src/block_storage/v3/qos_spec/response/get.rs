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
//! Response type for the get qos-specs/{id} operation

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// QosSpec response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct QosSpecResponse {
    /// The consumer type.
    ///
    pub consumer: Option<String>,

    /// The generated ID for the QoS specification.
    ///
    pub id: Option<String>,

    /// The name of the QoS specification.
    ///
    pub name: Option<String>,

    /// A `specs` object.
    ///
    pub specs: Option<HashMap<String, Option<String>>>,
}
