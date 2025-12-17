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

use super::finish;
use super::start;

/// Passkey auth related errors.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PasskeyError {
    /// Auth data is missing
    #[error("auth data is missing")]
    MissingAuthData,

    /// Start authentication request builder.
    #[error("error preparing auth request: {}", source)]
    InitAuthBuilder {
        #[from]
        source: start::AuthStartRequestBuilderError,
    },

    /// Start authentication passkey request builder.
    #[error("error preparing auth request: {}", source)]
    InitPasskeyBuilder {
        #[from]
        source: start::PasskeyBuilderError,
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

    /// WebAuthn.
    #[error(transparent)]
    WebAuthn {
        #[from]
        source: webauthn_authenticator_rs::error::WebauthnCError,
    },
}
