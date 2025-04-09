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
//! Response type for the post os-server-groups operation

use crate::common::IntString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ServerGroup response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServerGroupResponse {
    /// The UUID of the server group.
    ///
    pub id: String,

    /// A list of members in the server group.
    ///
    pub members: Option<Vec<String>>,

    /// Metadata key and value pairs. The maximum size for each metadata key
    /// and value pair is 255 bytes. It’s always empty and only used for
    /// keeping compatibility.
    ///
    /// **Available until version 2.63**
    ///
    pub metadata: Option<HashMap<String, String>>,

    /// The name of the server group.
    ///
    pub name: String,

    /// A list of exactly one policy name to associate with the server group.
    /// The current valid policy names are:
    ///
    /// - `anti-affinity` - servers in this group must be scheduled to
    ///   different hosts.
    /// - `affinity` - servers in this group must be scheduled to the same
    ///   host.
    /// - `soft-anti-affinity` - servers in this group should be scheduled to
    ///   different hosts if possible, but if not possible then they should
    ///   still be scheduled instead of resulting in a build failure. This
    ///   policy was added in microversion 2.15.
    /// - `soft-affinity` - servers in this group should be scheduled to the
    ///   same host if possible, but if not possible then they should still be
    ///   scheduled instead of resulting in a build failure. This policy was
    ///   added in microversion 2.15.
    ///
    /// **Available until version 2.63**
    ///
    pub policies: Option<Vec<Policies>>,

    /// The `policy` field represents the name of the policy. The current valid
    /// policy names are:
    ///
    /// - `anti-affinity` - servers in this group must be scheduled to
    ///   different hosts.
    /// - `affinity` - servers in this group must be scheduled to the same
    ///   host.
    /// - `soft-anti-affinity` - servers in this group should be scheduled to
    ///   different hosts if possible, but if not possible then they should
    ///   still be scheduled instead of resulting in a build failure.
    /// - `soft-affinity` - servers in this group should be scheduled to the
    ///   same host if possible, but if not possible then they should still be
    ///   scheduled instead of resulting in a build failure.
    ///
    /// **New in version 2.64**
    ///
    pub policy: Policy,

    /// The project ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    pub project_id: String,

    /// The `rules` field, which is a dict, can be applied to the policy.
    /// Currently, only the `max_server_per_host` rule is supported for the
    /// `anti-affinity` policy. The `max_server_per_host` rule allows
    /// specifying how many members of the anti-affinity group can reside on
    /// the same compute host. If not specified, only one member from the same
    /// anti-affinity group can reside on a given host.
    ///
    /// **New in version 2.64**
    ///
    pub rules: Option<Rules>,

    /// The user ID who owns the server group.
    ///
    /// **New in version 2.13**
    ///
    pub user_id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Policies {
    // Affinity
    #[serde(rename = "affinity")]
    Affinity,

    // AntiAffinity
    #[serde(rename = "anti-affinity")]
    AntiAffinity,

    // SoftAffinity
    #[serde(rename = "soft-affinity")]
    SoftAffinity,

    // SoftAntiAffinity
    #[serde(rename = "soft-anti-affinity")]
    SoftAntiAffinity,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Policy {
    // Affinity
    #[serde(rename = "affinity")]
    Affinity,

    // AntiAffinity
    #[serde(rename = "anti-affinity")]
    AntiAffinity,

    // SoftAffinity
    #[serde(rename = "soft-affinity")]
    SoftAffinity,

    // SoftAntiAffinity
    #[serde(rename = "soft-anti-affinity")]
    SoftAntiAffinity,
}

/// The `rules` field, which is a dict, can be applied to the policy.
/// Currently, only the `max_server_per_host` rule is supported for the
/// `anti-affinity` policy. The `max_server_per_host` rule allows specifying
/// how many members of the anti-affinity group can reside on the same compute
/// host. If not specified, only one member from the same anti-affinity group
/// can reside on a given host.
///
/// **New in version 2.64**
///
/// `Rules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rules {
    pub max_server_per_host: Option<IntString>,
}
