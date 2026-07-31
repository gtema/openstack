#![no_main]

use std::fs::File;
use std::io::Write;

use libfuzzer_sys::fuzz_target;
use openstack_sdk_core::state::State;

extern crate openstack_sdk_core;

// The on-disk auth/discovery cache files (`~/.osc/<auth_hash>[.discovery]`) are
// local, not network-controlled, but a corrupted file (disk corruption, partial
// write, cross-version cache reuse) is a plausible real-world scenario. The
// reader logic (split off a format-version byte, then `postcard::from_bytes`) is
// meant to degrade gracefully -- log and delete the file -- rather than panic on
// malformed content. This target checks that guarantee holds for arbitrary bytes.

fuzz_target!(|data: &[u8]| {
    let state = State::new();

    if let Ok(mut file) = tempfile::NamedTempFile::new()
        && file.write_all(data).is_ok()
    {
        let path = file.path().to_path_buf();
        if let Ok(mut read_file) = File::open(&path) {
            state.fuzz_read_auth_state_from_file(&mut read_file, &path);
        }
    }

    if let Ok(mut file) = tempfile::NamedTempFile::new()
        && file.write_all(data).is_ok()
    {
        let path = file.path().to_path_buf();
        if let Ok(mut read_file) = File::open(&path) {
            state.fuzz_read_discovery_state_from_file(&mut read_file, &path);
        }
    }
});
