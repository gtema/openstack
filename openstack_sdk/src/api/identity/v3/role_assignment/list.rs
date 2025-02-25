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

//! Get a list of role assignments.
//!
//! If no query parameters are specified, then this API will return a list of
//! all role assignments.
//!
//! Since this list is likely to be very long, this API would typically always
//! be used with one of more of the filter queries. Some typical examples are:
//!
//! `GET /v3/role_assignments?user.id={user_id}` would list all role
//! assignments involving the specified user.
//!
//! `GET /v3/role_assignments?scope.project.id={project_id}` would list all
//! role assignments involving the specified project.
//!
//! It is also possible to list all role assignments within a tree of projects:
//! `GET /v3/role_assignments?scope.project.id={project_id}&include_subtree=true`
//! would list all role assignments involving the specified project and all
//! sub-projects. `include_subtree=true` can only be specified in conjunction
//! with `scope.project.id`, specifying it without this will result in an HTTP
//! 400 Bad Request being returned.
//!
//! Each role assignment entity in the collection contains a link to the
//! assignment that gave rise to this entity.
//!
//! The scope section in the list response is extended to allow the
//! representation of role assignments that are inherited to projects.
//!
//! The query filter `scope.OS-INHERIT:inherited_to` can be used to filter
//! based on role assignments that are inherited. The only value of
//! `scope.OS-INHERIT:inherited_to` that is currently supported is `projects`,
//! indicating that this role is inherited to all projects of the owning domain
//! or parent project.
//!
//! If the query parameter `effective` is specified, rather than simply
//! returning a list of role assignments that have been made, the API returns a
//! list of effective assignments at the user, project and domain level, having
//! allowed for the effects of group membership, role inference rules as well
//! as inheritance from the parent domain or project. Since the effects of
//! group membership have already been allowed for, the group role assignment
//! entities themselves will not be returned in the collection. Likewise, since
//! the effects of inheritance have already been allowed for, the role
//! assignment entities themselves that specify the inheritance will also not
//! be returned in the collection. This represents the effective role
//! assignments that would be included in a scoped token. The same set of query
//! parameters can also be used in combination with the `effective` parameter.
//!
//! For example:
//!
//! `GET /v3/role_assignments?user.id={user_id}&effective` would, in other
//! words, answer the question “what can this user actually do?”.
//!
//! `GET /v3/role_assignments?user.id={user_id}&scope.project.id={project_id}&effective`
//! would return the equivalent set of role assignments that would be included
//! in the token response of a project scoped token.
//!
//! An example response for an API call with the query parameter `effective`
//! specified is given below:
//!
//! The entity `links` section of a response using the `effective` query
//! parameter also contains, for entities that are included by virtue of group
//! membership, a url that can be used to access the membership of the group.
//!
//! If the query parameter `include_names` is specified, rather than simply
//! returning the entity IDs in the role assignments, the collection will
//! additionally include the names of the entities. For example:
//!
//! `GET /v3/role_assignments?user.id={user_id}&effective&include_names=true`
//! would return:
//!
//! Relationship:
//! `https://docs.openstack.org/api/openstack-identity/3/rel/role_assignments`
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(default, setter(into))]
    effective: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    group_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    include_names: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    include_subtree: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    role_id: Option<Cow<'a, str>>,

    /// The ID of the domain.
    ///
    #[builder(default, setter(into))]
    scope_domain_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    scope_os_inherit_inherited_to: Option<Cow<'a, str>>,

    /// The ID of the project.
    ///
    #[builder(default, setter(into))]
    scope_project_id: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    scope_system: Option<Cow<'a, str>>,

    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}
impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl RequestBuilder<'_> {
    /// Add a single header to the Role_Assignment.
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

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "role_assignments".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("effective", self.effective.as_ref());
        params.push_opt("include_names", self.include_names.as_ref());
        params.push_opt("include_subtree", self.include_subtree.as_ref());
        params.push_opt("group.id", self.group_id.as_ref());
        params.push_opt("role.id", self.role_id.as_ref());
        params.push_opt("scope.system", self.scope_system.as_ref());
        params.push_opt("scope.domain.id", self.scope_domain_id.as_ref());
        params.push_opt("scope.project.id", self.scope_project_id.as_ref());
        params.push_opt("user.id", self.user_id.as_ref());
        params.push_opt(
            "scope.OS-INHERIT:inherited_to",
            self.scope_os_inherit_inherited_to.as_ref(),
        );

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("role_assignments".into())
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
    #![allow(unused_imports)]
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "sync")]
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Request::builder().build().unwrap().service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "role_assignments"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/role_assignments".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "role_assignments": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/role_assignments".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "role_assignments": {} }));
        });

        let endpoint = Request::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .into_iter(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
