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
use structable_derive::StructTable;

use crate::common::{OutputConfig, StructTable};

/// SimpleTenantUsage response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SimpleTenantUsageResponse {
    /// A list of the tenant usage objects.
    ///
    #[structable(optional, serialize)]
    pub tenant_usages: Option<Vec<TenantUsages>>,

    /// Links pertaining to usage. See
    /// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
    /// for more info.
    ///
    /// **New in version 2.40**
    ///
    #[structable(optional, serialize)]
    pub tenant_usages_links: Option<Vec<TenantUsagesLinks>>,
}

/// `ServerUsages` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerUsages {
    pub ended_at: Option<String>,
    pub flavor: Option<String>,
    pub hours: Option<f32>,
    pub instance_id: Option<String>,
    pub local_gb: Option<i32>,
    pub memory_mb: Option<i32>,
    pub name: Option<String>,
    pub started_at: Option<String>,
    pub state: Option<String>,
    pub tenant_id: Option<String>,
    pub uptime: Option<i32>,
    pub vcpus: Option<i32>,
}

/// `TenantUsages` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantUsages {
    pub server_usages: Option<Vec<ServerUsages>>,
    pub start: Option<String>,
    pub stop: Option<String>,
    pub tenant_id: Option<String>,
    pub total_hours: Option<f32>,
    pub total_local_gb_usage: Option<f32>,
    pub total_memory_mb_usage: Option<f32>,
    pub total_vcpus_usage: Option<f32>,
}

/// Links to the resources in question. See
/// [API Guide / Links and References](https://docs.openstack.org/api-guide/compute/links_and_references.html)
/// for more info.
///
/// `TenantUsagesLinks` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TenantUsagesLinks {
    pub href: Option<String>,
    pub rel: Option<String>,
}
