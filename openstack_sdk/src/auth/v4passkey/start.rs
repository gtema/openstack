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

//! Initialize passkey (security device) based login.
//!

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::api::rest_endpoint_prelude::*;
use crate::api::RestEndpoint;
use crate::auth::auth_helper::AuthHelper;
use crate::config;
use crate::types::{ApiVersion, ServiceType};

use super::PasskeyError;

/// Endpoint for initializing passkey authorization.
#[derive(Builder, Debug, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Passkey<'a> {
    /// User ID.
    #[builder(setter(into))]
    user_id: Cow<'a, str>,
}

/// Endpoint for initializing passkey authorization.
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct AuthStartRequest<'a> {
    /// passkey auth request.
    #[builder(setter(into))]
    passkey: Passkey<'a>,
}

impl<'a> AuthStartRequest<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> AuthStartRequestBuilder<'a> {
        AuthStartRequestBuilder::default()
    }
}

impl RestEndpoint for AuthStartRequest<'_> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "auth/passkey/start".to_string().into()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("passkey", serde_json::to_value(&self.passkey)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(4, 0))
    }
}

/// Get [`RestEndpoint`] for initializing the passkey authentication
pub async fn get_init_auth_ep<A: AuthHelper>(
    config: &config::CloudConfig,
    _auth_helper: &mut A,
) -> Result<impl RestEndpoint, PasskeyError> {
    if let Some(auth) = &config.auth {
        let mut ep = AuthStartRequest::builder();
        let mut passkey = PasskeyBuilder::default();

        if let Some(val) = &auth.user_id {
            passkey.user_id(val.clone());
        } else {
            return Err(PasskeyError::MissingAuthData);
        }
        ep.passkey(passkey.build()?);
        return Ok(ep.build()?);
    }

    Err(PasskeyError::MissingAuthData)
}

/// Passkey Authorization challenge.
///
/// A JSON serializable challenge which is issued to the user’s webbrowser for handling. This is
/// meant to be opaque, that is, you should not need to inspect or alter the content of the struct
/// - you should serialise it and transmit it to the client only.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PasskeyAuthenticationStartResponse {
    /// The options.
    pub public_key: PublicKeyCredentialRequestOptions,
    /// The mediation requested.
    pub mediation: Option<Mediation>,
}

/// The requested options for the authentication.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PublicKeyCredentialRequestOptions {
    /// The set of credentials that are allowed to sign this challenge.
    pub allow_credentials: Vec<AllowCredentials>,
    /// The challenge that should be signed by the authenticator.
    pub challenge: String,
    /// extensions.
    pub extensions: Option<RequestAuthenticationExtensions>,
    /// Hints defining which types credentials may be used in this operation.
    pub hints: Option<Vec<PublicKeyCredentialHint>>,
    /// The relying party ID.
    pub rp_id: String,
    /// The timeout for the authenticator in case of no interaction.
    pub timeout: Option<u32>,
    /// The verification policy the browser will request.
    pub user_verification: UserVerificationPolicy,
}

/// Request in residentkey workflows that conditional mediation should be used in the UI, or not.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Mediation {
    /// Discovered credentials are presented to the user in a dialog. Conditional UI is used. See
    /// https://github.com/w3c/webauthn/wiki/Explainer:-WebAuthn-Conditional-UI
    /// https://w3c.github.io/webappsec-credential-management/#enumdef-credentialmediationrequirement
    Conditional,
}

/// A descriptor of a credential that can be used.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AllowCredentials {
    /// The id of the credential.
    pub id: String,
    /// https://www.w3.org/TR/webauthn/#transport may be usb, nfc, ble, internal
    pub transports: Option<Vec<AuthenticatorTransport>>,
    /// The type of credential.
    pub type_: String,
}

/// https://www.w3.org/TR/webauthn/#enumdef-authenticatortransport
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum AuthenticatorTransport {
    /// https://www.w3.org/TR/webauthn/#dom-authenticatortransport-ble
    Ble,
    /// Hybrid transport, formerly caBLE. Part of the level 3 draft specification. https://w3c.github.io/webauthn/#dom-authenticatortransport-hybrid
    Hybrid,
    /// https://www.w3.org/TR/webauthn/#dom-authenticatortransport-internal
    Internal,
    /// https://www.w3.org/TR/webauthn/#dom-authenticatortransport-nfc
    Nfc,
    /// Test transport; used for Windows 10.
    Test,
    /// An unknown transport was provided - it will be ignored.
    Unknown,
    /// https://www.w3.org/TR/webauthn/#dom-authenticatortransport-usb
    Usb,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum UserVerificationPolicy {
    /// Require user verification bit to be set, and fail the registration or authentication if
    /// false. If the authenticator is not able to perform verification, it will not be usable with
    /// this policy.
    ///
    /// This policy is the default as it is the only secure and consistent user verification
    /// option.
    Required,
    /// Prefer UV if possible, but ignore if not present. In other webauthn deployments this is
    /// bypassable as it implies the library will not check UV is set correctly for this
    /// credential. Webauthn-RS is not vulnerable to this as we check the UV state always based on
    /// it’s presence at registration.
    ///
    /// However, in some cases use of this policy can lead to some credentials failing to verify
    /// correctly due to browser peripheral exchange bypasses.
    Preferred,
    /// Discourage - but do not prevent - user verification from being supplied. Many CTAP devices
    /// will attempt UV during registration but not authentication leading to user confusion.
    DiscouragedDoNotUse,
}

/// A hint as to the class of device that is expected to fufil this operation.
///
/// https://www.w3.org/TR/webauthn-3/#enumdef-publickeycredentialhints
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum PublicKeyCredentialHint {
    /// The credential is a platform authenticator.
    ClientDevice,
    /// The credential will come from an external device.
    Hybrid,
    /// The credential is a removable security key.
    SecurityKey,
}

/// Extension option inputs for PublicKeyCredentialRequestOptions
///
/// Implements [AuthenticatorExtensionsClientInputs] from the spec
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RequestAuthenticationExtensions {
    /// The appid extension options.
    pub appid: Option<String>,
    /// ⚠️ - Browsers do not support this!
    /// https://bugs.chromium.org/p/chromium/issues/detail?id=1023225 Hmac get secret.
    pub hmac_get_secret: Option<HmacGetSecretInput>,
    /// ⚠️ - Browsers do not support this! Uvm.
    pub uvm: Option<bool>,
}

/// The inputs to the hmac secret if it was created during registration.
///
/// https://fidoalliance.org/specs/fido-v2.1-ps-20210615/fido-client-to-authenticator-protocol-v2.1-ps-20210615.html#sctn-hmac-secret-extension
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct HmacGetSecretInput {
    /// Retrieve a symmetric secrets from the authenticator with this input.
    pub output1: String,
    /// Rotate the secret in the same operation.
    pub output2: Option<String>,
}

impl TryFrom<HmacGetSecretInput> for webauthn_rs_proto::extensions::HmacGetSecretInput {
    type Error = PasskeyError;
    fn try_from(val: HmacGetSecretInput) -> Result<Self, Self::Error> {
        Ok(Self {
            output1: URL_SAFE.decode(val.output1)?.into(),
            output2: val
                .output2
                .map(|s2| URL_SAFE.decode(s2))
                .transpose()?
                .map(Into::into),
        })
    }
}

impl TryFrom<RequestAuthenticationExtensions>
    for webauthn_rs_proto::extensions::RequestAuthenticationExtensions
{
    type Error = PasskeyError;
    fn try_from(val: RequestAuthenticationExtensions) -> Result<Self, Self::Error> {
        Ok(Self {
            appid: val.appid,
            hmac_get_secret: val.hmac_get_secret.map(TryInto::try_into).transpose()?,
            uvm: val.uvm,
        })
    }
}

impl From<AuthenticatorTransport> for webauthn_rs_proto::options::AuthenticatorTransport {
    fn from(val: AuthenticatorTransport) -> Self {
        match val {
            AuthenticatorTransport::Ble => webauthn_rs_proto::options::AuthenticatorTransport::Ble,
            AuthenticatorTransport::Hybrid => {
                webauthn_rs_proto::options::AuthenticatorTransport::Hybrid
            }
            AuthenticatorTransport::Internal => {
                webauthn_rs_proto::options::AuthenticatorTransport::Internal
            }
            AuthenticatorTransport::Nfc => webauthn_rs_proto::options::AuthenticatorTransport::Nfc,
            AuthenticatorTransport::Test => {
                webauthn_rs_proto::options::AuthenticatorTransport::Test
            }
            AuthenticatorTransport::Unknown => {
                webauthn_rs_proto::options::AuthenticatorTransport::Unknown
            }
            AuthenticatorTransport::Usb => webauthn_rs_proto::options::AuthenticatorTransport::Usb,
        }
    }
}

impl From<UserVerificationPolicy> for webauthn_rs_proto::options::UserVerificationPolicy {
    fn from(val: UserVerificationPolicy) -> Self {
        match val {
            UserVerificationPolicy::Required => {
                webauthn_rs_proto::options::UserVerificationPolicy::Required
            }
            UserVerificationPolicy::Preferred => {
                webauthn_rs_proto::options::UserVerificationPolicy::Preferred
            }
            UserVerificationPolicy::DiscouragedDoNotUse => {
                webauthn_rs_proto::options::UserVerificationPolicy::Discouraged_DO_NOT_USE
            }
        }
    }
}

impl From<PublicKeyCredentialHint> for webauthn_rs_proto::options::PublicKeyCredentialHints {
    fn from(val: PublicKeyCredentialHint) -> Self {
        match val {
            PublicKeyCredentialHint::ClientDevice => {
                webauthn_rs_proto::options::PublicKeyCredentialHints::ClientDevice
            }
            PublicKeyCredentialHint::Hybrid => {
                webauthn_rs_proto::options::PublicKeyCredentialHints::Hybrid
            }
            PublicKeyCredentialHint::SecurityKey => {
                webauthn_rs_proto::options::PublicKeyCredentialHints::SecurityKey
            }
        }
    }
}

impl TryFrom<AllowCredentials> for webauthn_rs_proto::options::AllowCredentials {
    type Error = PasskeyError;
    fn try_from(val: AllowCredentials) -> Result<Self, Self::Error> {
        Ok(Self {
            id: URL_SAFE.decode(val.id)?.into(),
            transports: val
                .transports
                .map(|tr| tr.into_iter().map(Into::into).collect::<Vec<_>>()),
            type_: val.type_,
        })
    }
}

impl TryFrom<PublicKeyCredentialRequestOptions>
    for webauthn_rs_proto::auth::PublicKeyCredentialRequestOptions
{
    type Error = PasskeyError;
    fn try_from(val: PublicKeyCredentialRequestOptions) -> Result<Self, Self::Error> {
        Ok(Self {
            allow_credentials: val
                .allow_credentials
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
            challenge: URL_SAFE.decode(val.challenge)?.into(),
            extensions: val.extensions.map(TryInto::try_into).transpose()?,
            hints: val
                .hints
                .map(|hints| hints.into_iter().map(Into::into).collect::<Vec<_>>()),
            rp_id: val.rp_id,
            timeout: val.timeout,
            user_verification: val.user_verification.into(),
        })
    }
}

impl TryFrom<PasskeyAuthenticationStartResponse>
    for webauthn_authenticator_rs::prelude::RequestChallengeResponse
{
    type Error = PasskeyError;
    fn try_from(val: PasskeyAuthenticationStartResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            public_key: val.public_key.try_into()?,
            mediation: val.mediation.map(|med| match med {
                Mediation::Conditional => webauthn_rs_proto::auth::Mediation::Conditional,
            }),
        })
    }
}
