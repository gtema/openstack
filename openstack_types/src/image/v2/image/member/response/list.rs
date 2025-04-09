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
//! Response type for the get images/{image_id}/members operation

use serde::{Deserialize, Serialize};

/// Member response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct MemberResponse {
    /// Date and time of image member creation
    ///
    pub created_at: Option<String>,

    /// An identifier for the image
    ///
    pub image_id: Option<String>,

    /// An identifier for the image member (tenantId)
    ///
    pub member_id: Option<String>,

    pub schema: Option<String>,

    /// The status of this image member
    ///
    pub status: Option<Status>,

    /// Date and time of last modification of image member
    ///
    pub updated_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum Status {
    // Pending
    #[serde(rename = "pending")]
    Pending,

    // Accepted
    #[serde(rename = "accepted")]
    Accepted,

    // Rejected
    #[serde(rename = "rejected")]
    Rejected,
}
