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
//! Response type for the patch OS-FEDERATION/service_providers/{service_provider_id} operation

use serde::{Deserialize, Serialize};

/// ServiceProvider response representation
#[derive(Clone, Deserialize, Serialize)]
pub struct ServiceProviderResponse {
    /// The URL to authenticate against
    ///
    pub auth_url: Option<String>,

    /// The description of the service provider
    ///
    pub description: Option<String>,

    /// Whether the service provider is enabled or not
    ///
    pub enabled: Option<bool>,

    /// The service provider ID
    ///
    pub id: Option<String>,

    /// The link to the resources in question.
    ///
    pub links: Option<Links>,

    /// The prefix of the RelayState SAML attribute
    ///
    pub relay_state_prefix: Option<String>,

    /// The service provider's URL
    ///
    pub sp_url: Option<String>,
}

/// The link to the resources in question.
///
/// `Links` type
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Links {
    pub _self: Option<String>,
}
