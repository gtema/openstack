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
//! Response type for the PUT `subnetpools/{id}` operation

use crate::common::deser_bool_str_opt;
use crate::common::deser_num_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Subnetpool response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct SubnetpoolResponse {
    /// An address scope to assign to the subnet pool.
    ///
    #[structable(optional)]
    pub address_scope_id: Option<String>,

    /// Time at which the resource has been created (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The size of the prefix to allocate when the `cidr` or `prefixlen`
    /// attributes are omitted when you create the subnet. Default is
    /// `min_prefixlen`.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub default_prefixlen: Option<i64>,

    /// A per-project quota on the prefix space that can be allocated from the
    /// subnet pool for project subnets. Default is no quota is enforced on
    /// allocations from the subnet pool. For IPv4 subnet pools,
    /// `default_quota` is measured in units of /32. For IPv6 subnet pools,
    /// `default_quota` is measured units of /64. All projects that use the
    /// subnet pool have the same prefix quota applied.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub default_quota: Option<i64>,

    /// A human-readable description for the resource.
    ///
    #[structable(optional)]
    pub description: Option<String>,

    /// The ID of the subnet pool.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The IP protocol version. Valid value is `4` or `6`. Default is `4`.
    ///
    #[structable(optional)]
    pub ip_version: Option<i32>,

    /// The subnetpool is default pool or not.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub is_default: Option<bool>,

    /// The maximum prefix size that can be allocated from the subnet pool. For
    /// IPv4 subnet pools, default is `32`. For IPv6 subnet pools, default is
    /// `128`.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub max_prefixlen: Option<i64>,

    /// The smallest prefix that can be allocated from a subnet pool. For IPv4
    /// subnet pools, default is `8`. For IPv6 subnet pools, default is `64`.
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub min_prefixlen: Option<i64>,

    /// Human-readable name of the resource.
    ///
    #[structable(optional)]
    pub name: Option<String>,

    /// A list of subnet prefixes to assign to the subnet pool. The API merges
    /// adjacent prefixes and treats them as a single prefix. Each subnet
    /// prefix must be unique among all subnet prefixes in all subnet pools
    /// that are associated with the address scope.
    ///
    #[structable(optional, serialize)]
    pub prefixes: Option<Vec<String>>,

    /// The revision number of the resource.
    ///
    #[structable(optional)]
    pub revision_number: Option<i32>,

    /// Indicates whether this resource is shared across all projects. By
    /// default, only administrative users can change this value.
    ///
    #[serde(deserialize_with = "deser_bool_str_opt")]
    #[structable(optional)]
    pub shared: Option<bool>,

    /// The list of tags on the resource.
    ///
    #[structable(optional, serialize)]
    pub tags: Option<Vec<String>>,

    /// The ID of the project.
    ///
    #[structable(optional)]
    pub tenant_id: Option<String>,

    /// Time at which the resource has been updated (in UTC ISO8601 format).
    ///
    #[structable(optional)]
    pub updated_at: Option<String>,
}
