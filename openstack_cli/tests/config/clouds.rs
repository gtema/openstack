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

use assert_cmd::Command;

/// Source config the command inherits connection settings from.
const SOURCE_CLOUDS: &str = r#"clouds:
  src:
    auth:
      auth_url: https://keystone:5000/v3
    region_name: RegionOne
"#;

const CREATE_RESPONSE: &str = r#"{"id": "cid", "secret": "sec", "name": "deploy"}"#;

fn add_cmd(source: &std::path::Path) -> Command {
    let mut cmd = Command::cargo_bin("osc").expect("osc binary");
    cmd.arg("--os-cloud")
        .arg("src")
        .arg("--os-client-config-file")
        .arg(source)
        .arg("config")
        .arg("clouds")
        .arg("add");
    cmd
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.arg("config").arg("clouds").arg("add").arg("--help");
    cmd.assert().success();

    Ok(())
}

#[test]
fn add_writes_new_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("src-clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("out/clouds.yaml");

    add_cmd(&source)
        .arg("--cloud-name")
        .arg("prod")
        .arg("--file")
        .arg(&target)
        .write_stdin(CREATE_RESPONSE)
        .assert()
        .success();

    let doc: serde_yaml::Value = serde_yaml::from_str(&std::fs::read_to_string(&target)?)?;
    let entry = &doc["clouds"]["prod"];
    assert_eq!(
        entry["auth"]["application_credential_id"].as_str(),
        Some("cid")
    );
    assert_eq!(
        entry["auth"]["application_credential_secret"].as_str(),
        Some("sec")
    );
    assert_eq!(
        entry["auth"]["auth_url"].as_str(),
        Some("https://keystone:5000/v3")
    );
    assert_eq!(entry["auth_type"].as_str(), Some("v3applicationcredential"));
    assert_eq!(entry["region_name"].as_str(), Some("RegionOne"));

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&target)?.permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "file with secret must be 0600");
    }

    Ok(())
}

#[test]
fn add_split_writes_secure_sibling() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("src-clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("out/clouds.yaml");

    add_cmd(&source)
        .arg("--split")
        .arg("--file")
        .arg(&target)
        .write_stdin(CREATE_RESPONSE)
        .assert()
        .success();

    let clouds = std::fs::read_to_string(&target)?;
    assert!(!clouds.contains("sec"), "secret must not be in clouds.yaml");
    let secure = std::fs::read_to_string(dir.path().join("out/secure.yaml"))?;
    assert!(secure.contains("application_credential_secret: sec"));

    Ok(())
}

#[test]
fn add_same_name_needs_overwrite() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("src-clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("clouds.yaml");
    std::fs::write(&target, "clouds:\n  openstack:\n    auth: {}\n")?;

    let output = add_cmd(&source)
        .arg("--file")
        .arg(&target)
        .write_stdin(CREATE_RESPONSE)
        .output()?;
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("--overwrite"),
        "collision error must point at --overwrite"
    );

    add_cmd(&source)
        .arg("--file")
        .arg(&target)
        .arg("--overwrite")
        .write_stdin(CREATE_RESPONSE)
        .assert()
        .success();

    Ok(())
}
