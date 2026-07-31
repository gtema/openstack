#![no_main]

use arbitrary::Arbitrary;
use bytes::Bytes;
use http::{HeaderValue, StatusCode};
use libfuzzer_sys::fuzz_target;
use openstack_sdk_core::api::ApiError;
use serde_json::{Map, Value};

extern crate openstack_sdk_core;

// `ApiError::from_openstack` extracts a human message from an arbitrary/untrusted
// OpenStack error JSON body, papering over how inconsistently services shape it:
// `{"message": ...}`, `{"error": ...}` (Keystone/Nova-style), `{"faultstring": ...}`
// (Octavia-style), a non-string value at any of those keys, or none of them present
// at all. We model the presence/shape of each of those three keys directly rather
// than fuzzing raw JSON text, since the function receives an already-parsed
// `serde_json::Value` (parsing the raw body is a different fuzz target's job).

#[derive(Debug, Arbitrary)]
enum FuzzLeaf {
    /// A string value -> recognized message.
    Str(String),
    /// A nested object value -> "we don't know how to parse this" path.
    Nested(String),
}

impl FuzzLeaf {
    fn to_value(&self) -> Value {
        match self {
            FuzzLeaf::Str(s) => Value::String(s.clone()),
            FuzzLeaf::Nested(s) => {
                let mut obj = Map::new();
                obj.insert("nested".into(), Value::String(s.clone()));
                Value::Object(obj)
            }
        }
    }
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    status: u16,
    message: Option<FuzzLeaf>,
    error: Option<FuzzLeaf>,
    faultstring: Option<FuzzLeaf>,
    req_id: Option<String>,
}

fuzz_target!(|input: FuzzInput| {
    let mut obj = Map::new();
    if let Some(v) = &input.message {
        obj.insert("message".into(), v.to_value());
    }
    if let Some(v) = &input.error {
        obj.insert("error".into(), v.to_value());
    }
    if let Some(v) = &input.faultstring {
        obj.insert("faultstring".into(), v.to_value());
    }
    let value = Value::Object(obj);

    let status = StatusCode::from_u16(input.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut builder = http::Response::builder().status(status);
    if let Some(id) = &input.req_id
        && let Ok(hv) = HeaderValue::from_str(id)
    {
        builder = builder.header("x-openstack-request-id", hv);
    }
    let Ok(rsp) = builder.body(Bytes::new()) else {
        return;
    };

    let _ = ApiError::<std::convert::Infallible>::fuzz_from_openstack(None, &rsp, value);
});
