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
    Dns,
    Image,
    Identity,
    LoadBalancer,
    Network,
    ObjectStore,
    Placement,
    Other(String),
}

impl fmt::Display for ServiceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceType::BlockStorage => write!(f, "block-storage"),
            ServiceType::Compute => write!(f, "compute"),
            ServiceType::Dns => write!(f, "dns"),
            ServiceType::Image => write!(f, "image"),
            ServiceType::Identity => write!(f, "identity"),
            ServiceType::LoadBalancer => write!(f, "load-balancer"),
            ServiceType::Network => write!(f, "network"),
            ServiceType::ObjectStore => write!(f, "object-store"),
            ServiceType::Placement => write!(f, "placement"),
            ServiceType::Other(x) => write!(f, "{x}"),
        }
    }
}

impl From<&str> for ServiceType {
    fn from(val: &str) -> ServiceType {
        match val {
            "block-storage" => ServiceType::BlockStorage,
            "compute" => ServiceType::Compute,
            "dns" => ServiceType::Dns,
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

impl fmt::Debug for BoxedAsyncRead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "_data_")
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

/// Status of the resource
#[derive(Debug, Default, PartialEq)]
pub enum EntryStatus {
    /// Normal status
    #[default]
    Normal,
    /// Any Error
    Error,
    /// Some Action is in progress
    Pending,
    /// Inactive
    Inactive,
}

impl From<Option<&String>> for EntryStatus {
    fn from(input: Option<&String>) -> EntryStatus {
        match input {
            Some(val) => match val.to_lowercase().as_str() {
                // Statuses treated as an error
                "degraded" | "error" | "error_deleting" | "error_backing-up"
                | "error_restoring" | "error_extending" | "killed" => Self::Error,
                // Statuses treated as an currently in progress
                "attaching" | "backing-up" | "build" | "building" | "creating" | "detaching"
                | "downloading" | "extending" | "importing" | "pending" | "queued"
                | "restoring" | "restoring-backup" | "saving" | "uploading" => Self::Pending,
                // inactive
                "available" | "deleted" | "no_monitor" | "offline" | "reserved" | "shutoff" => {
                    Self::Inactive
                }
                _ => Self::Normal,
            },
            None => Self::Normal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_status() {
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("DEGRADED")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("error")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("error_backing-up")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("error_deleting")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("error_restoring")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("error_extending")))
        );
        assert_eq!(
            EntryStatus::Error,
            EntryStatus::from(Some(&String::from("killed")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("attaching")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("backing-up")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("build")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("building")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("creating")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("detaching")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("downloading")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("extending")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("importing")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("pending")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("queued")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("restoring")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("restoring-backup")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("saving")))
        );
        assert_eq!(
            EntryStatus::Pending,
            EntryStatus::from(Some(&String::from("uploading")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("available")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("deleted")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("no_monitor")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("offline")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("reserved")))
        );
        assert_eq!(
            EntryStatus::Inactive,
            EntryStatus::from(Some(&String::from("shutoff")))
        );
        assert_eq!(
            EntryStatus::Normal,
            EntryStatus::from(Some(&String::from("foobar")))
        );
        assert_eq!(EntryStatus::Normal, EntryStatus::from(None));
    }
}
