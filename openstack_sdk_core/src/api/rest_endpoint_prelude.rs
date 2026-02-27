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

//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`RestEndpoint`](../trait.RestEndpoint.html) trait.

pub use std::borrow::Cow;

pub use crate::api::BodyError;
//pub use crate::api::Client;
pub use crate::api::JsonBodyParams;
pub use crate::api::Pageable;
pub use crate::api::QueryParams;
pub use crate::api::RestEndpoint;
pub use crate::types::ApiVersion;
pub use crate::types::ServiceType;
