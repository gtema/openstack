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
//! Response type for the post credentials operation

use serde::{Deserialize, Serialize};

/// Credential response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct CredentialResponse {
    /// The credential itself, as a serialized blob.
    ///
    pub blob: Option<String>,

    /// The UUID for the credential.
    ///
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    pub links: Option<Links>,

    /// The ID for the project.
    ///
    pub project_id: Option<String>,

    /// The credential type, such as `ec2` or `cert`. The implementation
    /// determines the list of supported types.
    ///
    #[serde(rename = "type")]
    pub _type: Option<String>,

    /// The ID of the user who owns the credential.
    ///
    pub user_id: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
