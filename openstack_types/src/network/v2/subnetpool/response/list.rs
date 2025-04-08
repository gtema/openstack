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
//! Response type for the get subnetpools operation

use crate::common::BoolString;
use crate::common::IntString;
use serde::{Deserialize, Serialize};

/// Subnetpool response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct SubnetpoolResponse {
    /// An address scope to assign to the subnet pool.
    ///
    address_scope_id: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    created_at: Option<String>,

    /// The size of the prefix to allocate when the `cidr` or `prefixlen`
    /// attributes are omitted when you create the subnet. Default is
    /// `min_prefixlen`.
    ///
    default_prefixlen: Option<IntString>,

    /// A per-project quota on the prefix space that can be allocated from the
    /// subnet pool for project subnets. Default is no quota is enforced on
    /// allocations from the subnet pool. For IPv4 subnet pools,
    /// `default_quota` is measured in units of /32. For IPv6 subnet pools,
    /// `default_quota` is measured units of /64. All projects that use the
    /// subnet pool have the same prefix quota applied.
    ///
    default_quota: Option<IntString>,

    /// A human-readable description for the resource.
    ///
    description: Option<String>,

    /// The ID of the subnet pool.
    ///
    id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    ip_version: Option<i32>,

    /// The subnetpool is default pool or not.
    ///
    is_default: Option<BoolString>,

    /// The maximum prefix size that can be allocated from the subnet pool. For
    /// IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is
    /// `128`.
    ///
    max_prefixlen: Option<IntString>,

    /// The smallest prefix that can be allocated from a subnet pool. For IPv4
    /// subnet pools, default is `8`. For IPv6 subnet pools, default is `64`.
    ///
    min_prefixlen: Option<IntString>,

    /// Human-readable name of the resource.
    ///
    name: Option<String>,

    /// A list of subnet prefixes to assign to the subnet pool. The API merges
    /// adjacent prefixes and treats them as a single prefix. Each subnet
    /// prefix must be unique among all subnet prefixes in all subnet pools
    /// that are associated with the address scope.
    ///
    prefixes: Option<Vec<String>>,

    /// The revision number of the resource.
    ///
    revision_number: Option<i32>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    ///
    shared: Option<BoolString>,

    /// The list of tags on the resource.
    ///
    tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    updated_at: Option<String>,
}
