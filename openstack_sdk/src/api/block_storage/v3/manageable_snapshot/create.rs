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

//! Instruct Cinder to manage a storage snapshot object.
//!
//! Manages an existing backend storage snapshot object (e.g. a Linux logical
//! volume or a SAN disk) by creating the Cinder objects required to manage it,
//! and possibly renaming the backend storage snapshot object (driver
//! dependent).
//!
//! From an API perspective, this operation behaves very much like a snapshot
//! creation operation.
//!
//! Required HTTP Body:
//!
//! ```text
//!
//! {
//!   "snapshot":
//!   {
//!     "volume_id": "<Cinder volume already exists in volume backend>",
//!     "ref":
//!        "<Driver-specific reference to the existing storage object>"
//!   }
//! }
//!
//! ```
//!
//! See the appropriate Cinder drivers' implementations of the manage_snapshot
//! method to find out the accepted format of 'ref'. For example,in LVM driver,
//! it will be the logic volume name of snapshot which you want to manage.
//!
//! This API call will return with an error if any of the above elements are
//! missing from the request, or if the 'volume_id' element refers to a cinder
//! volume that could not be found.
//!
//! The snapshot will later enter the error state if it is discovered that
//! 'ref' is bad.
//!
//! Optional elements to 'snapshot' are:
//!
//! ```text
//!
//! name           A name for the new snapshot.
//! description    A description for the new snapshot.
//! metadata       Key/value pairs to be associated with the new snapshot.
//!
//! ```
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeMap;

/// A `snapshot` object.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Snapshot<'a> {
    /// A description for the snapshot. Default is `None`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) description: Option<Option<Cow<'a, str>>>,

    /// One or more metadata key and value pairs for the snapshot.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, private, setter(name = "_metadata"))]
    pub(crate) metadata: Option<Option<BTreeMap<Cow<'a, str>, Cow<'a, str>>>>,

    /// The name of the snapshot. Default is `None`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Option<Cow<'a, str>>>,

    /// A reference to the existing volume. The internal structure of this
    /// reference depends on the volume driver implementation. For details
    /// about the required elements in the structure, see the documentation for
    /// the volume driver.
    ///
    #[serde(rename = "ref")]
    #[builder(setter(into))]
    pub(crate) _ref: Option<Value>,

    /// The UUID of the volume.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) volume_id: Cow<'a, str>,
}

impl<'a> SnapshotBuilder<'a> {
    /// One or more metadata key and value pairs for the snapshot.
    ///
    pub fn metadata<I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.metadata
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
    /// A `snapshot` object.
    ///
    #[builder(setter(into))]
    pub(crate) snapshot: Snapshot<'a>,

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
    /// Add a single header to the Manageable_Snapshot.
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
        "manageable_snapshots".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("snapshot", serde_json::to_value(&self.snapshot)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::BlockStorage
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("snapshot".into())
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
            Request::builder()
                .snapshot(
                    SnapshotBuilder::default()
                        ._ref(json!({}))
                        .volume_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::BlockStorage
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .snapshot(
                    SnapshotBuilder::default()
                        ._ref(json!({}))
                        .volume_id("foo")
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "snapshot"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/manageable_snapshots".to_string());

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "snapshot": {} }));
        });

        let endpoint = Request::builder()
            .snapshot(
                SnapshotBuilder::default()
                    ._ref(json!({}))
                    .volume_id("foo")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST)
                .path("/manageable_snapshots".to_string())
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "snapshot": {} }));
        });

        let endpoint = Request::builder()
            .snapshot(
                SnapshotBuilder::default()
                    ._ref(json!({}))
                    .volume_id("foo")
                    .build()
                    .unwrap(),
            )
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
