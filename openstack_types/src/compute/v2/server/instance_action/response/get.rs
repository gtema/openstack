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
//! Response type for the GET `servers/{server_id}/os-instance-actions/{id}` operation

use serde::{Deserialize, Serialize};
use structable::{StructTable, StructTableOptions};

/// InstanceAction response representation
#[derive(Clone, Deserialize, Serialize, StructTable)]
pub struct InstanceActionResponse {
    /// The name of the action.
    #[structable()]
    pub action: String,

    /// The events which occurred in this action in descending order of
    /// creation.
    ///
    /// Policy defaults enable only users with the administrative role or the
    /// owner of the server to see instance action event information. Cloud
    /// providers can change these permissions through the `policy.json` file.
    ///
    /// **New in version 2.51**
    #[structable(serialize)]
    pub events: Vec<Events>,

    /// The UUID of the server.
    #[serde(default)]
    #[structable(optional)]
    pub instance_uuid: Option<String>,

    /// The related error message for when an action fails.
    #[serde(default)]
    #[structable(optional)]
    pub message: Option<String>,

    /// The ID of the project which initiated the server action. This can be
    /// `null` for `nova-manage`-initiated actions.
    #[structable()]
    pub project_id: String,

    /// The request id generated when execute the API of this action.
    #[structable()]
    pub request_id: String,

    /// The date and time when the action was started. The date and time stamp
    /// format is [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    #[serde(default)]
    #[structable(optional)]
    pub start_time: Option<String>,

    /// The date and time when the instance action or the action event of
    /// instance action was updated. The date and time stamp format is
    /// [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601)
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    /// For example, `2015-08-27T09:49:58-05:00`. The `±hh:mm` value, if
    /// included, is the time zone as an offset from UTC. In the previous
    /// example, the offset value is `-05:00`.
    ///
    /// **New in version 2.58**
    #[serde(default)]
    #[structable(optional)]
    pub updated_at: Option<String>,

    /// The ID of the user which initiated the server action. This can be
    /// `null` for `nova-manage`-initiated actions.
    #[structable(optional)]
    pub user_id: Option<String>,
}

/// Event
/// `Events` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Events {
    #[serde(default)]
    pub details: Option<String>,
    pub event: String,
    #[serde(default)]
    pub finish_time: Option<String>,
    #[serde(default)]
    pub host: Option<String>,
    #[serde(default, rename = "hostId")]
    pub host_id: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub traceback: Option<String>,
}
