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

// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;

use crate::api::rest_endpoint::prepare_request;
use crate::api::{ApiError, RestEndpoint};

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

/// A query modifier that ignores the data returned from an endpoint. For
/// error responses it tries to extract error information from body. It can
/// be used for example for HEAD requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ignore<E> {
    endpoint: E,
}

/// Ignore the resulting data from an endpoint.
pub fn ignore<E>(endpoint: E) -> Ignore<E> {
    Ignore { endpoint }
}

#[cfg(feature = "sync")]
impl<E, C> Query<(), C> for Ignore<E>
where
    E: RestEndpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let ep = client.get_service_endpoint(
            &self.endpoint.service_type(),
            self.endpoint.api_version().as_ref(),
        )?;
        let (req, data) = prepare_request::<C, E>(
            ep,
            ep.build_request_url(&self.endpoint.endpoint())?,
            &self.endpoint,
        )?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest(req, data)?;

        let status = rsp.status();
        if !status.is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(query_uri, rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_openstack(query_uri, status, v));
        }

        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, C> QueryAsync<(), C> for Ignore<E>
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let ep = client.get_service_endpoint(
            &self.endpoint.service_type(),
            self.endpoint.api_version().as_ref(),
        )?;
        let (req, data) = prepare_request::<C, E>(
            ep,
            ep.build_request_url(&self.endpoint.endpoint())?,
            &self.endpoint,
        )?;

        let query_uri = req.uri_ref().cloned();
        let rsp = client.rest_async(req, data).await?;

        let status = rsp.status();
        if !status.is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(query_uri, rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_openstack(query_uri, status, v));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use http::StatusCode;
    use httpmock::MockServer;
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "async")]
    use crate::api::QueryAsync;
    use crate::api::{self, ApiError};
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;

    struct Dummy;

    impl RestEndpoint for Dummy {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }

        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_openstack_non_json_response() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::OK.into()).body("not json");
        });

        api::ignore(Dummy).query(&client).unwrap();
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_openstack_non_json_response_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::OK.into()).body("not json");
        });

        api::ignore(Dummy).query_async(&client).await.unwrap();
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_openstack_error_bad_json() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into());
        });

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_openstack_error_bad_json_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into());
        });

        let err = api::ignore(Dummy).query_async(&client).await.unwrap_err();
        if let ApiError::OpenStackService { status, .. } = err {
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_openstack_error_detection() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_openstack_error_detection_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });

        let err = api::ignore(Dummy).query_async(&client).await.unwrap_err();
        if let ApiError::OpenStack { msg, .. } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_openstack_error_detection_unknown() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let err_obj = json!({"bogus": "dummy error message"});
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(err_obj.clone());
        });

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::OpenStackUnrecognized { obj, .. } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_openstack_error_detection_unknown_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let err_obj = json!({"bogus": "dummy error message"});
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(err_obj.clone());
        });

        let err = api::ignore(Dummy).query_async(&client).await.unwrap_err();
        if let ApiError::OpenStackUnrecognized { obj, .. } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }
}
