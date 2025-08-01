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

//! Updates a user.
//!
//! If the back-end driver does not support this functionality, this call might
//! return the HTTP `Not Implemented (501)` response code.
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/rel/user`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Protocols<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) protocol_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) unique_id: Cow<'a, str>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Federated<'a> {
    #[serde()]
    #[builder(setter(into))]
    pub(crate) idp_id: Cow<'a, str>,

    #[serde()]
    #[builder(setter(into))]
    pub(crate) protocols: Vec<Protocols<'a>>,
}

/// The resource options for the user. Available resource options are
/// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
/// `ignore_lockout_failure_attempts`, `lock_password`,
/// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
/// `ignore_user_inactivity`.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Options<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ignore_change_password_upon_first_use: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ignore_lockout_failure_attempts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ignore_password_expiry: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) ignore_user_inactivity: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) lock_password: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) multi_factor_auth_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(into, name = "_multi_factor_auth_rules"))]
    pub(crate) multi_factor_auth_rules: Option<Vec<Vec<Cow<'a, str>>>>,
}

impl<'a> OptionsBuilder<'a> {
    pub fn multi_factor_auth_rules<I1, I2, V>(&mut self, iter: I1) -> &mut Self
    where
        I1: Iterator<Item = I2>,
        I2: IntoIterator<Item = V>,
        V: Into<Cow<'a, str>>,
    {
        self.multi_factor_auth_rules
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(|x| Vec::from_iter(x.into_iter().map(Into::into))));
        self
    }
}

/// A `user` object
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct User<'a> {
    /// The new ID of the default project for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) default_project_id: Option<Option<Cow<'a, str>>>,

    /// The description of the user resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// The ID of the new domain for the user. The ability to change the domain
    /// of a user is now deprecated, and will be removed in subequent release.
    /// It is already disabled by default in most Identity service
    /// implementations.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) domain_id: Option<Cow<'a, str>>,

    /// Enables or disables the user. An enabled user can authenticate and
    /// receive authorization. A disabled user cannot authenticate or receive
    /// authorization. Additionally, all tokens that the user holds become no
    /// longer valid. If you reenable this user, pre-existing tokens do not
    /// become valid. To enable the user, set to `true`. To disable the user,
    /// set to `false`. Default is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) enabled: Option<bool>,

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
    ///       {"protocol_id": mapped, "unique_id": "test@example.com"}
    ///     ]
    ///   }
    /// ]
    ///
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) federated: Option<Vec<Federated<'a>>>,

    /// The new name for the user. Must be unique within the owning domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The resource options for the user. Available resource options are
    /// `ignore_change_password_upon_first_use`, `ignore_password_expiry`,
    /// `ignore_lockout_failure_attempts`, `lock_password`,
    /// `multi_factor_auth_enabled`, and `multi_factor_auth_rules`
    /// `ignore_user_inactivity`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) options: Option<Options<'a>>,

    /// The new password for the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) password: Option<Option<Cow<'a, str>>>,

    #[builder(setter(name = "_properties"), default, private)]
    #[serde(flatten)]
    _properties: BTreeMap<Cow<'a, str>, Value>,
}

impl<'a> UserBuilder<'a> {
    pub fn properties<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self._properties
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `user` object
    #[builder(setter(into))]
    pub(crate) user: User<'a>,

    /// user_id parameter for /v3/users/{user_id} API
    #[builder(default, setter(into))]
    id: Cow<'a, str>,

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
    /// Add a single header to the User.
    pub fn header<K, V>(&mut self, header_name: K, header_value: V) -> &mut Self
    where
        K: Into<HeaderName>,
        V: Into<HeaderValue>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name.into(), header_value.into());
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

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::PATCH
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("users/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("user", serde_json::to_value(&self.user)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("user".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use httpmock::MockServer;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder()
                .user(UserBuilder::default().build().unwrap())
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
                .user(UserBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "user"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/users/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "user": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .user(UserBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::PATCH)
                .path(format!("/users/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "user": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .user(UserBuilder::default().build().unwrap())
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header(
                HeaderName::from_static("not_foo"),
                HeaderValue::from_static("not_bar"),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
