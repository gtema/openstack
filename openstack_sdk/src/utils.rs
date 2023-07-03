use futures::io::{AsyncRead, Error as IoError, ErrorKind as IoErrorKind};
use futures::stream::TryStreamExt;
use reqwest::{Body, Response};
use tokio_util::codec;
use tokio_util::compat::FuturesAsyncReadCompatExt;

/// Convert an object implementing AsyncRead to a reqwest Body.
#[inline]
pub fn async_read_to_body(read: impl AsyncRead + Send + Sync + 'static) -> Body {
    let stream =
        codec::FramedRead::new(read.compat(), codec::BytesCodec::new()).map_ok(|b| b.freeze());
    Body::wrap_stream(stream)
}

/// Convert a response to an object implementing AsyncRead.
#[inline]
pub fn body_to_async_read(resp: Response) -> impl AsyncRead + Send + Sync + 'static {
    resp.bytes_stream()
        .map_err(|orig| {
            let kind = if orig.is_timeout() {
                IoErrorKind::TimedOut
            } else {
                IoErrorKind::Other
            };
            IoError::new(kind, orig)
        })
        .into_async_read()
}
