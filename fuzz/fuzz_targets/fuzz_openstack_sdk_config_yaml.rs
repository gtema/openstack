#![no_main]

use std::io::Write;

use libfuzzer_sys::fuzz_target;
extern crate openstack_sdk;

mod common;

// Unlike `fuzz_openstack_sdk_config` (which only fuzzes the cloud-name
// argument), this target feeds the fuzzed bytes as the *content* of a
// `clouds.yaml`-style file, exercising the actual YAML parsing and
// `ConfigFile` deserialization path.
fuzz_target!(|data: &[u8]| {
    // Work around https://github.com/rust-cli/config-rs/issues/232
    if std::str::from_utf8(data).is_ok_and(common::has_huge_index) {
        return;
    }
    let Ok(mut file) = tempfile::Builder::new().suffix(".yaml").tempfile() else {
        return;
    };
    if file.write_all(data).is_err() {
        return;
    }
    let _ = openstack_sdk::config::ConfigFile::builder().add_source(file.path());
});
