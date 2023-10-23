// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use async_trait::async_trait;
use http::{header, HeaderMap, Request};

use crate::api::rest_endpoint::prepare_request;
use crate::api::{query, ApiError, AsyncClient, Client, Query, QueryAsync, RestEndpoint};

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

impl<E, C> Query<(), C> for Ignore<E>
where
    E: RestEndpoint,
    C: Client,
{
    fn query(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url =
            client.rest_endpoint(&self.endpoint.service_type(), &self.endpoint.endpoint())?;
        let (req, data) = prepare_request::<E>(url, &self.endpoint)?;

        let rsp = client.rest(req, data)?;

        let status = rsp.status();
        if !status.is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_openstack(status, v));
        }

        Ok(())
    }
}

#[async_trait]
impl<E, C> QueryAsync<(), C> for Ignore<E>
where
    E: RestEndpoint + Sync,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<(), ApiError<C::Error>> {
        let mut url =
            client.rest_endpoint(&self.endpoint.service_type(), &self.endpoint.endpoint())?;

        let (req, data) = prepare_request::<E>(url, &self.endpoint)?;

        let rsp = client.rest_async(req, data).await?;

        let status = rsp.status();
        if !status.is_success() {
            let v = if let Ok(v) = serde_json::from_slice(rsp.body()) {
                v
            } else {
                return Err(ApiError::server_error(rsp.status(), rsp.body()));
            };
            return Err(ApiError::from_openstack(status, v));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderMap;
    use http::StatusCode;
    use serde_json::json;

    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{self, ApiError, Query, QueryAsync};
    use crate::test::client::{MockAsyncServerClient, MockServerClient};
    use crate::types::ServiceType;

    struct Dummy;

    impl RestEndpoint for Dummy {
        fn method(&self) -> Method {
            Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummy".into()
        }

        fn service_type(&self) -> ServiceType {
            ServiceType::Other("dummy".to_string())
        }
    }

    #[derive(Debug)]
    struct DummyResult {
        #[allow(dead_code)]
        value: u8,
    }

    #[test]
    fn test_openstack_non_json_response() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).body("not json");
        });

        api::ignore(Dummy).query(&client).unwrap();
        mock.assert();
    }

    #[tokio::test]
    async fn test_openstack_non_json_response_async() {
        let client = MockAsyncServerClient::new().await;
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(200).body("not json");
        });

        api::ignore(Dummy).query_async(&client).await.unwrap();
        mock.assert();
    }

    #[test]
    fn test_openstack_error_bad_json() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
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

    #[test]
    fn test_openstack_error_detection() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(json!({"message": "dummy error message"}));
        });

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::OpenStack { status, msg } = err {
            assert_eq!(msg, "dummy error message");
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }

    #[test]
    fn test_openstack_error_detection_unknown() {
        let client = MockServerClient::new();
        let err_obj = json!({"bogus": "dummy error message"});
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummy");
            then.status(StatusCode::CONFLICT.into())
                .json_body(err_obj.clone());
        });

        let err = api::ignore(Dummy).query(&client).unwrap_err();
        if let ApiError::OpenStackUnrecognized { status, obj } = err {
            assert_eq!(obj, err_obj);
        } else {
            panic!("unexpected error: {}", err);
        }
        mock.assert();
    }
}
