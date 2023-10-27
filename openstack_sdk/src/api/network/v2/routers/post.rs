//! Create Router
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeSet;

/// The external gateway information of the router. If the router has an
/// external gateway, this would be a dict with network_id, enable_snat,
/// external_fixed_ips and qos_policy_id. Otherwise, this would be null.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ExternalGatewayInfo<'a> {
    /// Shared NAT
    #[serde(skip_serializing_if = "Option::is_none")]
    enable_snat: Option<bool>,

    /// Fixed IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    external_fixed_ips: Option<ExternalFixedIps<'a>>,

    /// ID of the network
    #[serde(skip_serializing_if = "Option::is_none")]
    network_id: Option<Cow<'a, str>>,
}

/// Fixed IPs
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ExternalFixedIps<'a> {
    /// IP address
    #[serde(skip_serializing_if = "Option::is_none")]
    ip_address: Option<Cow<'a, str>>,

    /// Subnet ID
    #[serde(skip_serializing_if = "Option::is_none")]
    subnet_id: Option<Cow<'a, str>>,
}

/// Query for router.post operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Router<'a> {
    /// The administrative state of the router, which is up ``True`` or down
    /// ``False``.
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// Availability zone hints to use when scheduling the router.
    #[builder(default, private, setter(name = "_availability_zone_hints"))]
    availability_zone_hints: BTreeSet<Cow<'a, str>>,

    /// The router description.
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// The distributed state of the router, which is distributed ``True`` or
    /// not ``False``.
    #[builder(default)]
    is_distributed: Option<bool>,

    /// The ndp proxy state of the router
    #[builder(default)]
    enable_ndp_proxy: Option<bool>,

    /// The external gateway information of the router. If the router has an
    /// external gateway, this would be a dict with network_id, enable_snat,
    /// external_fixed_ips and qos_policy_id. Otherwise, this would be null.
    #[builder(default, setter(into))]
    external_gateway_info: Option<ExternalGatewayInfo<'a>>,

    /// The ID of the flavor.
    #[builder(default, setter(into))]
    flavor_id: Option<Cow<'a, str>>,

    /// The highly-available state of the router, which is highly available
    /// ``True`` or not ``False``.
    #[builder(default)]
    is_ha: Option<bool>,

    /// The router name.
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// The ID of the project this router is associated with.
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// Tenant_id (deprecated attribute).
    #[builder(default, setter(into))]
    tenant_id: Option<Cow<'a, str>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Router<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RouterBuilder<'a> {
        RouterBuilder::default()
    }
}

impl<'a> RouterBuilder<'a> {
    /// Availability zone hints to use when scheduling the router.
    pub fn availability_zone_hints<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.availability_zone_hints
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

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

impl<'a> RestEndpoint for Router<'a> {
    fn method(&self) -> Method {
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "routers".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push("availability_zone_hints", &self.availability_zone_hints);
        params.push_opt("description", self.description.as_ref());
        params.push_opt("distributed", self.is_distributed);
        params.push_opt("enable_ndp_proxy", self.enable_ndp_proxy);
        params.push_opt("external_gateway_info", self.external_gateway_info.as_ref());
        params.push_opt("flavor_id", self.flavor_id.as_ref());
        params.push_opt("ha", self.is_ha);
        params.push_opt("name", self.name.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("tenant_id", self.tenant_id.as_ref());
        params.into_body_with_root_key("router")
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
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Router::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Router::builder().build().unwrap().response_key().unwrap(),
            "router"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/routers",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Router::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/routers",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "router": {} }));
        });

        let endpoint = Router::builder()
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

    #[test]
    fn endpoint_body() {
        let endpoint = Router::builder()
            .availability_zone_hints(["availability_zone_hints"].iter().cloned())
            .description("description")
            .flavor_id("flavor_id")
            .name("name")
            .project_id("project_id")
            .tenant_id("tenant_id")
            .build()
            .unwrap();

        let (mime, body) = endpoint.body().unwrap().unwrap();
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({
              "router": {
                 "availability_zone_hints": ["availability_zone_hints"],
                 "description": "description",
                 "flavor_id": "flavor_id",
                 "name": "name",
                 "project_id": "project_id",
                 "tenant_id": "tenant_id",
             }
            })
            .to_string()
        );
    }
}
