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

//! List an optionally filtered collection of resource providers.
//!
//! Normal Response Codes: 200
//!
//! Error response codes: badRequest(400)
//!
//! A 400 BadRequest response code will be returned if a resource class
//! specified in `resources` request parameter does not exist.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A string representing a resource provider uuid. When supplied, it will
    /// filter the returned allocation candidates to only those resource
    /// providers that are in the same tree with the given resource provider.
    #[builder(default, setter(into))]
    in_tree: Option<Cow<'a, str>>,

    /// A string representing an aggregate uuid; or the prefix in: followed by
    /// a comma-separated list of strings representing aggregate uuids. The
    /// resource providers in the allocation request in the response must
    /// directly or via the root provider be associated with the aggregate or
    /// aggregates identified by uuid:
    /// `member_of=5e08ea53-c4c6-448e-9334-ac4953de3cfa`,
    /// `member_of=in:42896e0d-205d-4fe3-bd1e-100924931787,5e08ea53-c4c6-448e-9334-ac4953de3cfa`
    /// Starting from microversion 1.24 specifying multiple member_of query
    /// string parameters is possible. Multiple member_of parameters will
    /// result in filtering providers that are directly or via root provider
    /// associated with aggregates listed in all of the member_of query string
    /// values. For example, to get the providers that are associated with
    /// aggregate A as well as associated with any of aggregates B or C, the
    /// user could issue the following query:
    /// `member_of=AGGA_UUID&member_of=in:AGGB_UUID,AGGC_UUID` Starting from
    /// microversion 1.32 specifying forbidden aggregates is supported in the
    /// member_of query string parameter. Forbidden aggregates are prefixed
    /// with a !. This negative expression can also be used in multiple
    /// member_of parameters: `member_of=AGGA_UUID&member_of=!AGGB_UUID` would
    /// translate logically to “Candidate resource providers must be in AGGA
    /// and not in AGGB.” We do NOT support ! on the values within in:, but we
    /// support !in:. Both of the following two example queries return
    /// candidate resource providers that are NOT in AGGA, AGGB, or AGGC:
    /// `member_of=!in:AGGA_UUID,AGGB_UUID,AGGC_UUID`,
    /// `member_of=!AGGA_UUID&member_of=!AGGB_UUID&member_of=!AGGC_UUID` We do
    /// not check if the same aggregate uuid is in both positive and negative
    /// expression to return 400 BadRequest. We still return 200 for such
    /// cases. For example: `member_of=AGGA_UUID&member_of=!AGGA_UUID` would
    /// return empty allocation_requests and provider_summaries, while:
    /// `member_of=in:AGGA_UUID,AGGB_UUID&member_of=!AGGA_UUID` would return
    /// resource providers that are NOT in AGGA but in AGGB.
    #[builder(default, private, setter(name = "_member_of"))]
    member_of: Option<Vec<Cow<'a, str>>>,

    /// The name of a resource provider to filter the list.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// A comma-separated list of traits that a provider must have:
    /// `required=HW_CPU_X86_AVX,HW_CPU_X86_SSE` Allocation requests in the
    /// response will be for resource providers that have capacity for all
    /// requested resources and the set of those resource providers will
    /// collectively contain all of the required traits. These traits may be
    /// satisfied by any provider in the same non-sharing tree or associated
    /// via aggregate as far as that provider also contributes resource to the
    /// request. Starting from microversion 1.22 traits which are forbidden
    /// from any resource provider contributing resources to the request may be
    /// expressed by prefixing a trait with a `!`. Starting from microversion
    /// 1.39 the required query parameter can be repeated. The trait lists from
    /// the repeated parameters are AND-ed together. So:
    /// `required=T1,!T2&required=T3` means T1 and not T2 and T3. Also starting
    /// from microversion 1.39 the required parameter supports the syntax:
    /// `required=in:T1,T2,T3` which means T1 or T2 or T3. Mixing forbidden
    /// traits into an in: prefixed value is not supported and rejected. But
    /// mixing a normal trait list and an in: prefixed trait list in two query
    /// params within the same request is supported. So:
    /// `required=in:T3,T4&required=T1,!T2` is supported and it means T1 and
    /// not T2 and (T3 or T4).
    #[builder(default, private, setter(name = "_required"))]
    required: Option<Vec<Cow<'a, str>>>,

    /// A comma-separated list of strings indicating an amount of resource of a
    /// specified class that providers in each allocation request must
    /// collectively have the capacity and availability to serve:
    /// `resources=VCPU:4,DISK_GB:64,MEMORY_MB:2048` These resources may be
    /// satisfied by any provider in the same non-sharing tree or associated
    /// via aggregate.
    #[builder(default, setter(into))]
    resources: Option<Cow<'a, str>>,

    /// The uuid of a resource provider.
    #[builder(default, setter(into))]
    uuid: Option<Cow<'a, str>>,

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
    /// A string representing an aggregate uuid; or the prefix in: followed by
    /// a comma-separated list of strings representing aggregate uuids. The
    /// resource providers in the allocation request in the response must
    /// directly or via the root provider be associated with the aggregate or
    /// aggregates identified by uuid:
    /// `member_of=5e08ea53-c4c6-448e-9334-ac4953de3cfa`,
    /// `member_of=in:42896e0d-205d-4fe3-bd1e-100924931787,5e08ea53-c4c6-448e-9334-ac4953de3cfa`
    /// Starting from microversion 1.24 specifying multiple member_of query
    /// string parameters is possible. Multiple member_of parameters will
    /// result in filtering providers that are directly or via root provider
    /// associated with aggregates listed in all of the member_of query string
    /// values. For example, to get the providers that are associated with
    /// aggregate A as well as associated with any of aggregates B or C, the
    /// user could issue the following query:
    /// `member_of=AGGA_UUID&member_of=in:AGGB_UUID,AGGC_UUID` Starting from
    /// microversion 1.32 specifying forbidden aggregates is supported in the
    /// member_of query string parameter. Forbidden aggregates are prefixed
    /// with a !. This negative expression can also be used in multiple
    /// member_of parameters: `member_of=AGGA_UUID&member_of=!AGGB_UUID` would
    /// translate logically to “Candidate resource providers must be in AGGA
    /// and not in AGGB.” We do NOT support ! on the values within in:, but we
    /// support !in:. Both of the following two example queries return
    /// candidate resource providers that are NOT in AGGA, AGGB, or AGGC:
    /// `member_of=!in:AGGA_UUID,AGGB_UUID,AGGC_UUID`,
    /// `member_of=!AGGA_UUID&member_of=!AGGB_UUID&member_of=!AGGC_UUID` We do
    /// not check if the same aggregate uuid is in both positive and negative
    /// expression to return 400 BadRequest. We still return 200 for such
    /// cases. For example: `member_of=AGGA_UUID&member_of=!AGGA_UUID` would
    /// return empty allocation_requests and provider_summaries, while:
    /// `member_of=in:AGGA_UUID,AGGB_UUID&member_of=!AGGA_UUID` would return
    /// resource providers that are NOT in AGGA but in AGGB.
    pub fn member_of<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.member_of
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// A comma-separated list of traits that a provider must have:
    /// `required=HW_CPU_X86_AVX,HW_CPU_X86_SSE` Allocation requests in the
    /// response will be for resource providers that have capacity for all
    /// requested resources and the set of those resource providers will
    /// collectively contain all of the required traits. These traits may be
    /// satisfied by any provider in the same non-sharing tree or associated
    /// via aggregate as far as that provider also contributes resource to the
    /// request. Starting from microversion 1.22 traits which are forbidden
    /// from any resource provider contributing resources to the request may be
    /// expressed by prefixing a trait with a `!`. Starting from microversion
    /// 1.39 the required query parameter can be repeated. The trait lists from
    /// the repeated parameters are AND-ed together. So:
    /// `required=T1,!T2&required=T3` means T1 and not T2 and T3. Also starting
    /// from microversion 1.39 the required parameter supports the syntax:
    /// `required=in:T1,T2,T3` which means T1 or T2 or T3. Mixing forbidden
    /// traits into an in: prefixed value is not supported and rejected. But
    /// mixing a normal trait list and an in: prefixed trait list in two query
    /// params within the same request is supported. So:
    /// `required=in:T3,T4&required=T1,!T2` is supported and it means T1 and
    /// not T2 and (T3 or T4).
    pub fn required<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.required
            .get_or_insert(None)
            .get_or_insert_with(Vec::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Resource_Provider.
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
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "resource_providers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("in_tree", self.in_tree.as_ref());
        if let Some(val) = &self.member_of {
            params.extend(val.iter().map(|value| ("member_of", value)));
        }
        params.push_opt("name", self.name.as_ref());
        if let Some(val) = &self.required {
            params.extend(val.iter().map(|value| ("required", value)));
        }
        params.push_opt("resources", self.resources.as_ref());
        params.push_opt("uuid", self.uuid.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Placement
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("resource_providers".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Placement
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder().build().unwrap().response_key().unwrap(),
            "resource_providers"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/resource_providers".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource_providers": {} }));
        });

        let endpoint = Request::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/resource_providers".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource_providers": {} }));
        });

        let endpoint = Request::builder()
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
