#![no_main]

use arbitrary::Arbitrary;
use http::{HeaderMap, HeaderValue, StatusCode};
use libfuzzer_sys::fuzz_target;
use openstack_sdk_auth_core::authtoken::fuzz_parse_error_response;

extern crate openstack_sdk_auth_core;

// `parse_error_response` (extracted from `AuthToken::from_reqwest_response`) is a
// 3-way fallback chain over an untrusted Keystone auth response: on 401 with an
// `openstack-auth-receipt` header it tries an auth-receipt JSON shape, then falls
// back to an identity-error JSON shape, then to a raw-text fallback; any other
// non-success response only tries the identity-error shape before falling back.
// It's the shared choke point every auth plugin (password, jwt, federation, ...)
// funnels through, so this one target covers all of them.

#[derive(Debug, Arbitrary)]
enum FuzzStatus {
    Unauthorized,
    Other(u16),
}

impl FuzzStatus {
    fn as_code(&self) -> StatusCode {
        match self {
            FuzzStatus::Unauthorized => StatusCode::UNAUTHORIZED,
            FuzzStatus::Other(v) => {
                StatusCode::from_u16(*v).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

#[derive(Debug, Arbitrary)]
enum FuzzReceiptHeader {
    Absent,
    /// A valid header value (any ASCII-ish text `to_str()` can handle).
    Ascii(String),
    /// Raw bytes that may fail `to_str()` (non-ASCII), exercising AuthReceiptNotString.
    NonAscii(Vec<u8>),
}

#[derive(Debug, Arbitrary)]
enum FuzzBody {
    /// Fully arbitrary text: covers "parses as neither shape" and the raw fallback.
    Raw(String),
    /// Matches `AuthErrorResponse` (`{"error": {"code": ..., "message": ...}}`).
    ErrorShape { code: u32, message: String },
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    status: FuzzStatus,
    receipt_header: FuzzReceiptHeader,
    body: FuzzBody,
}

fuzz_target!(|input: FuzzInput| {
    let mut headers = HeaderMap::new();
    match &input.receipt_header {
        FuzzReceiptHeader::Absent => {}
        FuzzReceiptHeader::Ascii(s) => {
            if let Ok(hv) = HeaderValue::from_str(s) {
                headers.insert("openstack-auth-receipt", hv);
            }
        }
        FuzzReceiptHeader::NonAscii(bytes) => {
            if let Ok(hv) = HeaderValue::from_bytes(bytes) {
                headers.insert("openstack-auth-receipt", hv);
            }
        }
    }

    let body_text = match &input.body {
        FuzzBody::Raw(s) => s.clone(),
        FuzzBody::ErrorShape { code, message } => {
            serde_json::json!({"error": {"code": code, "message": message}}).to_string()
        }
    };

    let _ = fuzz_parse_error_response(input.status.as_code(), &headers, &body_text);
});
