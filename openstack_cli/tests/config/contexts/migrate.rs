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

/// The cloud `migrate` translates; only it is requested, so clouds from
/// the ambient environment (i.e. devstack's in CI) stay out of the result.
const SOURCE_CLOUDS: &str = r#"clouds:
  src:
    auth:
      auth_url: https://keystone:5000/v3
      username: admin
      password: secret
      project_name: demo
    region_name: RegionOne
"#;

fn migrate_cmd(
    home: &std::path::Path,
    source: &std::path::Path,
    target: &std::path::Path,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;
    // Cut the command off from the ambient environment: env vars and the
    // home directory are cleared, and only `src` is requested, so clouds a
    // system-wide file (i.e. devstack's in CI) defines stay out.
    cmd.env_clear()
        .env("HOME", home)
        .arg("--os-client-config-file")
        .arg(source)
        .arg("config")
        .arg("contexts")
        .arg("migrate")
        .arg("src")
        .arg("--file")
        .arg(target);
    Ok(cmd)
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("osc")?;

    cmd.args(["config", "contexts", "migrate", "--help"]);
    cmd.assert().success();

    Ok(())
}

#[test]
fn migrate() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("contexts.yaml");

    migrate_cmd(dir.path(), &source, &target)?
        .assert()
        .success();

    let doc: yaml_serde::Value = yaml_serde::from_str(&std::fs::read_to_string(&target)?)?;
    assert_eq!(doc["servers"][0]["name"].as_str(), Some("src"));
    assert_eq!(
        doc["servers"][0]["server"]["auth_url"].as_str(),
        Some("https://keystone:5000/v3")
    );
    assert!(
        doc["servers"][0]["server"].get("interface").is_none(),
        "the default `interface: public` must not be written"
    );
    assert_eq!(doc["users"][0]["user"]["user_name"].as_str(), Some("admin"));
    assert_eq!(doc["users"][0]["user"]["password"].as_str(), Some("secret"));
    assert_eq!(
        doc["scopes"][0]["scope"]["project"]["name"].as_str(),
        Some("demo")
    );
    let ctx = &doc["contexts"][0]["context"];
    assert_eq!(ctx["server"].as_str(), Some("src"));
    assert_eq!(ctx["user"].as_str(), Some("src"));
    assert_eq!(ctx["scope"].as_str(), Some("src"));
    assert_eq!(ctx["region_name"].as_str(), Some("RegionOne"));
    assert!(doc.get("current-context").is_none());

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(&target)?.permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "the file holds a password");
    }

    Ok(())
}

/// A second run must not duplicate entries or touch the file.
#[test]
fn migrate_is_idempotent() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("contexts.yaml");

    migrate_cmd(dir.path(), &source, &target)?
        .assert()
        .success();
    let first = std::fs::read_to_string(&target)?;

    let output = migrate_cmd(dir.path(), &source, &target)?.output()?;
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("Skipped `src`: the name is already used"),
        "the already migrated cloud must be skipped:\n{stderr}"
    );

    assert_eq!(
        std::fs::read_to_string(&target)?,
        first,
        "a second run must leave the file untouched"
    );

    Ok(())
}

/// `--dry-run` must report what would be added without writing anything.
#[test]
fn dry_run_does_not_write() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let source = dir.path().join("clouds.yaml");
    std::fs::write(&source, SOURCE_CLOUDS)?;
    let target = dir.path().join("contexts.yaml");

    let output = migrate_cmd(dir.path(), &source, &target)?
        .arg("--dry-run")
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{stderr}");
    for line in ["servers: src", "users: src", "scopes: src", "contexts: src"] {
        assert!(stdout.contains(line), "{line:?} missing:\n{stdout}");
    }
    assert!(
        !stdout.contains("secret"),
        "credentials must not be shown:\n{stdout}"
    );
    assert!(stderr.contains("is not changed"), "{stderr}");
    assert!(!target.exists(), "dry run must not create the file");

    Ok(())
}
