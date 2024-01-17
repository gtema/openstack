//! Exchange a scoped token for a SAML assertion.
//!
//! POST /v3/auth/OS-FEDERATION/saml2
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

/// A `domain` object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct DomainStructStruct<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,
}

/// A `user` object, required if an application credential is identified by
/// name and not ID.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct User<'a> {
    /// The ID of the application credential used for authentication. If not
    /// provided, the application credential must be identified by its name and
    /// its owning user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// The name of the application credential used for authentication. If
    /// provided, must be accompanied by a user object.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    password: Option<Cow<'a, str>>,

    /// A `domain` object
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    domain: Option<DomainStructStruct<'a>>,
}

/// The `password` object, contains the authentication information.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Password<'a> {
    /// A `user` object, required if an application credential is identified by
    /// name and not ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    user: Option<User<'a>>,
}

/// A `token` object. The token authentication
/// method is used. This method is typically used in combination with
/// a request to change authorization scope.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Token<'a> {
    #[serde()]
    #[builder(setter(into))]
    id: Cow<'a, str>,
}

/// An `identity` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Identity<'a> {
    /// The authentication method. To authenticate with an application
    /// credential,
    /// specify `application\_credential`.
    #[serde()]
    #[builder(setter(into))]
    methods: Vec<Cow<'a, str>>,

    /// The `password` object, contains the authentication information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    password: Option<Password<'a>>,

    /// A `token` object. The token authentication
    /// method is used. This method is typically used in combination with
    /// a request to change authorization scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    token: Option<Token<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Project<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    domain: Option<DomainStructStruct<'a>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct OsTrustTrust<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct System {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    all: Option<bool>,
}

/// The authorization scope (Since v3.4). Specify
/// `unscoped` to make an explicit unscoped token request, which
/// returns an unscoped response without any authorization. This
/// request behaves the same as a token request with no scope where
/// the user has no default project defined. If an explicit,
/// `unscoped` token request is not made and the user has
/// authorization to their default project, then the response will
/// return a project-scoped token. If a default project is not defined,
/// a token is issued without an explicit scope of authorization,
/// which is the same as asking for an explicit unscoped token.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Scope<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    project: Option<Project<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    domain: Option<DomainStructStruct<'a>>,

    #[serde(rename = "OS-TRUST:trust", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    os_trust_trust: Option<OsTrustTrust<'a>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    system: Option<System>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum ScopeEnum<'a> {
    F1(Scope<'a>),
    F2(Cow<'a, str>),
}

/// An `auth` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Auth<'a> {
    /// An `identity` object.
    #[serde()]
    #[builder(setter(into))]
    identity: Identity<'a>,

    /// The authorization scope (Since v3.4). Specify
    /// `unscoped` to make an explicit unscoped token request, which
    /// returns an unscoped response without any authorization. This
    /// request behaves the same as a token request with no scope where
    /// the user has no default project defined. If an explicit,
    /// `unscoped` token request is not made and the user has
    /// authorization to their default project, then the response will
    /// return a project-scoped token. If a default project is not defined,
    /// a token is issued without an explicit scope of authorization,
    /// which is the same as asking for an explicit unscoped token.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    scope: Option<ScopeEnum<'a>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// An `auth` object.
    #[builder(setter(into))]
    auth: Auth<'a>,

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
    /// Add a single header to the Saml2.
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
        "v3/auth/OS-FEDERATION/saml2".to_string().into()
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
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .auth(
                    AuthBuilder::default()
                        .identity(
                            IdentityBuilder::default()
                                .methods(Vec::from(["foo".into()]))
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
        assert!(Request::builder()
            .auth(
                AuthBuilder::default()
                    .identity(
                        IdentityBuilder::default()
                            .methods(Vec::from(["foo".into()]))
                            .build()
                            .unwrap()
                    )
                    .build()
                    .unwrap()
            )
            .build()
            .unwrap()
            .response_key()
            .is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/v3/auth/OS-FEDERATION/saml2".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .auth(
                AuthBuilder::default()
                    .identity(
                        IdentityBuilder::default()
                            .methods(Vec::from(["foo".into()]))
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
                .path("/v3/auth/OS-FEDERATION/saml2".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .auth(
                AuthBuilder::default()
                    .identity(
                        IdentityBuilder::default()
                            .methods(Vec::from(["foo".into()]))
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
