#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use openstack_sdk_plugin_wasm::fuzz_identity_http_request_parsing;
use url::Url;

extern crate openstack_sdk_plugin_wasm;

// `identity_http_request` is the one host function exposed to every WASM auth
// plugin -- the sole point where guest-controlled bytes cross the Extism
// boundary into host code. Its request-parsing/validation step (JSON decode,
// `path` must be relative, URL join against the bound identity origin, method
// parse) runs on every call before any network I/O, so it must never panic on
// adversarial input, however the plugin was built.

#[derive(Debug, Arbitrary)]
enum FuzzRequest {
    /// Fully arbitrary text: covers "doesn't even parse as the request shape".
    Raw(String),
    /// A well-formed `HttpRequestMsg` JSON shape with fuzzed field values,
    /// reaching the relative-path check, URL join, and method parse.
    Structured {
        method: String,
        path: String,
        headers: Vec<(String, String)>,
        body: Option<String>,
    },
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    origin_host: String,
    request: FuzzRequest,
}

fuzz_target!(|input: FuzzInput| {
    let Ok(origin) = Url::parse(&format!("https://{}", input.origin_host)) else {
        return;
    };

    let request_json = match &input.request {
        FuzzRequest::Raw(s) => s.clone(),
        FuzzRequest::Structured {
            method,
            path,
            headers,
            body,
        } => serde_json::json!({
            "method": method,
            "path": path,
            "headers": headers.iter().cloned().collect::<std::collections::BTreeMap<_, _>>(),
            "body": body,
        })
        .to_string(),
    };

    fuzz_identity_http_request_parsing(&origin, &request_json);
});
