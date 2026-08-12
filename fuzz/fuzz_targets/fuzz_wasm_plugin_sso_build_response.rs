#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use openstack_sdk_plugin_wasm::plugin::fuzz_validate_sso_build_response;

extern crate openstack_sdk_plugin_wasm;

// `sso_build_request`'s response is the guest's one chance to steer where a
// real browser gets opened, so the host validates it before ever prompting
// the user: the URL must parse and be `https`, and the declared
// `redirect_host` must exactly match the host-bound callback listener's own
// authority. A malicious or buggy plugin fully controls both strings; this
// target checks the validation logic never panics on adversarial input, on
// either axis (the response shape and the expected-host comparison).

#[derive(Debug, Arbitrary)]
enum FuzzBuildOutput {
    /// Fully arbitrary text: covers "doesn't even parse as the response shape".
    Raw(String),
    /// A well-formed `{"url", "redirect_host"}` shape with fuzzed field
    /// values, reaching the URL-parse/scheme/host-match checks.
    Structured { url: String, redirect_host: String },
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    build_output: FuzzBuildOutput,
    expected_redirect_host: String,
}

fuzz_target!(|input: FuzzInput| {
    let build_output = match &input.build_output {
        FuzzBuildOutput::Raw(s) => s.clone(),
        FuzzBuildOutput::Structured { url, redirect_host } => {
            serde_json::json!({"url": url, "redirect_host": redirect_host}).to_string()
        }
    };

    fuzz_validate_sso_build_response(&build_output, &input.expected_redirect_host);
});
