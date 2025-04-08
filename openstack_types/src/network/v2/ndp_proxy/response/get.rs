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
//! Response type for the get ndp-proxies/{id} operation

use serde::{Deserialize, Serialize};

/// NdpProxy response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct NdpProxyResponse {
    created_at: Option<String>,

    description: Option<String>,

    id: Option<String>,

    ip_address: Option<String>,

    name: Option<String>,

    port_id: Option<String>,

    project_id: Option<String>,

    revision_number: Option<i32>,

    router_id: Option<String>,

    updated_at: Option<String>,
}
