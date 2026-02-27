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

//! GET operation on /v3/auth/OS-FEDERATION/websso/{protocol_id}
//!
use derive_builder::Builder;

use crate::api::rest_endpoint_prelude::*;

use std::borrow::Cow;

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// protocol_id parameter for /v3/auth/OS-FEDERATION/websso/{protocol_id}
    /// API
    #[builder(default, setter(into))]
    protocol_id: Cow<'a, str>,
}

impl RestEndpoint for Request<'_> {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!(
            "auth/OS-FEDERATION/websso/{protocol_id}",
            protocol_id = self.protocol_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams<'_> {
        QueryParams::default()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Identity
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("token".into())
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(3, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "sync")]
    use crate::api::Query;
    use crate::test::client::FakeOpenStackClient;
    use crate::types::ServiceType;
    use httpmock::MockServer;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            RequestBuilder::default().build().unwrap().service_type(),
            ServiceType::Identity
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            RequestBuilder::default()
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "token"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!(
                "/auth/OS-FEDERATION/websso/{protocol_id}",
                protocol_id = "protocol_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "token": {} }));
        });

        let endpoint = RequestBuilder::default()
            .protocol_id("protocol_id")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
