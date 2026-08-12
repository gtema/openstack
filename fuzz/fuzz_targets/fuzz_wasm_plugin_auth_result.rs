#![no_main]

use libfuzzer_sys::fuzz_target;
use openstack_sdk_plugin_wasm::plugin::fuzz_parse_auth_result;

extern crate openstack_sdk_plugin_wasm;

// Every guest export that finishes an auth attempt (`auth`, and
// `sso_parse_callback`) returns its result as an `AuthResultMsg` JSON string
// that the host deserializes. This is the last guest-controlled parse before
// the host either mints an `Auth::AuthToken` or surfaces an error message to
// the caller, so it must never panic regardless of what the guest returns --
// a compromised or simply buggy plugin is the threat model, not a
// well-behaved one.

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        fuzz_parse_auth_result(s);
    }
});
