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

//! OpenStack AuthToken based authorization (X-Auth-Token)

use std::fmt::Debug;
use std::str::FromStr;

use http::{HeaderName, HeaderValue};

pub use openstack_sdk_auth_core::authtoken::{AuthToken, AuthTokenError};
pub use openstack_sdk_auth_core::authtoken_scope::AuthTokenScope;

use crate::auth::v3_token_info;
use crate::config;

/// Supported AuthTypes
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum AuthType {
    /// v3 Application Credentials
    V3ApplicationCredential,
    /// OIDC Access token
    V3OidcAccessToken,
    /// v3 Password
    V3Password,
    /// v3 Token
    V3Token,
    /// TOTP
    V3Totp,
    /// v3multifactor
    V3Multifactor,
    /// WebSSO
    V3WebSso,
    #[cfg(feature = "keystone_ng")]
    /// Federation.
    V4Federation,
    #[cfg(feature = "keystone_ng")]
    /// JWT.
    V4Jwt,
    #[cfg(feature = "passkey")]
    /// Passkey.
    V4Passkey,
}

impl FromStr for AuthType {
    type Err = AuthTokenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "v3applicationcredential" | "applicationcredential" => {
                Ok(Self::V3ApplicationCredential)
            }
            "v3password" | "password" => Ok(Self::V3Password),
            "v3oidcaccesstoken" | "accesstoken" => Ok(Self::V3OidcAccessToken),
            "v3token" | "token" => Ok(Self::V3Token),
            "v3totp" => Ok(Self::V3Totp),
            "v3multifactor" => Ok(Self::V3Multifactor),
            "v3websso" => Ok(Self::V3WebSso),
            #[cfg(feature = "keystone_ng")]
            "v4federation" | "federation" => Ok(Self::V4Federation),
            #[cfg(feature = "keystone_ng")]
            "v4jwt" | "jwt" => Ok(Self::V4Jwt),
            #[cfg(feature = "passkey")]
            "v4passkey" | "passkey" => Ok(Self::V4Passkey),
            other => Err(Self::Err::IdentityMethod {
                auth_type: other.into(),
            }),
        }
    }
}

impl AuthType {
    /// Get the auth_type of the cloud connection
    pub fn from_cloud_config(config: &config::CloudConfig) -> Result<Self, AuthTokenError> {
        if let Some(auth_type) = &config.auth_type {
            Self::from_str(auth_type)
        } else {
            Ok(Self::V3Password)
        }
    }

    /// Return String representation of the AuthType
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V3ApplicationCredential => "v3applicationcredential",
            Self::V3Password => "v3password",
            Self::V3OidcAccessToken => "v3oidcaccesstoken",
            Self::V3Token => "v3token",
            Self::V3Multifactor => "v3multifactor",
            Self::V3Totp => "v3totp",
            Self::V3WebSso => "v3websso",
            #[cfg(feature = "keystone_ng")]
            Self::V4Federation => "v4federation",
            #[cfg(feature = "keystone_ng")]
            Self::V4Jwt => "v4jwt",
            #[cfg(feature = "passkey")]
            Self::V4Passkey => "v4passkey",
        }
    }
}

/// Prepare Endpoint for token info
pub(crate) fn build_token_info_endpoint<S: AsRef<str>>(
    subject_token: S,
) -> Result<v3_token_info::Request, AuthTokenError> {
    v3_token_info::RequestBuilder::default()
        .headers(
            [(
                Some(HeaderName::from_static("x-subject-token")),
                HeaderValue::from_str(subject_token.as_ref())?,
            )]
            .into_iter(),
        )
        .build()
        .map_err(AuthTokenError::plugin)
}
