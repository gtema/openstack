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

//! Passkey (security device) based login.
//!

use thiserror::Error;

use openstack_sdk_auth_core::AuthError;

use crate::finish;

/// Passkey auth related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PasskeyError {
    /// Passkey auth extensions.
    #[error(transparent)]
    AuthExtensions {
        #[from]
        source: finish::AuthenticationExtensionsClientOutputsBuilderError,
    },

    /// Passkey auth response.
    #[error(transparent)]
    AuthResponse {
        #[from]
        source: finish::AuthenticatorAssertionResponseRawBuilderError,
    },

    /// Base64 decode.
    #[error(transparent)]
    Base64 {
        #[from]
        source: base64::DecodeError,
    },

    /// Finish authentication request builder.
    #[error(transparent)]
    FinishPasskeyBuilder {
        #[from]
        source: finish::AuthFinishRequestBuilderError,
    },

    /// HmacGetSecretOutput.
    #[error(transparent)]
    HmacGetSecretOutput {
        #[from]
        source: finish::HmacGetSecretOutputBuilderError,
    },

    #[error("json error: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// Auth data is missing
    #[error("auth data is missing")]
    MissingAuthData,

    /// Reqwest library error.
    #[error("reqwest error: {}", source)]
    Reqwest {
        /// The source of the error.
        #[from]
        source: reqwest::Error,
    },

    /// URL parse error
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// WebAuthn.
    #[error(transparent)]
    WebAuthn {
        #[from]
        source: webauthn_authenticator_rs::error::WebauthnCError,
    },
}

impl From<PasskeyError> for AuthError {
    fn from(source: PasskeyError) -> Self {
        Self::plugin(source)
    }
}
