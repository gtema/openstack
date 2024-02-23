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

//! Authenticates an identity and generates a token. Uses the password
//! authentication method. Authorization is unscoped.
//!
//! The request body must include a payload that specifies the authentication
//! method, which is `password`, and the user, by ID or name, and password
//! credentials.
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/rel/auth_tokens`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Methods {
    #[serde(rename = "application_credential")]
    ApplicationCredential,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "totp")]
    Totp,
}

/// A `domain` object
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Domain<'a> {
    /// User Domain ID
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// User Domain Name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

/// A `user` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct User<'a> {
    /// The ID of the user. Required if you do not specify the user name.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The user name. Required if you do not specify the ID of the user. If
    /// you specify the user name, you must also specify the domain, by ID or
    /// name.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// User Password
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) password: Option<Cow<'a, str>>,

    /// A `domain` object
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<Domain<'a>>,
}

/// The `password` object, contains the authentication information.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Password<'a> {
    /// A `user` object.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) user: Option<User<'a>>,
}

/// A `token` object
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Token<'a> {
    /// Authorization Token value
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) id: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct UserDomainStruct<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct TotpUser<'a> {
    /// The user ID
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The user name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<UserDomainStruct<'a>>,

    /// MFA passcode
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) passcode: Cow<'a, str>,
}

/// Multi Factor Authentication information
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Totp<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) user: TotpUser<'a>,
}

/// A user object, required if an application credential is identified by name
/// and not ID.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ApplicationCredentialUser<'a> {
    /// The user ID
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// The user name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<UserDomainStruct<'a>>,
}

/// An application credential object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ApplicationCredential<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The secret for authenticating the application credential.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) secret: Cow<'a, str>,

    /// A user object, required if an application credential is identified by
    /// name and not ID.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) user: Option<ApplicationCredentialUser<'a>>,
}

/// An `identity` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Identity<'a> {
    /// The authentication method. For password authentication, specify
    /// `password`.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) methods: Vec<Methods>,

    /// The `password` object, contains the authentication information.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) password: Option<Password<'a>>,

    /// A `token` object
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) token: Option<Token<'a>>,

    /// Multi Factor Authentication information
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) totp: Option<Totp<'a>>,

    /// An application credential object.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) application_credential: Option<ApplicationCredential<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ProjectDomain<'a> {
    /// Project domain Id
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// Project domain name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Project<'a> {
    /// Project Name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// Project Id
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<ProjectDomain<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ScopeDomain<'a> {
    /// Domain id
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,

    /// Domain name
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct OsTrustTrust<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct System {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) all: Option<bool>,
}

/// The authorization scope, including the system (Since v3.10), a project, or
/// a domain (Since v3.4). If multiple scopes are specified in the same request
/// (e.g. project and domain or domain and system) an HTTP 400 Bad Request will
/// be returned, as a token cannot be simultaneously scoped to multiple
/// authorization targets. An ID is sufficient to uniquely identify a project
/// but if a project is specified by name, then the domain of the project must
/// also be specified in order to uniquely identify the project by name. A
/// domain scope may be specified by either the domain’s ID or name with
/// equivalent results.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Scope<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project: Option<Project<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain: Option<ScopeDomain<'a>>,

    #[serde(rename = "OS-TRUST:trust", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) os_trust_trust: Option<OsTrustTrust<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) system: Option<System>,
}

/// An `auth` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Auth<'a> {
    /// An `identity` object.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) identity: Identity<'a>,

    /// The authorization scope, including the system (Since v3.10), a project,
    /// or a domain (Since v3.4). If multiple scopes are specified in the same
    /// request (e.g. project and domain or domain and system) an HTTP 400 Bad
    /// Request will be returned, as a token cannot be simultaneously scoped to
    /// multiple authorization targets. An ID is sufficient to uniquely
    /// identify a project but if a project is specified by name, then the
    /// domain of the project must also be specified in order to uniquely
    /// identify the project by name. A domain scope may be specified by either
    /// the domain’s ID or name with equivalent results.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) scope: Option<Scope<'a>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// An `auth` object.
    ///
    #[builder(setter(into))]
    pub(crate) auth: Auth<'a>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Token.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> RestEndpoint for Request<'a> {
    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "v3/auth/tokens".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("auth", serde_json::to_value(&self.auth)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("token".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::*;
    use crate::api::Query;
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .auth(
                    AuthBuilder::default()
                        .identity(
                            IdentityBuilder::default()
                                .methods(Vec::from([Methods::ApplicationCredential]))
                                .build()
                                .unwrap()
                        )
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .auth(
                    AuthBuilder::default()
                        .identity(
                            IdentityBuilder::default()
                                .methods(Vec::from([Methods::ApplicationCredential]))
                                .build()
                                .unwrap()
                        )
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "token"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v3/auth/tokens".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": {} }));
        });

        let endpoint = Request::builder()
            .auth(
                AuthBuilder::default()
                    .identity(
                        IdentityBuilder::default()
                            .methods(Vec::from([Methods::ApplicationCredential]))
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v3/auth/tokens".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": {} }));
        });

        let endpoint = Request::builder()
            .auth(
                AuthBuilder::default()
                    .identity(
                        IdentityBuilder::default()
                            .methods(Vec::from([Methods::ApplicationCredential]))
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .iter()
                .cloned(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
