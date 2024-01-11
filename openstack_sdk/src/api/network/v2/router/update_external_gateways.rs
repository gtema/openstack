//! Update some external gateways of router.
//!
//! For general information on the add/update/remove external gateways
//! operations see `add\_external\_gateways` above.
//!
//! The external gateways to be updated are identified by the `network\_ids`
//! found in the PUT request. The `external\_fixed\_ips`, `enable\_snat`,
//! fields can be updated. The `network\_id` field cannot be updated - any
//! changes will cause a gateway port to be removed and recreated.
//!
//! The format of the request body is the same as the format of the read-only
//! `router.external\_gateways` parameter, but wrapped as follows:
//!
//! The `enable\_snat` field does not have any effect for extra gateways except
//! for the first external gateway in the list.
//!
//! The `network\_id` field is used to identify a particular gateway port along
//! with the `external\_fixed\_ips` field. Specifying just the `network\_id`
//! field
//! is ambiguous: Neutron will attempt to find the matching gateway port but if
//! there are multiple matches it will return an error response code.
//!
//! The `enable\_snat` field can be omitted from the request. Specifying
//! `external\_fixed\_ips` will result in matching ports based on those
//! fixed IPs. If a gateway port has a subset of the specified fixed IPs,
//! then the set of IPs will be updated to match the ones in the request.
//! Alternatively, if a gateway port has a superset of fixed IPs from the
//! request the IPs will be removed from the gateway port.
//!
//! The response codes and response body are the same as to the update of
//! the router. That is the whole router object is returned including the
//! `external\_gateway\_info` and `external\_gateways` parameters which
//! represents the result of the operation.
//!
//! Please note that updating `external\_gateway\_info` also updates
//! the first element of `external\_gateways` and it leaves the rest of
//! `external\_gateways` unchanged.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 412
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// id parameter for /v2.0/routers/{id} API
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
    /// Add a single header to the Router.
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
        http::Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.0/routers/{id}/update_external_gateways",
            id = self.id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
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
            Request::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT).path(format!(
                "/v2.0/routers/{id}/update_external_gateways",
                id = "id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder().id("id").build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!(
                    "/v2.0/routers/{id}/update_external_gateways",
                    id = "id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
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
