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

use std::error::Error;

use async_trait::async_trait;
use bytes::Bytes;
use url::Url;

use http::request::Builder as RequestBuilder;
use http::{HeaderMap, Response};

use crate::api::ApiError;
use crate::catalog::ServiceEndpoint;
use crate::types::{identity::v3::Project, BoxedAsyncRead, ServiceType};

/// A trait representing a client which can communicate with a OpenStack service API via REST API.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Construct final URL for the resource given the service type and RestEndpoint
    fn rest_endpoint(
        &self,
        service_type: &ServiceType,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        let service_url = self.get_service_endpoint(service_type)?.url;
        self.construct_endpoint_url(service_url, endpoint)
    }

    /// Combine service url with the rest_endpoint url to construct final request URL.
    ///
    /// This does deduplicate path segments to prevent service_url path conflicting with the
    /// endpoint url prefix.
    fn construct_endpoint_url(
        &self,
        service_url: Url,
        endpoint: &str,
    ) -> Result<Url, ApiError<Self::Error>> {
        let work_service_url = service_url;
        let mut work_endpoint = endpoint;
        if let Some(segments) = work_service_url.path_segments() {
            // Service catalog may point to /v2.1/ and target endpoint start
            // with v2.1/servers. The same may happen also for project_id being
            // used in the service catalog while rest endpoint also contain it.
            // In order to construct proper url look in the path elements of
            // the service catalog and for each entry ensure target url does
            // not start with that value.
            let mut overlap: bool = false;
            for part in segments.filter(|x| !x.is_empty()) {
                if work_endpoint.starts_with(part) {
                    work_endpoint = work_endpoint
                        .get(part.len() + 1..)
                        .expect("Cannot remove prefix from url");
                    overlap = true;
                } else if overlap {
                    break;
                }
            }
        }
        Ok(work_service_url.join(work_endpoint)?)
    }

    /// Get current token project information
    fn get_current_project(&self) -> Option<Project>;

    /// Get service endpoint information
    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
    ) -> Result<ServiceEndpoint, ApiError<Self::Error>>;
}

/// A trait representing a client which can communicate with a OpenStack cloud APIs.
#[cfg(feature = "sync")]
pub trait Client: RestClient {
    /// Send a REST query.
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

/// A trait representing an asynchronous client which can communicate with OpenStack cloud.
#[cfg(feature = "async")]
#[async_trait]
pub trait AsyncClient: RestClient {
    /// Send a REST query asynchronously.
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;

    /// Send a REST query asynchronously.
    async fn rest_read_body_async(
        &self,
        request: RequestBuilder,
        body: BoxedAsyncRead,
    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;

    /// Send a REST query asynchronously and return body as AsyncRead.
    async fn download_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<(HeaderMap, BoxedAsyncRead), ApiError<Self::Error>>;

    // Send a REST query asynchronously with body provided by AsyncRead.
    //    async fn upload_async(
    //        &self,
    //        request: RequestBuilder,
    //        body: BoxedAsyncRead,
    //    ) -> Result<Response<Bytes>, ApiError<Self::Error>>;
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::error::RestError;

    struct TestClient {}
    impl RestClient for TestClient {
        type Error = RestError;
        fn rest_endpoint(
            &self,
            _service_type: &ServiceType,
            _endpoint: &str,
        ) -> Result<Url, ApiError<Self::Error>> {
            todo!();
        }

        fn get_current_project(&self) -> Option<Project> {
            todo!();
        }

        fn get_service_endpoint(
            &self,
            _service_type: &ServiceType,
        ) -> Result<ServiceEndpoint, ApiError<Self::Error>> {
            todo!();
        }
    }

    #[test]
    fn test_construct_endpoint_url() {
        let client = TestClient {};
        let map = [
            ("http://foo.bar/", "", "http://foo.bar/"),
            ("http://foo.bar/", "info", "http://foo.bar/info"),
            ("http://foo.bar/", "v1", "http://foo.bar/v1"),
            (
                "http://foo.bar/",
                "v1/resource",
                "http://foo.bar/v1/resource",
            ),
            (
                "http://foo.bar/v1/",
                "v1/resource",
                "http://foo.bar/v1/resource",
            ),
            (
                "http://foo.bar/v1/PROJECT_ID/",
                "resources",
                "http://foo.bar/v1/PROJECT_ID/resources",
            ),
            (
                "http://foo.bar/prefix/",
                "info",
                "http://foo.bar/prefix/info",
            ),
            (
                "http://foo.bar/prefix/v1/",
                "v1/info",
                "http://foo.bar/prefix/v1/info",
            ),
        ];
        for (service_url, endpoint, expected) in map {
            assert_eq!(
                client
                    .construct_endpoint_url(Url::parse(service_url).unwrap(), endpoint)
                    .unwrap(),
                Url::parse(expected).unwrap(),
                "Endpoint: {} with URL: {} results in {}",
                service_url,
                endpoint,
                expected
            );
        }
    }
}
