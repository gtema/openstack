// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use assert_cmd::prelude::*;
use rand::distr::{Alphanumeric, SampleString};
use serde_json::Value;
use std::process::Command;

const PUBLIC_KEY: &str = "ecdsa-sha2-nistp256 AAAAE2VjZHNhLXNoYTItbmlzdHAyNTYAAAAIbmlzdHAyNTYAAABBBMNR0VlJUeGZI/ax5NOoMM8amfT4zdUv4LysQSAC8D0G3A6Bs9hUZ120+2LLTvAS2GQAU3EtaJQNgJi7G1kYHA8=";

#[test]
fn roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let kp_name = format!(
        "test-rust-{}",
        Alphanumeric.sample_string(&mut rand::rng(), 16)
    );

    let output = Command::cargo_bin("osc")?
        .args([
            "compute",
            "keypair",
            "create292",
            "--name",
            &kp_name,
            "--public-key",
            PUBLIC_KEY,
            "-o",
            "json",
        ])
        .output()
        .expect("failed to create keypair");
    let data: Value = serde_json::from_slice(&output.stdout)?;
    let public_key = data["public_key"].as_str().expect("ID is present");

    assert_eq!(public_key, PUBLIC_KEY);

    let output = Command::cargo_bin("osc")?
        .args(["compute", "keypair", "show", &kp_name, "-o", "json"])
        .output()
        .expect("failed to show keypair");
    let _data: Value = serde_json::from_slice(&output.stdout)?;

    Command::cargo_bin("osc")?
        .args(["compute", "keypair", "delete", &kp_name])
        .ok()?;

    Ok(())
}
