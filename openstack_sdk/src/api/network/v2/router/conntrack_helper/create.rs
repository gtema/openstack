//! Creates a router conntrack helper.
//!
//! Normal response codes: 201
//!
//! Error response codes: 400, 404
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum Protocol {
    #[serde(rename = "dccp")]
    Dccp,
    #[serde(rename = "icmp")]
    Icmp,
    #[serde(rename = "ipv6-icmp")]
    Ipv6Icmp,
    #[serde(rename = "sctp")]
    Sctp,
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
}

/// A router `conntrack helper` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ConntrackHelper<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// The network protocol for the netfilter conntrack target rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    protocol: Option<Protocol>,

    /// The network port for the netfilter conntrack target rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    port: Option<f32>,

    /// The netfilter conntrack helper module.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    helper: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A router `conntrack helper` object.
    #[builder(setter(into))]
    conntrack_helper: ConntrackHelper<'a>,

    /// router_id parameter for /v2.0/routers/{router_id}/tags/{id} API
    #[builder(default, setter(into))]
    router_id: Cow<'a, str>,

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
    /// Add a single header to the Conntrack_Helper.
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
        format!(
            "v2.0/routers/{router_id}/conntrack_helpers",
            router_id = self.router_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push(
            "conntrack_helper",
            serde_json::to_value(&self.conntrack_helper)?,
        );

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("conntrack_helper".into())
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
                .conntrack_helper(ConntrackHelperBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .conntrack_helper(ConntrackHelperBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "conntrack_helper"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.0/routers/{router_id}/conntrack_helpers",
                router_id = "router_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "conntrack_helper": {} }));
        });

        let endpoint = Request::builder()
            .router_id("router_id")
            .conntrack_helper(ConntrackHelperBuilder::default().build().unwrap())
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
                .path(format!(
                    "/v2.0/routers/{router_id}/conntrack_helpers",
                    router_id = "router_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "conntrack_helper": {} }));
        });

        let endpoint = Request::builder()
            .router_id("router_id")
            .conntrack_helper(ConntrackHelperBuilder::default().build().unwrap())
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
