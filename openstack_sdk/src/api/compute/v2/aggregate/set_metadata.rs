use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;
use serde::Serialize;

use serde::Deserialize;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// The set\_metadata object used to set metadata for host aggregate.
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct SetMetadata<'a> {
    /// Metadata key and value pairs associated with the aggregate.
    /// The maximum size for each metadata key and value pair is 255 bytes.
    ///
    ///
    /// New keys will be added to existing aggregate metadata. For existing
    /// keys, if the value is `null` the entry is removed, otherwise the
    /// value is updated. Note that the special `availability\_zone` metadata
    /// entry cannot be unset to `null`.
    ///
    ///
    ///
    /// Warning
    ///
    ///
    /// You should not change the availability zone of an
    /// aggregate when that aggregate has hosts which contain servers in it
    /// since that may impact the ability for those servers to move to another
    /// host.
    #[serde()]
    #[builder(private, setter(name = "_metadata"))]
    pub(crate) metadata: BTreeMap<Cow<'a, str>, Option<Cow<'a, str>>>,
}

impl<'a> SetMetadataBuilder<'a> {
    /// Metadata key and value pairs associated with the aggregate.
    /// The maximum size for each metadata key and value pair is 255 bytes.
    ///
    ///
    /// New keys will be added to existing aggregate metadata. For existing
    /// keys, if the value is `null` the entry is removed, otherwise the
    /// value is updated. Note that the special `availability\_zone` metadata
    /// entry cannot be unset to `null`.
    ///
    ///
    ///
    /// Warning
    ///
    ///
    /// You should not change the availability zone of an
    /// aggregate when that aggregate has hosts which contain servers in it
    /// since that may impact the ability for those servers to move to another
    /// host.
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Option<Cow<'a, str>>>,
    {
        self.metadata
            .get_or_insert_with(BTreeMap::new)
            .extend(iter.map(|(k, v)| (k.into(), v.into())));
        self
    }
}
#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The set\_metadata object used to set metadata for host aggregate.
    #[builder(setter(into))]
    pub(crate) set_metadata: SetMetadata<'a>,

    /// id parameter for /v2.1/os-aggregates/{id}/images API
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
    /// Add a single header to the Aggregate.
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
        format!("v2.1/os-aggregates/{id}/action", id = self.id.as_ref(),).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("set_metadata", serde_json::to_value(&self.set_metadata)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Compute
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("aggregate".into())
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
                .set_metadata(
                    SetMetadataBuilder::default()
                        .metadata(
                            BTreeMap::<String, Option<String>>::new()
                                .into_iter()
                                .map(|(k, v)| (k, v.map(Into::into)))
                        )
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::Compute
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .set_metadata(
                    SetMetadataBuilder::default()
                        .metadata(
                            BTreeMap::<String, Option<String>>::new()
                                .into_iter()
                                .map(|(k, v)| (k, v.map(Into::into)))
                        )
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "aggregate"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path(format!("/v2.1/os-aggregates/{id}/action", id = "id",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "aggregate": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .set_metadata(
                SetMetadataBuilder::default()
                    .metadata(
                        BTreeMap::<String, Option<String>>::new()
                            .into_iter()
                            .map(|(k, v)| (k, v.map(Into::into))),
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
                .path(format!("/v2.1/os-aggregates/{id}/action", id = "id",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "aggregate": {} }));
        });

        let endpoint = Request::builder()
            .id("id")
            .set_metadata(
                SetMetadataBuilder::default()
                    .metadata(
                        BTreeMap::<String, Option<String>>::new()
                            .into_iter()
                            .map(|(k, v)| (k, v.map(Into::into))),
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
