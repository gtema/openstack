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
//! Response type for the get images/{image_id}/members/{member_id} operation

use serde::{Deserialize, Serialize};

/// Member response representation
#[derive(Clone, Deserialize, Serialize)]
struct MemberResponse {
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
    ///
    created_at: Option<String>,

    /// The UUID of the image.
    ///
    image_id: Option<String>,

    /// The ID of the image member. An image member is usually a project (also
    /// called the “tenant”) with whom the image is shared.
    ///
    member_id: Option<String>,

    /// The URL for the schema describing an image member.
    ///
    schema: Option<String>,

    /// The status of this image member. Value is one of `pending`, `accepted`,
    /// `rejected`.
    ///
    status: Option<Status>,

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
    ///
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Rejected
    #[serde(rename = "rejected")]
    Rejected,

    // Accepted
    #[serde(rename = "accepted")]
    Accepted,

    // Pending
    #[serde(rename = "pending")]
    Pending,
}
