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

//! This operation provisions a member and adds it to a pool by using the
//! configuration that you define in the request object. After the API
//! validates the request and starts the provisioning process, it returns a
//! response object, which contains a unique ID.
//!
//! In the response, the member [provisioning status](#prov-status) is
//! `ACTIVE`, `PENDING_CREATE`, or `ERROR`.
//!
//! If the status is `PENDING_CREATE`, issue GET
//! `/v2/lbaas/pools/{pool_id}/members/{member_id}` to view the progress of the
//! provisioning operation. When the member status changes to `ACTIVE`, the
//! member is successfully provisioned and is ready for further configuration.
//!
//! If the API cannot fulfill the request due to insufficient data or data that
//! is not valid, the service returns the HTTP `Bad Request (400)` response
//! code with information about the failure in the response body. Validation
//! errors require that you correct the error and submit the request again.
//!
//! At a minimum, you must specify these member attributes:
//!
//! Some attributes receive default values if you omit them from the request:
//!
//! If you omit the `subnet_id` parameter, the `vip_subnet_id` for the parent
//! load balancer will be used for the member subnet UUID.
//!
//! The member `address` does not necessarily need to be a member of the
//! `subnet_id` subnet. Members can be routable from the subnet specified
//! either via the default route or by using `host_routes` defined on the
//! subnet.
//!
//! Administrative users can specify a project ID that is different than their
//! own to create members for other projects.
//!
//! `monitor_address` and/or `monitor_port` can be used to have the health
//! monitor, if one is configured for the pool, connect to an alternate IP
//! address and port when executing a health check on the member.
//!
//! To create a member, the load balancer must have an `ACTIVE` provisioning
//! status.
//!
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::rest_endpoint_prelude::*;

use serde::Deserialize;
use serde::Serialize;
use std::borrow::Cow;

/// Defines mandatory and optional attributes of a POST request.
///
#[derive(Builder, Debug, Deserialize, Clone, Serialize)]
#[builder(setter(strip_option))]
pub struct Member<'a> {
    /// The IP address of the resource.
    ///
    #[serde()]
    #[builder(setter(into))]
    pub(crate) address: Cow<'a, str>,

    /// The administrative state of the resource, which is up (`true`) or down
    /// (`false`). Default is `true`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) admin_state_up: Option<bool>,

    /// Is the member a backup? Backup members only receive traffic when all
    /// non-backup members are down.
    ///
    /// **New in version 2.1**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) backup: Option<bool>,

    /// An alternate IP address used for health monitoring a backend member.
    /// Default is `null` which monitors the member `address`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) monitor_address: Option<Cow<'a, str>>,

    /// An alternate protocol port used for health monitoring a backend member.
    /// Default is `null` which monitors the member `protocol_port`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) monitor_port: Option<i32>,

    /// Human-readable name of the resource.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) name: Option<Cow<'a, str>>,

    /// The ID of the project owning this resource. (deprecated)
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) project_id: Option<Cow<'a, str>>,

    /// The protocol port number for the resource.
    ///
    #[serde()]
    #[builder()]
    pub(crate) protocol_port: i32,

    /// Request that an SR-IOV VF be used for the member network port. Defaults
    /// to `false`.
    ///
    /// **New in version 2.29**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) request_sriov: Option<bool>,

    /// The subnet ID the member service is accessible from.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) subnet_id: Option<Cow<'a, str>>,

    /// A list of simple strings assigned to the resource.
    ///
    /// **New in version 2.5**
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tags: Option<Vec<Cow<'a, str>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into))]
    pub(crate) tenant_id: Option<Cow<'a, str>>,

    /// The weight of a member determines the portion of requests or
    /// connections it services compared to the other members of the pool. For
    /// example, a member with a weight of 10 receives five times as many
    /// requests as a member with a weight of 2. A value of 0 means the member
    /// does not receive new connections but continues to service existing
    /// connections. A valid value is from `0` to `256`. Default is `1`.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub(crate) weight: Option<i32>,
}

#[derive(Builder, Debug, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    /// Defines mandatory and optional attributes of a POST request.
    ///
    #[builder(setter(into))]
    pub(crate) member: Member<'a>,

    /// pool_id parameter for /v2/lbaas/pools/{pool_id}/members/{member_id} API
    ///
    #[builder(default, setter(into))]
    pool_id: Cow<'a, str>,

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
    /// Add a single header to the Member.
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
            "lbaas/pools/{pool_id}/members",
            pool_id = self.pool_id.as_ref(),
        )
        .into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn body(&self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let mut params = JsonBodyParams::default();

        params.push("member", serde_json::to_value(&self.member)?);

        params.into_body()
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::LoadBalancer
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("member".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }

    /// Returns required API version
    fn api_version(&self) -> Option<ApiVersion> {
        Some(ApiVersion::new(2, 0))
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
                .member(
                    MemberBuilder::default()
                        .address("foo")
                        .protocol_port(123)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .service_type(),
            ServiceType::LoadBalancer
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Request::builder()
                .member(
                    MemberBuilder::default()
                        .address("foo")
                        .protocol_port(123)
                        .build()
                        .unwrap()
                )
                .build()
                .unwrap()
                .response_key()
                .unwrap(),
            "member"
        );
    }

    #[cfg(feature = "sync")]
    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::POST).path(format!(
                "/lbaas/pools/{pool_id}/members",
                pool_id = "pool_id",
            ));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "member": {} }));
        });

        let endpoint = Request::builder()
            .pool_id("pool_id")
            .member(
                MemberBuilder::default()
                    .address("foo")
                    .protocol_port(123)
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
                .path(format!(
                    "/lbaas/pools/{pool_id}/members",
                    pool_id = "pool_id",
                ))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "member": {} }));
        });

        let endpoint = Request::builder()
            .pool_id("pool_id")
            .member(
                MemberBuilder::default()
                    .address("foo")
                    .protocol_port(123)
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
