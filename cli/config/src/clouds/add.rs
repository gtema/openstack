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

//! Add a cloud entry built from an application credential.
//!
//! Hand-written command implementing
//! <https://github.com/gtema/openstack/issues/1323>.

use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};

use clap::Args;
use eyre::{OptionExt, WrapErr, eyre};
use serde::Serialize;
use tracing::info;
use zeroize::{ZeroizeOnDrop, Zeroizing};

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_core::config::{CloudConfig, find_clouds_file, find_secure_file};

/// Add a cloud entry built from an application credential.
///
/// Reads the JSON printed by `osc identity user application-credential
/// create -o json` from stdin and merges a ready-to-use cloud entry into
/// clouds.yaml (or, with --split, the credential into secure.yaml). The
/// entry inherits connection settings (auth_url, region, TLS options) from
/// the cloud selected with `--os-cloud`; no authentication is performed.
///
/// An existing target file is merged into, but rewritten: comments and
/// formatting are not preserved.
#[derive(Args)]
#[command(about = "Add a cloud entry from an application credential")]
pub struct AddCommand {
    /// Name of the cloud entry to add.
    #[arg(default_value = "openstack", long)]
    cloud_name: String,

    /// Target clouds.yaml path. Defaults to `--os-client-config-file`, else
    /// the discovered standard clouds.yaml, else
    /// `$XDG_CONFIG_HOME/openstack/clouds.yaml`.
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,

    /// Write the credential id and secret into a separate secure.yaml
    /// instead of clouds.yaml.
    #[arg(action = clap::ArgAction::SetTrue, long)]
    split: bool,

    /// Replace an existing cloud entry of the same name.
    #[arg(action = clap::ArgAction::SetTrue, long)]
    overwrite: bool,
}

impl AddCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(
        &self,
        parsed_args: &C,
        cloud_config: &CloudConfig,
    ) -> Result<(), OpenStackCliError> {
        info!("Add cloud entry to clouds.yaml");

        let input = read_stdin()?;
        let (credential_id, secret) = extract_credential(&input)?;

        let connection = &parsed_args.global_opts().connection;
        let clouds_path = resolve_clouds_path(
            self.file.as_deref(),
            connection.os_client_config_file.as_deref(),
        );
        let secure_path = self.split.then(|| {
            resolve_secure_path(
                self.file.as_deref(),
                connection.os_client_secure_file.as_deref(),
                &clouds_path,
            )
        });

        // Merge is the default: an existing target is read and the entry
        // added to it.
        let clouds_existing = read_existing(&clouds_path)?;
        let secure_existing = secure_path
            .as_deref()
            .map(read_existing)
            .transpose()?
            .flatten();

        let (clouds_entry, secure_entry) =
            build_entries(cloud_config, &credential_id, &secret, self.split)?;

        // Render everything before writing anything: a same-name collision
        // or malformed target aborts with no file touched. The rendered
        // contents carry the credential; wipe them on drop.
        let clouds_content = Zeroizing::new(render_target(
            clouds_existing.as_deref(),
            &self.cloud_name,
            &clouds_entry,
            self.overwrite,
        )?);
        let secure_content = secure_entry
            .as_ref()
            .map(|entry| {
                render_target(
                    secure_existing.as_deref(),
                    &self.cloud_name,
                    entry,
                    self.overwrite,
                )
                .map(Zeroizing::new)
            })
            .transpose()?;

        write_yaml_file(&clouds_path, &clouds_content, !self.split)?;
        println!(
            "Added cloud `{}` to {}",
            self.cloud_name,
            clouds_path.display()
        );
        if let (Some(path), Some(content)) = (&secure_path, &secure_content) {
            write_yaml_file(path, content, true)?;
            println!("Added cloud `{}` to {}", self.cloud_name, path.display());
        }
        Ok(())
    }
}

/// Read the application credential JSON from stdin.
fn read_stdin() -> Result<Zeroizing<String>, eyre::Report> {
    let mut stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Err(eyre!(
            "the application credential JSON is expected on stdin, e.g. `osc identity user application-credential create --name foo -o json | osc config clouds add`"
        ));
    }
    let mut buf = Zeroizing::new(String::new());
    stdin
        .read_to_string(&mut buf)
        .wrap_err("cannot read stdin")?;
    Ok(buf)
}

/// Extract the credential id and secret from the piped create response.
/// Both the bare resource (the `-o json` output) and the
/// `{"application_credential": {...}}` wrapped API form are accepted.
fn extract_credential(input: &str) -> Result<(Zeroizing<String>, Zeroizing<String>), eyre::Report> {
    let doc: serde_json::Value = serde_json::from_str(input).wrap_err("stdin is not valid JSON")?;
    let resource = doc.get("application_credential").unwrap_or(&doc);
    let id = resource
        .get("id")
        .and_then(|v| v.as_str())
        .map(|v| Zeroizing::new(v.to_string()))
        .ok_or_eyre("the input is missing the credential `id`")?;
    let secret = resource
        .get("secret")
        .and_then(|v| v.as_str())
        .map(|v| Zeroizing::new(v.to_string()))
        .ok_or_eyre(
            "the input is missing the credential `secret` (the secret is only returned by the create call)",
        )?;
    Ok((id, secret))
}

/// Resolve the target clouds.yaml: `--file` > `--os-client-config-file` >
/// the discovered standard file > the XDG default location.
fn resolve_clouds_path(file: Option<&Path>, os_client_config_file: Option<&str>) -> PathBuf {
    if let Some(path) = file {
        path.to_path_buf()
    } else if let Some(path) = os_client_config_file {
        PathBuf::from(path)
    } else if let Some(path) = find_clouds_file() {
        path
    } else {
        default_config_dir().join("clouds.yaml")
    }
}

/// Resolve the secure.yaml target. With `--file` the sibling secure.yaml is
/// used so the pair stays together; otherwise `--os-client-secure-file` >
/// the discovered standard file > sibling of the resolved clouds.yaml.
fn resolve_secure_path(
    file: Option<&Path>,
    os_client_secure_file: Option<&str>,
    clouds_path: &Path,
) -> PathBuf {
    if file.is_none() {
        if let Some(path) = os_client_secure_file {
            return PathBuf::from(path);
        }
        if let Some(path) = find_secure_file() {
            return path;
        }
    }
    clouds_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .join("secure.yaml")
}

/// `$XDG_CONFIG_HOME/openstack`, matching the SDK config file discovery.
fn default_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("openstack")
}

/// One entry under the `clouds:` key of a clouds.yaml/secure.yaml file.
#[derive(Debug, Default, Serialize)]
struct CloudEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_type: Option<String>,
    auth: AuthBlock,
    #[serde(skip_serializing_if = "Option::is_none")]
    region_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interface: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cacert: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    verify: Option<bool>,
}

/// The `auth` block of a cloud entry. The block holds the credential;
/// [`ZeroizeOnDrop`] wipes it from memory on drop.
#[derive(Debug, Default, Serialize, ZeroizeOnDrop)]
struct AuthBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_credential_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_credential_secret: Option<String>,
}

/// Build the clouds.yaml entry and, in split mode, the secure.yaml entry
/// carrying the credential id and secret.
fn build_entries(
    config: &CloudConfig,
    credential_id: &str,
    secret: &str,
    split: bool,
) -> Result<(CloudEntry, Option<CloudEntry>), eyre::Report> {
    let auth_url = config
        .auth
        .as_ref()
        .and_then(|auth| auth.auth_url.clone())
        .ok_or_eyre("cannot determine the auth_url of the current cloud")?;

    let clouds_entry = CloudEntry {
        auth_type: Some("v3applicationcredential".into()),
        auth: AuthBlock {
            auth_url: Some(auth_url),
            application_credential_id: (!split).then(|| credential_id.into()),
            application_credential_secret: (!split).then(|| secret.into()),
        },
        region_name: config.region_name.clone(),
        // `interface` is serde-defaulted to "public" on config load; only a
        // non-default value is worth exporting.
        interface: config.interface.clone().filter(|i| i != "public"),
        cacert: config.cacert.clone(),
        verify: config.verify,
    };
    let secure_entry = split.then(|| CloudEntry {
        auth: AuthBlock {
            auth_url: None,
            application_credential_id: Some(credential_id.into()),
            application_credential_secret: Some(secret.into()),
        },
        ..Default::default()
    });
    Ok((clouds_entry, secure_entry))
}

/// Produce the final YAML for a target file. `existing` carries the current
/// file content when merging into it; comments in it are not preserved.
fn render_target(
    existing: Option<&str>,
    cloud_name: &str,
    entry: &CloudEntry,
    overwrite_entry: bool,
) -> Result<String, eyre::Report> {
    match existing {
        None => {
            let mut clouds = serde_yaml::Mapping::new();
            clouds.insert(cloud_name.into(), serde_yaml::to_value(entry)?);
            let mut root = serde_yaml::Mapping::new();
            root.insert("clouds".into(), serde_yaml::Value::Mapping(clouds));
            Ok(serde_yaml::to_string(&serde_yaml::Value::Mapping(root))?)
        }
        Some(current) => {
            let mut doc: serde_yaml::Value =
                serde_yaml::from_str(current).wrap_err("the target file is not valid YAML")?;
            let root = doc
                .as_mapping_mut()
                .ok_or_eyre("the target file is not a YAML mapping")?;
            let clouds = root
                .entry("clouds".into())
                .or_insert_with(|| serde_yaml::Value::Mapping(Default::default()))
                .as_mapping_mut()
                .ok_or_eyre("`clouds` in the target file is not a mapping")?;
            let name_key: serde_yaml::Value = cloud_name.into();
            if clouds.contains_key(&name_key) && !overwrite_entry {
                return Err(eyre!(
                    "cloud `{cloud_name}` already exists in the target file; pass --overwrite to replace it"
                ));
            }
            clouds.insert(name_key, serde_yaml::to_value(entry)?);
            Ok(serde_yaml::to_string(&doc)?)
        }
    }
}

/// Read the current content of a target file; `None` when it does not
/// exist.
fn read_existing(path: &Path) -> Result<Option<String>, eyre::Report> {
    if !path.exists() {
        return Ok(None);
    }
    Ok(Some(std::fs::read_to_string(path).wrap_err_with(|| {
        format!("cannot read {}", path.display())
    })?))
}

/// Write a YAML file, creating parent directories; files carrying the
/// credential secret get 0600.
fn write_yaml_file(path: &Path, content: &str, contains_secret: bool) -> Result<(), eyre::Report> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("cannot create {}", parent.display()))?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create(true).truncate(true);
    #[cfg(unix)]
    if contains_secret {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .wrap_err_with(|| format!("cannot open {} for writing", path.display()))?;
    file.write_all(content.as_bytes())
        .wrap_err_with(|| format!("cannot write {}", path.display()))?;
    // The mode above only applies on creation; harden pre-existing files too.
    #[cfg(unix)]
    if contains_secret {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
            .wrap_err_with(|| format!("cannot set permissions on {}", path.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with(auth_url: Option<&str>) -> CloudConfig {
        CloudConfig {
            auth: Some(openstack_sdk_core::config::Auth {
                auth_url: auth_url.map(Into::into),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn extract_bare_and_wrapped_input() {
        let bare = r#"{"id": "cid", "secret": "sec", "name": "foo"}"#;
        let (id, secret) = extract_credential(bare).unwrap();
        assert_eq!(id.as_str(), "cid");
        assert_eq!(secret.as_str(), "sec");

        let wrapped = r#"{"application_credential": {"id": "cid", "secret": "sec"}}"#;
        let (id, secret) = extract_credential(wrapped).unwrap();
        assert_eq!(id.as_str(), "cid");
        assert_eq!(secret.as_str(), "sec");
    }

    #[test]
    fn extract_rejects_bad_input() {
        assert!(extract_credential("not json").is_err());
        assert!(extract_credential(r#"{"secret": "sec"}"#).is_err());
        assert!(extract_credential(r#"{"id": "cid"}"#).is_err());
        assert!(extract_credential(r#"{"id": "cid", "secret": null}"#).is_err());
    }

    #[test]
    fn clouds_path_resolution_order() {
        assert_eq!(
            resolve_clouds_path(Some(Path::new("/tmp/c.yaml")), Some("/tmp/os.yaml")),
            PathBuf::from("/tmp/c.yaml")
        );
        assert_eq!(
            resolve_clouds_path(None, Some("/tmp/os.yaml")),
            PathBuf::from("/tmp/os.yaml")
        );
    }

    #[test]
    fn secure_path_sibling_of_explicit_file() {
        // With --file the secure.yaml must live next to it, even when a
        // standard secure file would be discovered.
        assert_eq!(
            resolve_secure_path(
                Some(Path::new("/tmp/foo/clouds.yaml")),
                Some("/other/secure.yaml"),
                Path::new("/tmp/foo/clouds.yaml"),
            ),
            PathBuf::from("/tmp/foo/secure.yaml")
        );
        // Without --file the explicit secure file option wins.
        assert_eq!(
            resolve_secure_path(None, Some("/other/secure.yaml"), Path::new("/tmp/c.yaml")),
            PathBuf::from("/other/secure.yaml")
        );
    }

    #[test]
    fn single_file_entry_carries_secret() {
        let (clouds, secure) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        assert_eq!(clouds.auth_type.as_deref(), Some("v3applicationcredential"));
        assert_eq!(
            clouds.auth.auth_url.as_deref(),
            Some("https://keystone:5000")
        );
        assert_eq!(
            clouds.auth.application_credential_id.as_deref(),
            Some("cid")
        );
        assert_eq!(
            clouds.auth.application_credential_secret.as_deref(),
            Some("sec")
        );
        assert!(secure.is_none());
    }

    #[test]
    fn split_moves_credentials_to_secure_entry() {
        let (clouds, secure) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            true,
        )
        .unwrap();
        assert!(clouds.auth.application_credential_secret.is_none());
        assert!(clouds.auth.application_credential_id.is_none());
        let secure = secure.expect("secure entry in split mode");
        assert_eq!(
            secure.auth.application_credential_secret.as_deref(),
            Some("sec")
        );
        assert_eq!(
            secure.auth.application_credential_id.as_deref(),
            Some("cid")
        );
        assert!(secure.auth.auth_url.is_none());
        assert!(secure.auth_type.is_none());
    }

    #[test]
    fn inherits_allowlist_but_skips_default_interface() {
        let mut config = config_with(Some("https://keystone:5000"));
        config.region_name = Some("RegionOne".into());
        config.cacert = Some("/etc/ssl/custom.pem".into());
        config.verify = Some(false);
        config.interface = Some("public".into());
        let (clouds, _) = build_entries(&config, "cid", "sec", false).unwrap();
        assert_eq!(clouds.region_name.as_deref(), Some("RegionOne"));
        assert_eq!(clouds.cacert.as_deref(), Some("/etc/ssl/custom.pem"));
        assert_eq!(clouds.verify, Some(false));
        assert!(
            clouds.interface.is_none(),
            "default interface must be omitted"
        );

        config.interface = Some("internal".into());
        let (clouds, _) = build_entries(&config, "cid", "sec", false).unwrap();
        assert_eq!(clouds.interface.as_deref(), Some("internal"));
    }

    #[test]
    fn missing_auth_url_is_an_error() {
        assert!(build_entries(&config_with(None), "cid", "sec", false).is_err());
        assert!(build_entries(&CloudConfig::default(), "cid", "sec", false).is_err());
    }

    #[test]
    fn render_fresh_file() {
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(None, "mycloud", &entry, false).unwrap();
        let doc: serde_yaml::Value = serde_yaml::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth"]["application_credential_id"],
            serde_yaml::Value::String("cid".into())
        );
        assert_eq!(
            doc["clouds"]["mycloud"]["auth_type"],
            serde_yaml::Value::String("v3applicationcredential".into())
        );
    }

    #[test]
    fn merge_preserves_other_clouds() {
        let existing = "clouds:\n  other:\n    auth:\n      auth_url: https://other:5000\n";
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(Some(existing), "mycloud", &entry, false).unwrap();
        let doc: serde_yaml::Value = serde_yaml::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["other"]["auth"]["auth_url"],
            serde_yaml::Value::String("https://other:5000".into())
        );
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn merge_same_name_errors_without_overwrite() {
        let existing = "clouds:\n  mycloud:\n    auth:\n      auth_url: https://old:5000\n";
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        assert!(render_target(Some(existing), "mycloud", &entry, false).is_err());
        let out = render_target(Some(existing), "mycloud", &entry, true).unwrap();
        let doc: serde_yaml::Value = serde_yaml::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth"]["application_credential_id"],
            serde_yaml::Value::String("cid".into())
        );
    }

    #[test]
    fn merge_into_malformed_yaml_errors() {
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        assert!(render_target(Some(": not yaml : ["), "mycloud", &entry, false).is_err());
        assert!(render_target(Some("- a\n- list\n"), "mycloud", &entry, false).is_err());
    }

    #[test]
    fn read_existing_semantics() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clouds.yaml");
        assert!(read_existing(&path).unwrap().is_none());

        std::fs::write(&path, "clouds: {}\n").unwrap();
        assert_eq!(
            read_existing(&path).unwrap().as_deref(),
            Some("clouds: {}\n")
        );
    }

    #[test]
    fn write_creates_parents_and_sets_mode() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/dir/clouds.yaml");
        write_yaml_file(&path, "clouds: {}\n", true).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "clouds: {}\n");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = std::fs::metadata(&path).unwrap().permissions().mode();
            assert_eq!(mode & 0o777, 0o600);
        }
    }
}
