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
//! Response type for the post users/{user_id}/application_credentials operation

use serde::{Deserialize, Serialize};

/// ApplicationCredential response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ApplicationCredentialResponse {
    /// A list of access_rules objects
    ///
    pub access_rules: Option<Vec<AccessRules>>,

    /// A description of the application credential's purpose.
    ///
    pub description: Option<String>,

    pub expires_at: Option<String>,

    /// The ID of the application credential.
    ///
    pub id: Option<String>,

    /// The name of the application credential. Must be unique to a user.
    ///
    pub name: Option<String>,

    /// The ID of the project the application credential was created for and
    /// that authentication requests using this application credential will be
    /// scoped to.
    ///
    pub project_id: Option<String>,

    /// An optional list of role objects, identified by ID or name. The list
    /// may only contain roles that the user has assigned on the project. If
    /// not provided, the roles assigned to the application credential will be
    /// the same as the roles in the current token.
    ///
    pub roles: Option<Vec<Roles>>,

    /// The secret for the application credential, either generated by the
    /// server or provided by the user. This is only ever shown once in the
    /// response to a create request. It is not stored nor ever shown again. If
    /// the secret is lost, a new application credential must be created.
    ///
    pub secret: Option<String>,

    /// An optional flag to restrict whether the application credential may be
    /// used for the creation or destruction of other application credentials
    /// or trusts. Defaults to false.
    ///
    pub unrestricted: Option<bool>,
}

/// `Roles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Roles {
    pub id: Option<String>,
    pub name: Option<String>,
}

/// `AccessRules` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccessRules {
    pub id: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub service: Option<String>,
}
