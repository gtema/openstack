use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(untagged)]
pub enum VnicType {
    #[serde(alias = "baremetal")]
    Baremetal,
    #[serde(alias = "normal")]
    Normal,
    #[serde(alias = "macvtap")]
    Macvtap,
    #[serde(alias = "smart-nic")]
    SmartNic,
    #[serde(alias = "vdpa")]
    Vdpa,
    #[serde(alias = "direct-physical")]
    DirectPhysical,
    #[serde(alias = "virtio-forwarder")]
    VirtioForwarder,
    #[serde(alias = "remote-managed")]
    RemoteManaged,
    #[serde(alias = "accelerator-direct-physical")]
    AcceleratorDirectPhysical,
    #[serde(alias = "accelerator-direct")]
    AcceleratorDirect,
    #[serde(alias = "direct")]
    Direct,
}

#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Binding<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    host: Option<Cow<'a, str>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    vnic_type: Option<VnicType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_profile"))]
    profile: Option<Option<BTreeMap<Cow<'a, str>, Value>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,
}

impl<'a> BindingBuilder<'a> {
    pub fn profile<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Value>,
    {
        self.profile
            .get_or_insert(None)
            .get_or_insert(None)
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into))]
    binding: Binding<'a>,

    /// port_id parameter for /v2.0/ports/{port_id}/add_allowed_address_pairs
    /// API
    #[builder(setter(into), default)]
    port_id: Cow<'a, str>,

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
    /// Add a single header to the Binding.
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
        Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "v2.0/ports/{port_id}/bindings",
            port_id = self.port_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("binding", serde_json::to_value(&self.binding)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("binding".into())
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
                .binding(BindingBuilder::default().build().unwrap())
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
                .binding(BindingBuilder::default().build().unwrap())
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "binding"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/v2.0/ports/{port_id}/bindings",
                port_id = "port_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "binding": {} }));
        });

        let endpoint = Request::builder()
            .port_id("port_id")
            .binding(BindingBuilder::default().build().unwrap())
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
                    "/v2.0/ports/{port_id}/bindings",
                    port_id = "port_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "binding": {} }));
        });

        let endpoint = Request::builder()
            .port_id("port_id")
            .binding(BindingBuilder::default().build().unwrap())
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
