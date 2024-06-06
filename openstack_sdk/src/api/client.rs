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

use http::request::Builder as RequestBuilder;
use http::{HeaderMap, Response};

use crate::api::ApiError;
use crate::catalog::ServiceEndpoint;
//use crate::types::api_version::ApiVersion;
use crate::types::{identity::v3::Project, ApiVersion, BoxedAsyncRead, ServiceType};

/// A trait representing a client which can communicate with a OpenStack service API via REST API.
pub trait RestClient {
    /// The errors which may occur for this client.
    type Error: Error + Send + Sync + 'static;

    /// Get current token project information
    fn get_current_project(&self) -> Option<Project>;

    /// Get service endpoint information
    fn get_service_endpoint(
        &self,
        service_type: &ServiceType,
        version: Option<&ApiVersion>,
    ) -> Result<&ServiceEndpoint, ApiError<Self::Error>>;
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
