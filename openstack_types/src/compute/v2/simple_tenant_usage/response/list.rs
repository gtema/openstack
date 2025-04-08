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
//! Response type for the get os-simple-tenant-usage operation

use serde::{Deserialize, Serialize};

/// SimpleTenantUsage response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct SimpleTenantUsageResponse {
    /// A list of the tenant usage objects.
    ///
    tenant_usages: Option<Vec<TenantUsages>>,

    /// Links pertaining to usage. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    /// **New in version 2.40**
    ///
    tenant_usages_links: Option<Vec<TenantUsagesLinks>>,
}

/// `ServerUsages` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerUsages {
    ended_at: Option<String>,
    flavor: Option<String>,
    hours: Option<f32>,
    instance_id: Option<String>,
    local_gb: Option<i32>,
    memory_mb: Option<i32>,
    name: Option<String>,
    started_at: Option<String>,
    state: Option<String>,
    tenant_id: Option<String>,
    uptime: Option<i32>,
    vcpus: Option<i32>,
}

/// `TenantUsages` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantUsages {
    server_usages: Option<Vec<ServerUsages>>,
    start: Option<String>,
    stop: Option<String>,
    tenant_id: Option<String>,
    total_hours: Option<f32>,
    total_local_gb_usage: Option<f32>,
    total_memory_mb_usage: Option<f32>,
    total_vcpus_usage: Option<f32>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `TenantUsagesLinks` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantUsagesLinks {
    href: Option<String>,
    rel: Option<String>,
}
