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
//! Response type for the GET `os-quota-sets/{id}` operation

use crate::common::deser_num_str_opt;
use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// QuotaSet response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct QuotaSetResponse {
    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub cores: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub fixed_ips: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub floating_ips: Option<i64>,

    /// The UUID of the tenant/user the quotas listed for.
    ///
    #[structable(optional)]
    pub id: Option<String>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub injected_file_content_bytes: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub injected_file_path_bytes: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub injected_files: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub instances: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub key_pairs: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub metadata_items: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub networks: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub ram: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub security_group_rules: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub security_groups: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub server_group_members: Option<i64>,

    /// The number of allowed injected files for the quota class.
    ///
    /// **Available until version 2.56**
    ///
    #[serde(deserialize_with = "deser_num_str_opt")]
    #[structable(optional)]
    pub server_groups: Option<i64>,
}
