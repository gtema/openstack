#![no_main]

use libfuzzer_sys::fuzz_target;
extern crate openstack_sdk;

// `&str` targets are driven by `arbitrary`'s built-in impl: libfuzzer-sys
// interprets the raw fuzz bytes as (possibly truncated) UTF-8 before handing
// them to the closure, so malformed byte sequences are filtered out for us.
fuzz_target!(|s: &str| {
    let _ = openstack_sdk::api::fuzz_parse_link_header(s);
});
