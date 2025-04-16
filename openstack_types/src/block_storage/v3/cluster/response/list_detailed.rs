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
//! Response type for the GET `clusters/detail` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// Cluster response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct ClusterResponse {
    /// The ID of active storage backend. Only in `cinder-volume` service.
    ///
    /// **New in version 3.26**
    #[serde(default)]
    #[structable(optional)]
    pub active_backend_id: Option<String>,

    /// The binary name of the services in the cluster.
    #[serde(default)]
    #[structable(optional)]
    pub binary: Option<String>,

    /// The date and time when the resource was created.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    #[serde(default)]
    #[structable(optional)]
    pub created_at: Option<String>,

    /// The reason for disabling a resource.
    #[serde(default)]
    #[structable(optional)]
    pub disabled_reason: Option<String>,

    /// Whether the cluster is frozen or not.
    ///
    /// **New in version 3.26**
    #[serde(default)]
    #[structable(optional)]
    pub frozen: Option<bool>,

    /// The last periodic heartbeat received.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC.
    #[serde(default)]
    #[structable(optional, wide)]
    pub last_heartbeat: Option<String>,

    /// The name of the service cluster.
    #[serde(default)]
    #[structable(optional)]
    pub name: Option<String>,

    /// The number of down hosts in the cluster.
    #[serde(default)]
    #[structable(optional, wide)]
    pub num_down_hosts: Option<i32>,

    /// The number of hosts in the cluster.
    #[serde(default)]
    #[structable(optional, wide)]
    pub num_hosts: Option<i32>,

    /// The cluster replication status. Only included in responses if
    /// configured. One of: `enabled` or `disabled`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub replication_status: Option<ReplicationStatus>,

    /// The state of the cluster. One of `up` or `down`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub state: Option<State>,

    /// The status of the cluster. One of `enabled` or `disabled`.
    #[serde(default)]
    #[structable(optional, serialize)]
    pub status: Option<Status>,

    /// The date and time when the resource was updated.
    ///
    /// The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset from
    /// UTC. In the previous example, the offset value is `-05:00`.
    ///
    /// If the `updated_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum ReplicationStatus {
    // Disabled
    #[serde(rename = "disabled")]
    Disabled,

    // Enabled
    #[serde(rename = "enabled")]
    Enabled,
}

impl std::str::FromStr for ReplicationStatus {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum State {
    // Down
    #[serde(rename = "down")]
    Down,

    // Up
    #[serde(rename = "up")]
    Up,
}

impl std::str::FromStr for State {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Status {
    // Disabled
    #[serde(rename = "disabled")]
    Disabled,

    // Enabled
    #[serde(rename = "enabled")]
    Enabled,
}

impl std::str::FromStr for Status {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "disabled" => Ok(Self::Disabled),
            "enabled" => Ok(Self::Enabled),
            _ => Err(()),
        }
    }
}
