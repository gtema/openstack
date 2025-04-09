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
//! Response type for the put lbaas/healthmonitors/{healthmonitor_id} operation

use serde::{Deserialize, Serialize};

/// Healthmonitor response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct HealthmonitorResponse {
    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`).
    ///
    pub admin_state_up: Option<bool>,

    /// The UTC date and timestamp when the resource was created.
    ///
    pub created_at: Option<String>,

    /// The time, in seconds, between sending probes to members.
    ///
    pub delay: Option<i32>,

    /// The domain name, which be injected into the HTTP Host Header to the
    /// backend server for HTTP health check.
    ///
    /// **New in version 2.10**
    ///
    pub domain_name: Option<String>,

    /// The list of HTTP status codes expected in response from the member to
    /// declare it healthy. Specify one of the following values:
    ///
    /// - A single value, such as `200`
    /// - A list, such as `200, 202`
    /// - A range, such as `200-204`
    ///
    pub expected_codes: Option<String>,

    /// The HTTP method that the health monitor uses for requests. One of
    /// `CONNECT`, `DELETE`, `GET`, `HEAD`, `OPTIONS`, `PATCH`, `POST`, `PUT`,
    /// or `TRACE`.
    ///
    pub http_method: Option<String>,

    /// The HTTP version. One of `1.0` or `1.1`. The default is `1.0`.
    ///
    /// **New in version 2.10**
    ///
    pub http_version: Option<f32>,

    /// The associated health monitor ID.
    ///
    pub id: Option<String>,

    /// The number of successful checks before changing the `operating status`
    /// of the member to `ONLINE`. A valid value is from `1` to `10`.
    ///
    pub max_retries: Option<i32>,

    /// The number of allowed check failures before changing the
    /// `operating status` of the member to `ERROR`. A valid value is from `1`
    /// to `10`.
    ///
    pub max_retries_down: Option<i32>,

    /// Human-readable name of the resource.
    ///
    pub name: Option<String>,

    /// The operating status of the resource. See
    /// [Operating Status Codes](#op-status).
    ///
    pub operating_status: Option<String>,

    pub pools: Option<Vec<Pools>>,

    /// The ID of the project owning this resource.
    ///
    pub project_id: Option<String>,

    /// The provisioning status of the resource. See
    /// [Provisioning Status Codes](#prov-status).
    ///
    pub provisioning_status: Option<String>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    pub tags: Option<Vec<String>>,

    pub tenant_id: Option<String>,

    /// The maximum time, in seconds, that a monitor waits to connect before it
    /// times out. This value must be less than the delay value.
    ///
    pub timeout: Option<i32>,

    /// The type of health monitor. One of `HTTP`, `HTTPS`, `PING`, `SCTP`,
    /// `TCP`, `TLS-HELLO`, or `UDP-CONNECT`.
    ///
    #[serde(rename = "type")]
    pub _type: Option<String>,

    /// The UTC date and timestamp when the resource was last updated.
    ///
    pub updated_at: Option<String>,

    /// The HTTP URL path of the request sent by the monitor to test the health
    /// of a backend member. Must be a string that begins with a forward slash
    /// (`/`).
    ///
    pub url_path: Option<String>,
}

/// Base type for complex types
///
/// `Pools` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pools {
    pub id: String,
}
