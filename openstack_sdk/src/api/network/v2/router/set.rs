//! Updates a logical router.
//!
//! This operation does not enable the update of router interfaces.
//! To update a router interface, use the add router interface and
//! remove router interface operations.
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 412
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ExternalFixedIps<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    ip_address: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    subnet_id: Option<Cow<'a, str>>,
}

/// The external gateway information of the router.
/// If the router has an external gateway, this would be a dict with
/// `network\_id`, `enable\_snat`, `external\_fixed\_ips` and
/// `qos\_policy\_id`.
/// Otherwise, this would be `null`.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct ExternalGatewayInfo<'a> {
    #[serde()]
    #[builder(setter(into))]
    network_id: Cow<'a, str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    enable_snat: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    external_fixed_ips: Option<Vec<ExternalFixedIps<'a>>>,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Routes<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    destination: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    nexthop: Option<Cow<'a, str>>,
}

/// A `router` object.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Router<'a> {
    /// Human-readable name of the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The administrative state of the resource, which is
    /// up (`true`) or down (`false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    admin_state_up: Option<bool>,

    /// The external gateway information of the router.
    /// If the router has an external gateway, this would be a dict with
    /// `network\_id`, `enable\_snat`, `external\_fixed\_ips` and
    /// `qos\_policy\_id`.
    /// Otherwise, this would be `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    external_gateway_info: Option<ExternalGatewayInfo<'a>>,

    /// `true` indicates a highly-available router.
    /// It is available when `l3-ha` extension is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    ha: Option<Option<bool>>,

    /// Enable NDP proxy attribute. Default is `false`, To persist this
    /// attribute
    /// value, set the `enable\_ndp\_proxy\_by\_default` option in the
    /// `neutron.conf` file. It is available when `router-extend-ndp-proxy`
    /// extension is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    enable_ndp_proxy: Option<Option<bool>>,

    /// `true` indicates a distributed router.
    /// It is available when `dvr` extension is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    distributed: Option<Option<bool>>,

    /// The extra routes configuration for L3 router.
    /// A list of dictionaries with `destination` and `nexthop` parameters.
    /// It is available when `extraroute` extension is enabled.
    /// Default is an empty list (`[]`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    routes: Option<Vec<Routes<'a>>>,

    /// A human-readable description for the resource.
    /// Default is an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// A `router` object.
    #[builder(setter(into))]
    router: Router<'a>,

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
    fn method(&self) -> Method {
        Method::PUT
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("v2.0/routers/{id}", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("router", serde_json::to_value(&self.router)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("router".into())
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
                .router(RouterBuilder::default().build().unwrap())
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
                .router(RouterBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "router"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2.0/routers/{id}", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .router(RouterBuilder::default().build().unwrap())
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::PUT)
                .path(format!("/v2.0/routers/{id}", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .router(RouterBuilder::default().build().unwrap())
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
