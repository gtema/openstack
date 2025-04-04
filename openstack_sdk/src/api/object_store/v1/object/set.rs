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

//! Creates or updates object metadata.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// The unique name for the account. An account is also known as the
    /// project or tenant.
    ///
    #[builder(default, setter(into))]
    account: Cow<'a, str>,

    /// When the bulk-delete query parameter is present in the POST request,
    /// multiple objects or containers can be deleted with a single request.
    /// See Bulk Delete for how this feature is used.
    ///
    #[builder(default)]
    bulk_delete: Option<bool>,

    /// The unique (within an account) name for the container. The container
    /// name must be from 1 to 256 characters long and can start with any
    /// character and contain any pattern. Character set must be UTF-8. The
    /// container name cannot contain a slash (/) character because this
    /// character delimits the container and object name. For example, the path
    /// /v1/account/www/pages specifies the www container, not the www/pages
    /// container.
    ///
    #[builder(default, setter(into))]
    container: Cow<'a, str>,

    /// When the extract-archive query parameter is present in the POST
    /// request, an archive (tar file) is uploaded and extracted to create
    /// multiple objects. See Extract Archive for how this feature is used.
    ///
    #[builder(default)]
    extract_archive: Option<bool>,

    /// Overrides the default file name. Object Storage generates a default
    /// file name for GET temporary URLs that is based on the object name.
    /// Object Storage returns this value in the Content-Disposition response
    /// header. Browsers can interpret this file name value as a file
    /// attachment to save. For more information about temporary URLs, see
    /// Temporary URL middleware.
    ///
    #[builder(default, setter(into))]
    filename: Option<Cow<'a, str>>,

    /// If you include the multipart-manifest=get query parameter and the
    /// object is a large object, the object contents are not returned.
    /// Instead, the manifest is returned in the X-Object-Manifest response
    /// header for dynamic large objects or in the response body for static
    /// large objects.
    ///
    #[builder(default, setter(into))]
    multipart_manifest: Option<Cow<'a, str>>,

    /// The unique name for the object.
    ///
    #[builder(default, setter(into))]
    object: Cow<'a, str>,

    /// If you include the symlink=get query parameter and the object is a
    /// symlink, then the response will include data and metadata from the
    /// symlink itself rather than from the target.
    ///
    #[builder(default, setter(into))]
    symlink: Option<Cow<'a, str>>,

    /// The date and time in UNIX Epoch time stamp format or ISO 8601 UTC
    /// timestamp when the signature for temporary URLs expires. For example,
    /// 1440619048 or 2015-08-26T19:57:28Z is equivalent to Mon, Wed, 26 Aug
    /// 2015 19:57:28 GMT. For more information about temporary URLs, see
    /// Temporary URL middleware.
    ///
    #[builder(default)]
    temp_url_expires: Option<i32>,

    /// Used with temporary URLs to sign the request with an HMAC-SHA1
    /// cryptographic signature that defines the allowed HTTP method,
    /// expiration date, full path to the object, and the secret key for the
    /// temporary URL. For more information about temporary URLs, see Temporary
    /// URL middleware.
    ///
    #[builder(default, setter(into))]
    temp_url_sig: Option<Cow<'a, str>>,

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
    /// Add a single header to the Object.
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
        http::Method::POST
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "{account}/{container}/{object}",
            account = self.account.as_ref(),
            container = self.container.as_ref(),
            object = self.object.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("multipart-manifest", self.multipart_manifest.as_ref());
        params.push_opt("temp_url_sig", self.temp_url_sig.as_ref());
        params.push_opt("temp_url_expires", self.temp_url_expires);
        params.push_opt("filename", self.filename.as_ref());
        params.push_opt("symlink", self.symlink.as_ref());
        params.push_opt("bulk-delete", self.bulk_delete);
        params.push_opt("extract-archive", self.extract_archive);

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::ObjectStore
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        None
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(1, 0))
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
            ServiceType::ObjectStore
        );
    }

    #[test]
    fn test_response_key() {
        assert!(Request::builder().build().unwrap().response_key().is_none())
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/{account}/{container}/{object}",
                account = "account",
                container = "container",
                object = "object",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .account("account")
            .container("container")
            .object("object")
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
            when.method(httpmock::Method::POST)
                .path(format!(
                    "/{account}/{container}/{object}",
                    account = "account",
                    container = "container",
                    object = "object",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Request::builder()
            .account("account")
            .container("container")
            .object("object")
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
