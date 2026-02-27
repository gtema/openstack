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

//! OpenStack API bindings
//!
//! This module provides implementation for the individual APIs as well as the necessary logic
//!
//! ## Query/QueryAsync trait
//!
//! API requests that return data should be invoked using [Query] or [QueryAsync] style.
//!
//! ```
//!    use openstack_sdk_core::api::QueryAsync;
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use std::borrow::Cow;
//!    # use openstack_sdk_core::{api::RestEndpoint, types::ServiceType};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    #
//!    # #[derive(derive_builder::Builder)]
//!    # pub struct Request<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for Request<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         format!("flavors/{id}", id = self.id.as_ref(),).into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # let ep = RequestBuilder::default().build().unwrap();
//!    let data_raw: serde_json::Value = ep.query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//! ## RawQuery/RawQueryAsync trait
//!
//! It may be sometimes desired to get the raw API response for example to access headers. It is
//! possible using [RawQuery]/[RawQueryAsync] trait on such endpoints.
//!
//! ```
//!    use openstack_sdk_core::api::RawQueryAsync;
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use std::borrow::Cow;
//!    # use openstack_sdk_core::{api::RestEndpoint, types::ServiceType};
//!    # use http::{Response};
//!    # use bytes::Bytes;
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # #[derive(derive_builder::Builder)]
//!    # pub struct Request<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for Request<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         format!("flavors/{id}", id = self.id.as_ref(),).into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # let ep = RequestBuilder::default().build().unwrap();
//!    let rsp: Response<Bytes> = ep.raw_query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! ## Find combinator
//!
//! Finding resource by `name` or `id` is possible using [`find`](fn@find) combinator. First a API
//! request to get resource directly by the identified (i.e. `flavors/<VALUE>`) is done. When it
//! returns positive data it is used as a find response. Otherwise list API call is invoked
//! (passing name filter parameter when available). Single operation return entry is used as find
//! result otherwise an error is returned. Only endpoints implementing
//! [`Findable`] trait support that.
//!
//! ```
//!    use openstack_sdk_core::api::QueryAsync;
//!    use openstack_sdk_core::api::find;
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use http::Response;
//!    # use std::borrow::Cow;
//!    # use derive_builder::Builder;
//!    # use openstack_sdk_core::{api::{ApiError, Pageable, RestClient, RestEndpoint}, types::ServiceType};
//!    # use openstack_sdk_core::api::Findable;
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # #[derive(Builder)]
//!    # pub struct GetRequest<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for GetRequest<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         format!("flavors/{id}", id = self.id.as_ref(),).into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # #[derive(Builder)]
//!    # pub struct ListRequest<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for ListRequest<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         "flavors".to_string().into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # impl Pageable for ListRequest<'_> {}
//!    #
//!    # #[derive(Debug, Builder, Clone)]
//!    # pub struct FindRequest<'a> {
//!    #     #[builder(setter(into), default)]
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl<'a> Findable for FindRequest<'a> {
//!    #     type G = GetRequest<'a>;
//!    #     type L = ListRequest<'a>;
//!    #     fn get_ep<C: RestClient>(&self) -> Result<Self::G, ApiError<C::Error>> {
//!    #         let mut ep = GetRequestBuilder::default();
//!    #         ep.id(self.id.clone());
//!    #         ep.build().map_err(ApiError::endpoint_builder)
//!    #     }
//!    #
//!    #     fn list_ep<C: RestClient>(&self) -> Result<Self::L, ApiError<C::Error>> {
//!    #         let mut ep = ListRequestBuilder::default();
//!    #         ep.build().map_err(ApiError::endpoint_builder)
//!    #     }
//!    #     /// Locate flavor in a list
//!    #     fn locate_resource_in_list<C: RestClient>(
//!    #         &self,
//!    #         data: Vec<serde_json::Value>,
//!    #     ) -> Result<serde_json::Value, ApiError<C::Error>> {
//!    #         // flavor is not supporting name as query parameter to the list.
//!    #         // Therefore it is necessary to go through complete list of results.
//!    #         let mut maybe_result: Option<serde_json::Value> = None;
//!    #         for item in data.iter() {
//!    #             if let Some(name_as_val) = item.get("name") {
//!    #                 if let Some(name) = name_as_val.as_str() {
//!    #                     if name == self.id {
//!    #                         if maybe_result.is_none() {
//!    #                             maybe_result = Some(item.clone());
//!    #                         } else {
//!    #                             return Err(ApiError::IdNotUnique);
//!    #                         }
//!    #                     }
//!    #                 }
//!    #             }
//!    #         }
//!    #         maybe_result.ok_or(ApiError::ResourceNotFound)
//!    #     }
//!    # }
//!    # let ep = FindRequestBuilder::default().build().unwrap();
//!    let data_raw: serde_json::Value = find(ep).query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! When identifier is clearly known to be `name` [`find`](fn@find_by_name) is more useful and is
//! saving unnecessary API roundtrip for attempting to query resource by the identifier and
//! immediately triggers listing operation.
//!
//! ```
//!    use openstack_sdk_core::api::QueryAsync;
//!    use openstack_sdk_core::api::find_by_name;
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use http::Response;
//!    # use std::borrow::Cow;
//!    # use derive_builder::Builder;
//!    # use openstack_sdk_core::{api::{ApiError, Pageable, RestClient, RestEndpoint}, types::ServiceType};
//!    # use openstack_sdk_core::api::Findable;
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # #[derive(Builder)]
//!    # pub struct GetRequest<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for GetRequest<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         format!("flavors/{id}", id = self.id.as_ref(),).into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # #[derive(Builder)]
//!    # pub struct ListRequest<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for ListRequest<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         "flavors".to_string().into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # impl Pageable for ListRequest<'_> {}
//!    #
//!    # #[derive(Debug, Builder, Clone)]
//!    # pub struct FindRequest<'a> {
//!    #     #[builder(setter(into), default)]
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl<'a> Findable for FindRequest<'a> {
//!    #     type G = GetRequest<'a>;
//!    #     type L = ListRequest<'a>;
//!    #     fn get_ep<C: RestClient>(&self) -> Result<Self::G, ApiError<C::Error>> {
//!    #         let mut ep = GetRequestBuilder::default();
//!    #         ep.id(self.id.clone());
//!    #         ep.build().map_err(ApiError::endpoint_builder)
//!    #     }
//!    #
//!    #     fn list_ep<C: RestClient>(&self) -> Result<Self::L, ApiError<C::Error>> {
//!    #         let mut ep = ListRequestBuilder::default();
//!    #         ep.build().map_err(ApiError::endpoint_builder)
//!    #     }
//!    #     /// Locate flavor in a list
//!    #     fn locate_resource_in_list<C: RestClient>(
//!    #         &self,
//!    #         data: Vec<serde_json::Value>,
//!    #     ) -> Result<serde_json::Value, ApiError<C::Error>> {
//!    #         // flavor is not supporting name as query parameter to the list.
//!    #         // Therefore it is necessary to go through complete list of results.
//!    #         let mut maybe_result: Option<serde_json::Value> = None;
//!    #         for item in data.iter() {
//!    #             if let Some(name_as_val) = item.get("name") {
//!    #                 if let Some(name) = name_as_val.as_str() {
//!    #                     if name == self.id {
//!    #                         if maybe_result.is_none() {
//!    #                             maybe_result = Some(item.clone());
//!    #                         } else {
//!    #                             return Err(ApiError::IdNotUnique);
//!    #                         }
//!    #                     }
//!    #                 }
//!    #             }
//!    #         }
//!    #         maybe_result.ok_or(ApiError::ResourceNotFound)
//!    #     }
//!    # }
//!    # let ep = FindRequestBuilder::default().build().unwrap();
//!    let data_raw: serde_json::Value = find_by_name(ep).query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! ## Pagination combinator
//!
//! Support for querying paginated resources is covered using [`paged`](fn@paged) combinator. The
//! endpoint must implement [`Pageable`] trait to support this combinator.
//!
//! ```
//!    use openstack_sdk_core::api::{QueryAsync, Pagination};
//!    use openstack_sdk_core::api::paged;
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use std::borrow::Cow;
//!    # use openstack_sdk_core::{api::{Pageable, RestEndpoint}, types::ServiceType};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # #[derive(derive_builder::Builder)]
//!    # pub struct Request<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for Request<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         "flavors".to_string().into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # impl Pageable for Request<'_> {}
//!    # let ep = RequestBuilder::default().build().unwrap();
//!    let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(100))
//!        .query_async(&client)
//!        .await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! ## Ignoring response combinator
//!
//! Some APIs natively do not return any response. Trying to use [Query]/[QueryAsync] trait on them
//! result in an error while casing the data. When API do not return any response or it is
//! explicitly uninteresting a [`ignore`](fn@ignore) combinator can be used. When API returned an
//! error it is properly thrown.
//!
//! ```
//!    use openstack_sdk_core::api::{ignore, QueryAsync};
//!    # use openstack_sdk_core::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use std::borrow::Cow;
//!    # use openstack_sdk_core::{api::RestEndpoint, types::ServiceType};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack").unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # #[derive(derive_builder::Builder)]
//!    # pub struct Request<'a> {
//!    #     id: Cow<'a, str>,
//!    # }
//!    #
//!    # impl RestEndpoint for Request<'_> {
//!    #     fn method(&self) -> http::Method {
//!    #         http::Method::GET
//!    #     }
//!    #
//!    #     fn endpoint(&self) -> Cow<'static, str> {
//!    #         format!("flavors/{id}", id = self.id.as_ref(),).into()
//!    #     }
//!    #
//!    #     fn service_type(&self) -> ServiceType {
//!    #         ServiceType::Compute
//!    #     }
//!    #
//!    #     fn response_key(&self) -> Option<Cow<'static, str>> {
//!    #         Some("flavor".into())
//!    #     }
//!    # }
//!    # let ep = RequestBuilder::default().build().unwrap();
//!    ignore(ep).query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
#![allow(clippy::module_inception)]

mod client;
pub mod common;
mod error;
mod find;
mod ignore;
#[allow(dead_code)]
mod paged;
mod params;
pub mod query;
pub mod rest_endpoint;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::client::RestClient;

pub use self::rest_endpoint::RestEndpoint;
pub use self::rest_endpoint::check_response_error;

#[cfg(feature = "async")]
pub use self::client::AsyncClient;
#[cfg(feature = "sync")]
pub use self::client::Client;
#[cfg(feature = "sync")]
pub use self::query::Query;
#[cfg(feature = "async")]
pub use self::query::QueryAsync;
#[cfg(feature = "sync")]
pub use self::query::RawQuery;
#[cfg(feature = "async")]
pub use self::query::RawQueryAsync;

pub mod rest_endpoint_prelude;

pub use self::paged::Pageable;
pub use self::paged::Paged;
pub use self::paged::Pagination;
pub use self::paged::PaginationError;
pub use self::paged::paged;

pub use self::find::{Findable, find, find_by_name};

pub use self::params::JsonBodyParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::ignore::Ignore;
pub use self::ignore::ignore;
