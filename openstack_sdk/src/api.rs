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
mod paged;
mod params;
pub(crate) mod query;
mod rest_endpoint;

#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod block_storage;
#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod compute;
#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod identity;
#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod image;
#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod network;
#[allow(dead_code, unused_imports, unused_variables, unused_mut)]
pub mod object_store;

pub use self::error::ApiError;
pub use self::error::BodyError;

pub use self::client::AsyncClient;
pub use self::client::Client;
pub use self::client::RestClient;

pub use self::rest_endpoint::RestEndpoint;

pub use self::query::Query;
pub use self::query::QueryAsync;
pub use self::query::RawQuery;
pub use self::query::RawQueryAsync;

pub mod rest_endpoint_prelude;

pub use self::paged::paged;
pub use self::paged::Pageable;
pub use self::paged::Paged;
pub use self::paged::Pagination;
pub use self::paged::PaginationError;

pub use self::find::find;
// pub use self::find::Findable;

pub use self::params::JsonBodyParams;
pub use self::params::ParamValue;
pub use self::params::QueryParams;

pub use self::ignore::ignore;
pub use self::ignore::Ignore;
