//! Endpoint prelude
//!
//! This module re-exports all of the types needed for endpoints to implement the
//! [`RestEndpoint`](../trait.RestEndpoint.html) trait.

pub use std::borrow::Cow;

// pub use http::Method;

pub use crate::api::BodyError;
pub use crate::api::Client;
pub use crate::api::JsonBodyParams;
pub use crate::api::Pageable;
pub use crate::api::QueryParams;
pub use crate::api::RestEndpoint;
pub use crate::types::ServiceType;
