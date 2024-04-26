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
#[allow(clippy::module_inception)]
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

pub use self::find::find;

pub use self::params::JsonBodyParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;
