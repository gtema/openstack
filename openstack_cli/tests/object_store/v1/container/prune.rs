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
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

#[test]
fn prune_help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("object-store")
        .arg("container")
        .arg("prune")
        .arg("--help");
    cmd.assert().success();

    Ok(())
}

#[tokio::test]
async fn container_prune_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    skip_without_extension!("network", "security-groups-default-rules");

    let container_name = "sdk-dummy-container";
    let object_name = "dummy-object";
    let mut file = NamedTempFile::new()?;
    file.write_all(b"SDK dummy content")?;

    // Create container
    Command::cargo_bin("osc")?
        .arg("object-store")
        .arg("container")
        .arg("create")
        .arg(container_name)
        .assert()
        .success();

    // Upload object
    Command::cargo_bin("osc")?
        .arg("object-store")
        .arg("object")
        .arg("upload")
        .arg(container_name)
        .arg(object_name)
        .arg("--file")
        .arg(file.path())
        .assert()
        .success();

    // Prune container
    Command::cargo_bin("osc")?
        .arg("object-store")
        .arg("container")
        .arg("prune")
        .arg(container_name)
        .arg("--prefix")
        .arg(object_name)
        .assert()
        .success();

    // Delete container
    Command::cargo_bin("osc")?
        .arg("object-store")
        .arg("container")
        .arg("delete")
        .arg(container_name)
        .assert()
        .success();

    Ok(())
}
