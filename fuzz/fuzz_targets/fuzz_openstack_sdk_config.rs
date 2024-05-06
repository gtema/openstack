#![no_main]

use libfuzzer_sys::fuzz_target;
extern crate openstack_sdk;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = openstack_sdk::config::ConfigFile::new()
            .unwrap()
            .get_cloud_config(s.to_string());
    }
});
