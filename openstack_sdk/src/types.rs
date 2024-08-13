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

//! Types of the SDK

#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::fmt;

pub mod api_version;
pub mod compute;
pub mod identity;

use futures::io::AsyncRead;
use futures::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};

pub use crate::types::api_version::ApiVersion;

/// Supported Service Types
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ServiceType {
    BlockStorage,
    Compute,
    Image,
    Identity,
    LoadBalancer,
    Network,
    ObjectStore,
    Other(String),
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceType::BlockStorage => write!(f, "block-storage"),
            ServiceType::Compute => write!(f, "compute"),
            ServiceType::Image => write!(f, "image"),
            ServiceType::Identity => write!(f, "identity"),
            ServiceType::LoadBalancer => write!(f, "load-balancer"),
            ServiceType::Network => write!(f, "network"),
            ServiceType::ObjectStore => write!(f, "object-store"),
            ServiceType::Other(x) => write!(f, "{x}"),
        }
    }
}

impl From<&str> for ServiceType {
    fn from(val: &str) -> ServiceType {
        match val {
            "block-storage" => ServiceType::BlockStorage,
            "compute" => ServiceType::Compute,
            "identity" => ServiceType::Identity,
            "image" => ServiceType::Image,
            "load-balancer" => ServiceType::LoadBalancer,
            "network" => ServiceType::Network,
            "object-store" => ServiceType::ObjectStore,
            _ => ServiceType::Other(val.to_string()),
        }
    }
}

/// A wrapper around `AsyncRead` trait allowing returning HTTP response body as something
/// implementing `AsyncRead`.
/// Returning `impl AsyncRead` would be the best option, but since
/// we need to return it from inside a trait function it is
/// currently not possible in Rust to do so and the only way is to
/// `return Box<dyn AsyncRead>`. This is however also challenging,
/// since it is not possible to invoke tokio `compat` function to
/// convert `futures::AsyncRead` into `tokio::io::AsyncRead`. In order
/// to deal with that this wrapper is created and something
/// implementing `AsyncRead` can be passed into it and further used
/// as anything implementing `AsyncRead` on its own.
pub struct BoxedAsyncRead {
    reader: Pin<Box<dyn AsyncRead>>,
}
unsafe impl Send for BoxedAsyncRead {}
unsafe impl Sync for BoxedAsyncRead {}

impl BoxedAsyncRead {
    pub fn new(reader: impl AsyncRead + Send + Sync + 'static) -> Self {
        Self {
            reader: Box::pin(reader),
        }
    }
}

impl AsyncRead for BoxedAsyncRead {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        self.reader.as_mut().poll_read(cx, buf)
    }
}

/// A reference to a resource by its Name and ID.
#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
pub struct IdAndName {
    /// The name of the entity.
    pub name: String,
    /// The UID for the entity.
    pub id: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}
