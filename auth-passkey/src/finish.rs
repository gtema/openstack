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

//! Finish passkey (security device) auth: exchange signed challenge with Keystone token.

use std::borrow::Cow;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::error::PasskeyError;

/// A client response to an authentication challenge. This contains all required information to
/// asses and assert trust in a credentials legitimacy, followed by authentication to a user.
///
/// You should not need to handle the inner content of this structure - you should provide this to
/// the correctly handling function of Webauthn only.
#[derive(Builder, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[builder(setter(into, strip_option))]
pub struct AuthFinishRequest<'a> {
    /// The credential Id, likely base64.
    id: Cow<'a, str>,
    /// Unsigned Client processed extensions.
    extensions: AuthenticationExtensionsClientOutputs<'a>,
    /// The binary of the credential id.
    raw_id: Cow<'a, str>,
    /// The authenticator response.
    response: AuthenticatorAssertionResponseRaw<'a>,
    /// The authenticator type.
    type_: Cow<'a, str>,
    /// The ID of the user.
    user_id: Cow<'a, str>,
}

/// [AuthenticatorAssertionResponseRaw](https://w3c.github.io/webauthn/#authenticatorassertionresponse)
#[derive(Builder, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[builder(setter(into, strip_option))]
pub struct AuthenticatorAssertionResponseRaw<'a> {
    /// Raw authenticator data.
    pub authenticator_data: Cow<'a, str>,
    /// Signed client data.
    pub client_data_json: Cow<'a, str>,
    /// Signature.
    pub signature: Cow<'a, str>,
    /// Optional userhandle.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user_handle: Option<Cow<'a, str>>,
}

/// [AuthenticationExtensionsClientOutputs](https://w3c.github.io/webauthn/#dictdef-authenticationextensionsclientoutputs)
///
/// The default option here for Options are None, so it can be derived
#[derive(Builder, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[builder(setter(into, strip_option))]
pub struct AuthenticationExtensionsClientOutputs<'a> {
    /// Indicates whether the client used the provided appid extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub appid: Option<bool>,
    /// The response to a hmac get secret request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub hmac_get_secret: Option<HmacGetSecretOutput<'a>>,
}

/// The response to a hmac get secret request.
#[derive(Builder, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[builder(setter(into, strip_option))]
pub struct HmacGetSecretOutput<'a> {
    /// Output of HMAC(Salt 1 || Client Secret).
    pub output1: Cow<'a, str>,
    /// Output of HMAC(Salt 2 || Client Secret).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub output2: Option<Cow<'a, str>>,
}

impl TryFrom<webauthn_authenticator_rs::prelude::PublicKeyCredential>
    for AuthFinishRequestBuilder<'_>
{
    type Error = PasskeyError;
    fn try_from(
        value: webauthn_authenticator_rs::prelude::PublicKeyCredential,
    ) -> Result<Self, Self::Error> {
        let mut req = AuthFinishRequestBuilder::default();
        req.id(value.id);
        req.raw_id(URL_SAFE.encode(value.raw_id));
        req.type_(value.type_);
        let mut ext_builder = AuthenticationExtensionsClientOutputsBuilder::default();
        if let Some(appid) = value.extensions.appid {
            ext_builder.appid(appid);
        }
        if let Some(ext) = &value.extensions.hmac_get_secret {
            let mut hmac_out = HmacGetSecretOutputBuilder::default();
            hmac_out.output1(URL_SAFE.encode(ext.output1.clone()));
            if let Some(out2) = &ext.output2 {
                hmac_out.output2(URL_SAFE.encode(out2));
            }
            ext_builder.hmac_get_secret(hmac_out.build()?);
        }
        req.extensions(ext_builder.build()?);
        let mut rsp_builder = AuthenticatorAssertionResponseRawBuilder::default();
        rsp_builder.authenticator_data(URL_SAFE.encode(value.response.authenticator_data));
        rsp_builder.client_data_json(URL_SAFE.encode(value.response.client_data_json));
        rsp_builder.signature(URL_SAFE.encode(value.response.signature));
        if let Some(uh) = &value.response.user_handle {
            rsp_builder.user_handle(URL_SAFE.encode(uh));
        }
        req.response(rsp_builder.build()?);
        Ok(req)
    }
}
