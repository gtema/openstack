//! Types of the SDK
use serde::{Deserialize, Serialize};

pub mod compute;
pub mod identity;

use futures::io::AsyncRead;
use futures::io::Error;
use futures::stream::IntoAsyncRead;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A wrapper around AsyncRead trait allowing returning HTTP response body as something implementing AsyncRead.
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
#[derive(Deserialize, Debug, Clone, Serialize)]
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
