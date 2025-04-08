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
//! Response type for the post OS-TRUST/trusts operation

use serde::{Deserialize, Serialize};

/// Trust response representation
#[derive(Clone, Deserialize, Serialize)]
struct TrustResponse {
    /// If set to true then a trust between a trustor and any third-party user
    /// may be issued by the trustee just like a regular trust. If set to
    /// false, stops further redelegation. False by default.
    ///
    allow_redelegation: Option<bool>,

    deleted_at: Option<String>,

    /// Specifies the expiration time of the trust. A trust may be revoked
    /// ahead of expiration. If the value represents a time in the past, the
    /// trust is deactivated. In the redelegation case it must not exceed the
    /// value of the corresponding expires_at field of the redelegated trust or
    /// it may be omitted, then the expires_at value is copied from the
    /// redelegated trust.
    ///
    expires_at: Option<String>,

    /// The ID of the trust.
    ///
    id: Option<String>,

    /// If set to true, then the user attribute of tokens generated based on
    /// the trust will represent that of the trustor rather than the trustee,
    /// thus allowing the trustee to impersonate the trustor. If impersonation
    /// if set to false, then the token's user attribute will represent that of
    /// the trustee.
    ///
    impersonation: Option<bool>,

    /// The links for the `user` resource.
    ///
    links: Option<Links>,

    /// Identifies the project upon which the trustor is delegating
    /// authorization.
    ///
    project_id: Option<String>,

    /// Returned with redelegated trust provides information about the
    /// predecessor in the trust chain.
    ///
    redelegated_trust_id: Option<String>,

    /// Specifies the maximum remaining depth of the redelegated trust chain.
    /// Each subsequent trust has this field decremented by 1 automatically.
    /// The initial trustor issuing new trust that can be redelegated, must set
    /// allow_redelegation to true and may set redelegation_count to an integer
    /// value less than or equal to max_redelegation_count configuration
    /// parameter in order to limit the possible length of derived trust
    /// chains. The trust issued by the trustor using a project-scoped token
    /// (not redelegating), in which allow_redelegation is set to true (the new
    /// trust is redelegatable), will be populated with the value specified in
    /// the max_redelegation_count configuration parameter if
    /// redelegation_count is not set or set to null. If allow_redelegation is
    /// set to false then redelegation_count will be set to 0 in the trust. If
    /// the trust is being issued by the trustee of a redelegatable
    /// trust-scoped token (redelegation case) then redelegation_count should
    /// not be set, as it will automatically be set to the value in the
    /// redelegatable trust-scoped token decremented by 1. Note, if the
    /// resulting value is 0, this means that the new trust will not be
    /// redelegatable, regardless of the value of allow_redelegation.
    ///
    redelegation_count: Option<i32>,

    /// Specifies how many times the trust can be used to obtain a token. This
    /// value is decreased each time a token is issued through the trust. Once
    /// it reaches 0, no further tokens will be issued through the trust. The
    /// default value is null, meaning there is no limit on the number of
    /// tokens issued through the trust. If redelegation is enabled it must not
    /// be set.
    ///
    remaining_uses: Option<i32>,

    roles: Option<Vec<Roles>>,

    /// The links for the `user` resource.
    ///
    roles_links: Option<RolesLinks>,

    /// Represents the user who is capable of consuming the trust.
    ///
    trustee_user_id: Option<String>,

    /// Represents the user who created the trust, and who's authorization is
    /// being delegated.
    ///
    trustor_user_id: Option<String>,
}

/// The links for the `user` resource.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Links {
    next: Option<String>,
    previous: Option<String>,
    _self: String,
}

/// `Options` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Options {
    immutable: Option<bool>,
}

/// `Roles` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Roles {
    description: Option<String>,
    domain_id: Option<String>,
    id: Option<String>,
    links: Option<Links>,
    name: Option<String>,
    options: Option<Options>,
}

/// The links for the `user` resource.
///
/// `RolesLinks` type
#[derive(Clone, Debug, Deserialize, Serialize)]
struct RolesLinks {
    next: Option<String>,
    previous: Option<String>,
    _self: String,
}
