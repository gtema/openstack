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
//!    use openstack_sdk::api::QueryAsync;
//!    # use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # let ep = openstack_sdk::api::compute::v2::flavor::get::Request::builder().build().unwrap();
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
//!    use openstack_sdk::api::RawQueryAsync;
//!    # use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use http::{Response};
//!    # use bytes::Bytes;
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # let ep = openstack_sdk::api::compute::v2::flavor::get::Request::builder().build().unwrap();
//!    let rsp: Response<Bytes> = ep.raw_query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! ## Find combinator
//!
//! Finding resource by `name` or `id` is possible using [`find`](fn@find) combinator.
//!
//! ```
//!    use openstack_sdk::api::QueryAsync;
//!    use openstack_sdk::api::find;
//!    # use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # use http::Response;
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # let ep = openstack_sdk::api::compute::v2::flavor::find::Request::builder().build().unwrap();
//!    let data_raw: serde_json::Value = find(ep).query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
//! ## Pagination combinator
//!
//! Support for querying paginated resources is covered using [`paged`](fn@paged) combinator.
//!
//! ```
//!    use openstack_sdk::api::{QueryAsync, Pagination};
//!    use openstack_sdk::api::paged;
//!    # use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # let ep = openstack_sdk::api::compute::v2::flavor::list::Request::builder().build().unwrap();
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
//!    use openstack_sdk::api::{ignore, QueryAsync};
//!    # use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//!    # async fn func() -> Result<(), OpenStackError> {
//!    # let cfg = ConfigFile::new().unwrap();
//!    # let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!    # let client = AsyncOpenStack::new(&profile).await?;
//!    # let ep = openstack_sdk::api::compute::v2::flavor::delete::Request::builder().build().unwrap();
//!    ignore(ep).query_async(&client).await?;
//!    # Ok(())
//!    # }
//! ```
//!
#![allow(clippy::module_inception)]

mod client;
mod common;
mod error;
mod find;
mod ignore;
#[allow(dead_code)]
mod paged;
mod params;
pub(crate) mod query;
mod rest_endpoint;

#[cfg(feature = "block_storage")]
pub mod block_storage;
#[cfg(feature = "compute")]
pub mod compute;
#[allow(dead_code)]
#[cfg(feature = "identity")]
pub mod identity;
#[allow(dead_code)]
#[cfg(feature = "image")]
pub mod image;
#[cfg(feature = "load_balancer")]
pub mod load_balancer;
#[cfg(feature = "network")]
pub mod network;
#[cfg(feature = "object_store")]
pub mod object_store;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::client::RestClient;

pub use self::rest_endpoint::check_response_error;
pub use self::rest_endpoint::RestEndpoint;

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

pub use self::paged::paged;
pub use self::paged::Pageable;
pub use self::paged::Paged;
pub use self::paged::Pagination;
pub use self::paged::PaginationError;

pub use self::find::{find, find_by_name};

pub use self::params::JsonBodyParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;
