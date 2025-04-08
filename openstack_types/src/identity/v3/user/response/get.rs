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
//! Response type for the get users/{user_id} operation

use serde::{Deserialize, Serialize};

/// User response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct UserResponse {
    /// The ID of the default project for the user.
    ///
    pub default_project_id: Option<String>,

    /// The resource description.
    ///
    pub description: Option<String>,

    /// The ID of the domain.
    ///
    pub domain_id: Option<String>,

    /// If the user is enabled, this value is `true`. If the user is disabled,
    /// this value is `false`.
    ///
    pub enabled: Option<bool>,

    /// List of federated objects associated with a user. Each object in the
    /// list contains the `idp_id` and `protocols`. `protocols` is a list of
    /// objects, each of which contains `protocol_id` and `unique_id` of the
    /// protocol and user respectively. For example:
    ///
    /// ```text
    /// "federated": [
    ///   {
    ///     "idp_id": "efbab5a6acad4d108fec6c63d9609d83",
    ///     "protocols": [
    ///       {"protocol_id": "mapped", "unique_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    ///
    pub federated: Option<Vec<Federated>>,

    /// The user ID.
    ///
    pub id: Option<String>,

    /// The links for the `user` resource.
    ///
    pub links: Option<Links>,

    /// The user name. Must be unique within the owning domain.
    ///
    pub name: Option<String>,

    /// The resource options for the user. Available resource options are
    /// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
    /// `ignore_lockout_failure_attempts`, `lock_password`,
    /// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
    /// `ignore_user_inactivity`.
    ///
    pub options: Option<Options>,

    /// The date and time when the password expires. The time zone is UTC.
    ///
    /// This is a response object attribute; not valid for requests. A `null`
    /// value indicates that the password never expires.
    ///
    /// **New in version 3.7**
    ///
    pub password_expires_at: Option<String>,
}

/// `Protocols` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Protocols {
    pub protocol_id: String,
    pub unique_id: String,
}

/// `Federated` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Federated {
    pub idp_id: String,
    pub protocols: Vec<Protocols>,
}

/// The links for the `user` resource.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub _self: String,
}

/// The resource options for the user. Available resource options are
/// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
/// `ignore_lockout_failure_attempts`, `lock_password`,
/// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
/// `ignore_user_inactivity`.
///
/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Options {
    pub ignore_change_password_upon_first_use: Option<bool>,
    pub ignore_lockout_failure_attempts: Option<bool>,
    pub ignore_password_expiry: Option<bool>,
    pub ignore_user_inactivity: Option<bool>,
    pub lock_password: Option<bool>,
    pub multi_factor_auth_enabled: Option<bool>,
    pub multi_factor_auth_rules: Option<Vec<Vec<String>>>,
}
