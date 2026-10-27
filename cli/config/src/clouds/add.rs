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
use secrecy::SecretString;
use tracing::info;
use zeroize::Zeroizing;

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_core::config::{
    Auth, CloudConfig, ConfigFile, find_clouds_file, find_secure_file,
};

use crate::yaml_edit;

/// Add a cloud entry built from an application credential.
///
/// Reads the JSON printed by `osc identity user application-credential
/// create -o json` from stdin and merges a ready-to-use cloud entry into
/// clouds.yaml (or, with --split, the credential secret into secure.yaml).
/// The entry inherits the settings of the cloud selected with `--os-cloud`,
/// which is required here (auth_url, region, TLS and any other option); of
/// its `auth` block only `auth_url` is kept. No authentication is performed.
///
/// An existing target file is edited in place: the new entry is spliced in
/// and everything else (comments, `&anchor`/`<<: *anchor` merge keys,
/// formatting) is left byte-for-byte as it was.
#[derive(Debug, Args)]
#[command(about = "Add a cloud entry from an application credential")]
pub struct AddCommand {
    /// Name of the cloud entry to add. Letters, digits, `.`, `_` and `-`
    /// only.
    #[arg(default_value = "openstack", long, value_parser = parse_cloud_name)]
    cloud_name: String,

    /// Target clouds.yaml path. Defaults to `--os-client-config-file`, else
    /// the discovered standard clouds.yaml, else
    /// `$XDG_CONFIG_HOME/openstack/clouds.yaml`.
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,

    /// Write the credential secret into a separate secure.yaml instead of
    /// clouds.yaml; the credential id stays in clouds.yaml.
    #[arg(action = clap::ArgAction::SetTrue, long)]
    split: bool,

    /// Update an existing cloud entry of the same name.
    ///
    /// The credential and the settings this command manages are replaced;
    /// any other key you have set on the entry by hand (e.g. `interface`,
    /// `cacert`) is kept, as are all comments in the file.
    ///
    /// An entry of the same name in secure.yaml counts too, as it is
    /// merged over clouds.yaml on load. Without --split its old credential
    /// is removed from there, so the one written to clouds.yaml takes
    /// effect.
    #[arg(action = clap::ArgAction::SetTrue, long)]
    overwrite: bool,
}

impl AddCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        info!("Add cloud entry to clouds.yaml");

        let cloud_config = resolve_source_cloud(parsed_args)?;

        let input = read_stdin()?;
        let (credential_id, secret) = extract_credential(&input)?;

        let connection = &parsed_args.global_opts().connection;
        let clouds_path = resolve_clouds_path(
            self.file.as_deref(),
            connection.os_client_config_file.as_deref(),
        );
        // Resolved also without --split: a credential already in
        // secure.yaml would override the one written to clouds.yaml.
        let secure_path = resolve_secure_path(
            self.file.as_deref(),
            connection.os_client_secure_file.as_deref(),
            &clouds_path,
        );

        // Merge is the default: an existing target is read and the entry
        // added to it.
        let clouds_existing = read_existing(&clouds_path)?;
        let secure_existing = read_existing(&secure_path)?;

        let (clouds_entry, secure_entry) =
            build_entries(&cloud_config, &credential_id, &secret, self.split)?;

        // Render everything before writing anything: a same-name collision
        // or malformed target aborts with no file touched. The rendered
        // contents carry the credential; wipe them on drop.
        let clouds_content = Zeroizing::new(
            render_target(
                clouds_existing.as_deref(),
                &self.cloud_name,
                &clouds_entry,
                self.overwrite,
                CLEARED_KEYS,
            )
            .wrap_err_with(|| format!("cannot update {}", clouds_path.display()))?,
        );
        let secure_content = render_secure(
            secure_existing.as_deref(),
            &self.cloud_name,
            secure_entry.as_ref(),
            self.overwrite,
        )
        .wrap_err_with(|| format!("cannot update {}", secure_path.display()))?
        .map(Zeroizing::new);

        // Stage both files before moving either into place, so a failure
        // writing one does not leave the pair half-updated.
        let clouds_file = StagedFile::new(&clouds_path, &clouds_content, !self.split)?;
        let has_secure = secure_content.is_some();
        let secure_file = secure_content
            .as_deref()
            .map(|content| StagedFile::new(&secure_path, content, true))
            .transpose()?;

        // The two renames cannot be made atomic together: if the second
        // fails, say which file was already replaced.
        clouds_file.commit()?;
        if let Some(file) = secure_file {
            file.commit().wrap_err_with(|| {
                format!(
                    "{} was updated but {} was not; fix the error and re-run with --overwrite",
                    clouds_path.display(),
                    secure_path.display()
                )
            })?;
        }

        let verb = if self.overwrite { "Updated" } else { "Added" };
        println!(
            "{verb} cloud `{}` in {}",
            self.cloud_name,
            clouds_path.display()
        );
        if has_secure {
            if self.split {
                println!(
                    "{verb} cloud `{}` in {}",
                    self.cloud_name,
                    secure_path.display()
                );
            } else {
                println!(
                    "Removed the old credential of cloud `{}` from {}",
                    self.cloud_name,
                    secure_path.display()
                );
            }
        }
        Ok(())
    }
}

/// Accept a cloud name only from a conservative character set that YAML
/// reads back as a string, so it can be written as a plain key without
/// quoting or escaping.
fn parse_cloud_name(name: &str) -> Result<String, String> {
    let charset_ok = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if !charset_ok {
        return Err(String::from(
            "the cloud name must be non-empty and contain only ASCII letters, digits, `.`, `_` and `-`",
        ));
    }
    // `null`, `true`, `123`, `1.5`, `-` ... would be read back as something
    // other than the name.
    if !matches!(
        yaml_serde::from_str::<yaml_serde::Value>(name),
        Ok(yaml_serde::Value::String(_))
    ) {
        return Err(format!(
            "the cloud name `{name}` would be read as a non-string YAML value; choose another one"
        ));
    }
    Ok(name.to_string())
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

/// Build the cloud entry and, in split mode, the secure.yaml entry.
///
/// The entry is the source cloud with its credential swapped for the
/// application credential: every non-credential setting (region, TLS
/// options, `*_api_version` and other extra keys) is inherited, while the
/// old `auth` block is dropped except for `auth_url` - an application
/// credential is already scoped, so a project/domain would be rejected.
///
/// The source is always run through [`CloudConfig::split_sensitive`], also
/// without `--split`, to refuse a source carrying a short-lived secret
/// (e.g. `token`) that would otherwise be persisted and to leave the
/// secrets of its own credential behind.
fn build_entries(
    source: &CloudConfig,
    credential_id: &str,
    secret: &str,
    split: bool,
) -> Result<(CloudConfig, Option<CloudConfig>), eyre::Report> {
    let auth_url = source
        .auth
        .as_ref()
        .and_then(|auth| auth.auth_url.clone())
        .ok_or_eyre("cannot determine the auth_url of the current cloud")?;

    // The public half of the source drops the secrets of its own credential
    // that live outside `auth` (plugin specific `options` such as a
    // `client_secret`), so they are not inherited into the new entry. It
    // also refuses a source carrying a short-lived secret.
    let mut entry = source.split_sensitive()?.public;
    entry.auth = Some(Auth {
        auth_url: Some(auth_url),
        application_credential_id: Some(credential_id.into()),
        application_credential_secret: Some(SecretString::from(secret)),
        ..Default::default()
    });
    entry.auth_type = Some("v3applicationcredential".into());
    // Keep in sync with `CLEARED_KEYS`.
    entry.auth_methods = None;
    entry.name = None;
    // `interface` is serde-defaulted to "public" on config load, so every
    // loaded config has it; only a non-default value is worth exporting.
    entry.interface = entry.interface.filter(|i| i != "public");

    let split_config = entry.split_sensitive()?;
    Ok(if split {
        (split_config.public, Some(split_config.secure))
    } else {
        (entry, None)
    })
}

/// Top-level keys [`build_entries`] clears. An unset field is not
/// serialized, so merging the entry over an existing one would keep the old
/// value; [`render_target`] removes them explicitly.
const CLEARED_KEYS: &[&str] = &["auth_methods"];

/// Keys selecting a cloud's credential. secure.yaml is merged over
/// clouds.yaml on load, so any of them left in a secure.yaml entry would
/// override the new one.
const CREDENTIAL_KEYS: &[&str] = &["auth_type", "auth_methods", "auth"];

/// The [`CREDENTIAL_KEYS`] a split secure entry does not set: it carries
/// only `auth`, so a stale `auth_type` or `auth_methods` must go.
const SECURE_CLEARED_KEYS: &[&str] = &["auth_type", "auth_methods"];

/// Resolve the cloud whose connection settings the new entry inherits.
///
/// `osc config` is dispatched before `entry_point` resolves a cloud, since
/// most config commands need none; this command does, so it repeats the
/// lookup here. Unlike `entry_point` it never falls back to the
/// interactive picker: writing a config file is not a prompt-and-guess
/// operation, so the cloud must be named explicitly.
fn resolve_source_cloud<C: CliArgs>(parsed_args: &C) -> Result<CloudConfig, eyre::Report> {
    let connection = &parsed_args.global_opts().connection;

    let mut cloud_config = if connection.cloud_config_from_env {
        let mut cloud_config = CloudConfig::from_env()?;
        cloud_config.name = Some(
            connection
                .os_cloud_name
                .clone()
                .unwrap_or_else(|| String::from("envvars")),
        );
        cloud_config
    } else {
        let cloud_name = connection.os_cloud.as_ref().ok_or_eyre(
            "`--os-cloud` (or `OS_CLOUD`) must be given to say which cloud's connection settings the new entry inherits; `--cloud-config-from-env` works too",
        )?;
        let cfg = ConfigFile::new_with_user_specified_configs(
            connection.os_client_config_file.as_deref(),
            connection.os_client_secure_file.as_deref(),
        )?;
        cfg.get_cloud_config(cloud_name)?
            .ok_or_else(|| eyre!("cloud `{cloud_name}` is not configured"))?
    };

    if let Some(region_name) = &connection.os_region_name {
        cloud_config.region_name = Some(region_name.clone());
    }

    Ok(cloud_config)
}

/// Produce the final YAML for a target file.
///
/// `existing` carries the current file content when editing one in place.
/// The edit is a splice via [`yaml_edit`], so comments, `&anchor`/`<<:
/// *anchor` merge keys and formatting elsewhere in the file survive
/// byte-for-byte; only the targeted entry is rewritten.
///
/// On overwrite the `cleared` keys of the old entry are removed first;
/// see [`CLEARED_KEYS`].
fn render_target(
    existing: Option<&str>,
    cloud_name: &str,
    entry: &CloudConfig,
    overwrite_entry: bool,
    cleared: &[&str],
) -> Result<String, eyre::Report> {
    // A file with no YAML node at all - absent, empty, or nothing but
    // comments - has nothing to splice into. Build the block ourselves and
    // keep whatever was there as a prefix, so a comment-only file keeps its
    // header instead of tripping the parser.
    let Some(current) = existing.filter(|c| has_yaml_content(c)) else {
        let mut block = format!("clouds:\n  {cloud_name}:\n");
        for line in yaml_serde::to_string(entry)?.lines() {
            block.push_str("    ");
            block.push_str(line);
            block.push('\n');
        }
        return Ok(match existing {
            Some(preamble) if !preamble.trim().is_empty() => {
                format!("{}\n{block}", preamble.trim_end())
            }
            _ => block,
        });
    };

    let mut doc =
        yaml_edit::YamlDocument::parse(current).wrap_err("the target file is not valid YAML")?;

    if doc.contains_key("clouds", cloud_name) {
        if !overwrite_entry {
            return Err(eyre!(
                "cloud `{cloud_name}` already exists in the target file; pass --overwrite to replace it"
            ));
        }
        // Merge rather than replace: `upsert_mapping_entry` would drop the
        // comment attached to the next cloud entry. See
        // `merge_mapping_entry`'s doc comment for the trade-off - keys of
        // the old entry that this one does not set are kept, which for a
        // credential rotation is what the user wants. The keys this command
        // clears are the exception: dropped, not kept.
        for field in cleared {
            doc.remove_entry_field("clouds", cloud_name, field)
                .wrap_err_with(|| format!("could not update cloud `{cloud_name}`"))?;
        }
        doc.merge_mapping_entry("clouds", cloud_name, entry)
            .wrap_err_with(|| format!("could not update cloud `{cloud_name}`"))?;
    } else {
        doc.upsert_mapping_entry("clouds", cloud_name, entry)
            .wrap_err_with(|| format!("could not add cloud `{cloud_name}`"))?;
    }

    Ok(doc.source().to_string())
}

/// Produce the final YAML for secure.yaml, or `None` when it stays as is.
///
/// With `--split` (`entry` is set) the secure entry is rendered like any
/// target. Without it, a same-name entry there is still a collision: it
/// would override the credential written to clouds.yaml, so on overwrite
/// its [`CREDENTIAL_KEYS`] are removed. An entry left with no keys is
/// removed as a whole; either way `yamlpatch` drops a comment directly
/// following a removed block (see the `yaml_edit` tests).
fn render_secure(
    existing: Option<&str>,
    cloud_name: &str,
    entry: Option<&CloudConfig>,
    overwrite_entry: bool,
) -> Result<Option<String>, eyre::Report> {
    if let Some(entry) = entry {
        return render_target(
            existing,
            cloud_name,
            entry,
            overwrite_entry,
            SECURE_CLEARED_KEYS,
        )
        .map(Some);
    }

    let Some(current) = existing.filter(|c| has_yaml_content(c)) else {
        return Ok(None);
    };
    let mut doc =
        yaml_edit::YamlDocument::parse(current).wrap_err("the target file is not valid YAML")?;
    if !doc.contains_key("clouds", cloud_name) {
        return Ok(None);
    }
    if !overwrite_entry {
        return Err(eyre!(
            "cloud `{cloud_name}` already exists in the secure file; pass --overwrite to replace it"
        ));
    }
    for field in CREDENTIAL_KEYS {
        doc.remove_entry_field("clouds", cloud_name, field)
            .wrap_err_with(|| format!("could not update cloud `{cloud_name}`"))?;
    }
    Ok(Some(doc.source().to_string()))
}

/// Whether `text` holds any YAML node, as opposed to being empty or only
/// blank lines, comments and document markers.
fn has_yaml_content(text: &str) -> bool {
    text.lines()
        .map(str::trim)
        .any(|line| !line.is_empty() && !line.starts_with('#') && line != "---" && line != "...")
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

/// A target file's new content, written to a temporary sibling and not yet
/// moved into place.
///
/// The temporary file is created 0600 on unix, so a secret never sits in a
/// file readable by others, and the target is only replaced by a rename
/// once the content is fully on disk: a failed write leaves the original
/// untouched. Dropping it without [`commit`](Self::commit) removes the
/// temporary file.
struct StagedFile {
    temp: tempfile::NamedTempFile,
    target: PathBuf,
}

impl StagedFile {
    /// Write `content` next to `path`, creating parent directories. Files
    /// carrying the credential secret end up 0600; any other file keeps
    /// the permissions of the one it replaces.
    fn new(path: &Path, content: &str, contains_secret: bool) -> Result<Self, eyre::Report> {
        // Rename onto the file a symlink points at, not over the link itself.
        let target = match std::fs::canonicalize(path) {
            Ok(target) => target,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => path.to_path_buf(),
            Err(err) => {
                return Err(err).wrap_err_with(|| format!("cannot resolve {}", path.display()));
            }
        };
        let parent = target
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("cannot create {}", parent.display()))?;

        let existing_permissions = std::fs::metadata(&target).ok().map(|m| m.permissions());
        let file_name = target
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let prefix = format!(".{file_name}.");
        let mut builder = tempfile::Builder::new();
        builder.prefix(&prefix).suffix(".tmp");
        // A new file without the secret gets the usual umask-derived mode.
        #[cfg(unix)]
        if !contains_secret && existing_permissions.is_none() {
            use std::os::unix::fs::PermissionsExt;
            builder.permissions(std::fs::Permissions::from_mode(0o666));
        }
        let mut temp = builder
            .tempfile_in(parent)
            .wrap_err_with(|| format!("cannot create a temporary file in {}", parent.display()))?;
        if !contains_secret && let Some(permissions) = existing_permissions {
            temp.as_file()
                .set_permissions(permissions)
                .wrap_err_with(|| format!("cannot set permissions on {}", temp.path().display()))?;
        }
        temp.write_all(content.as_bytes())
            .and_then(|()| temp.as_file().sync_all())
            .wrap_err_with(|| format!("cannot write {}", temp.path().display()))?;
        Ok(Self { temp, target })
    }

    /// Move the new content into place.
    fn commit(self) -> Result<(), eyre::Report> {
        let target = self.target;
        self.temp
            .persist(&target)
            .map_err(|err| err.error)
            .wrap_err_with(|| format!("cannot replace {}", target.display()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;

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

    fn auth(entry: &CloudConfig) -> &Auth {
        entry.auth.as_ref().expect("entry has an auth block")
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
            auth(&clouds).auth_url.as_deref(),
            Some("https://keystone:5000")
        );
        assert_eq!(
            auth(&clouds).application_credential_id.as_deref(),
            Some("cid")
        );
        assert_eq!(
            auth(&clouds)
                .application_credential_secret
                .as_ref()
                .map(ExposeSecret::expose_secret),
            Some("sec")
        );
        assert!(secure.is_none());
    }

    #[test]
    fn split_moves_only_the_secret_to_secure_entry() {
        let (clouds, secure) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            true,
        )
        .unwrap();
        assert!(auth(&clouds).application_credential_secret.is_none());
        assert_eq!(
            auth(&clouds).application_credential_id.as_deref(),
            Some("cid")
        );
        let secure = secure.expect("secure entry in split mode");
        assert_eq!(
            auth(&secure)
                .application_credential_secret
                .as_ref()
                .map(ExposeSecret::expose_secret),
            Some("sec")
        );
        assert!(auth(&secure).application_credential_id.is_none());
        assert!(auth(&secure).auth_url.is_none());
        assert!(secure.auth_type.is_none());

        let yaml = yaml_serde::to_string(&secure).unwrap();
        assert_eq!(yaml, "auth:\n  application_credential_secret: sec\n");
    }

    #[test]
    fn inherits_settings_but_not_the_old_credential() {
        let mut config = config_with(Some("https://keystone:5000"));
        config.auth.as_mut().unwrap().username = Some("admin".into());
        config.auth.as_mut().unwrap().password = Some(SecretString::from("hunter2"));
        config.auth.as_mut().unwrap().project_name = Some("demo".into());
        config.auth_type = Some("v3password".into());
        config.name = Some("src".into());
        config.region_name = Some("RegionOne".into());
        config.cacert = Some("/etc/ssl/custom.pem".into());
        config.verify = Some(false);
        config.interface = Some("public".into());
        config
            .options
            .insert("identity_api_version".into(), config::Value::from("3"));
        let (clouds, _) = build_entries(&config, "cid", "sec", false).unwrap();
        assert_eq!(clouds.region_name.as_deref(), Some("RegionOne"));
        assert_eq!(clouds.cacert.as_deref(), Some("/etc/ssl/custom.pem"));
        assert_eq!(clouds.verify, Some(false));
        assert!(clouds.options.contains_key("identity_api_version"));
        assert!(clouds.name.is_none());
        assert!(
            clouds.interface.is_none(),
            "default interface must be omitted"
        );
        let yaml = yaml_serde::to_string(&clouds).unwrap();
        for stale in ["admin", "hunter2", "demo", "v3password"] {
            assert!(!yaml.contains(stale), "stale `{stale}` leaked:\n{yaml}");
        }

        config.interface = Some("internal".into());
        let (clouds, _) = build_entries(&config, "cid", "sec", false).unwrap();
        assert_eq!(clouds.interface.as_deref(), Some("internal"));
    }

    #[test]
    fn source_with_ephemeral_secret_is_refused() {
        let mut config = config_with(Some("https://keystone:5000"));
        config
            .options
            .insert("token".into(), config::Value::from("tok"));
        assert!(build_entries(&config, "cid", "sec", false).is_err());
        assert!(build_entries(&config, "cid", "sec", true).is_err());
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
        let out = render_target(None, "mycloud", &entry, false, CLEARED_KEYS).unwrap();
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth"]["application_credential_id"],
            yaml_serde::Value::String("cid".into())
        );
        assert_eq!(
            doc["clouds"]["mycloud"]["auth_type"],
            yaml_serde::Value::String("v3applicationcredential".into())
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
        let out = render_target(Some(existing), "mycloud", &entry, false, CLEARED_KEYS).unwrap();
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["other"]["auth"]["auth_url"],
            yaml_serde::Value::String("https://other:5000".into())
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
        assert!(render_target(Some(existing), "mycloud", &entry, false, CLEARED_KEYS).is_err());
        let out = render_target(Some(existing), "mycloud", &entry, true, CLEARED_KEYS).unwrap();
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth"]["application_credential_id"],
            yaml_serde::Value::String("cid".into())
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
        assert!(
            render_target(
                Some(": not yaml : ["),
                "mycloud",
                &entry,
                false,
                CLEARED_KEYS
            )
            .is_err()
        );
        assert!(
            render_target(
                Some("- a\n- list\n"),
                "mycloud",
                &entry,
                false,
                CLEARED_KEYS
            )
            .is_err()
        );
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

    #[cfg(unix)]
    fn mode(path: &Path) -> u32 {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[cfg(unix)]
    fn set_mode(path: &Path, mode: u32) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode)).unwrap();
    }

    /// The files in `dir`, to check no temporary file is left behind.
    fn dir_entries(dir: &Path) -> Vec<String> {
        let mut names: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        names
    }

    #[test]
    fn write_creates_parents_and_sets_mode() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/dir/clouds.yaml");
        StagedFile::new(&path, "clouds: {}\n", true)
            .unwrap()
            .commit()
            .unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "clouds: {}\n");
        assert_eq!(dir_entries(path.parent().unwrap()), ["clouds.yaml"]);
        #[cfg(unix)]
        assert_eq!(mode(&path), 0o600);
    }

    #[cfg(unix)]
    #[test]
    fn write_hardens_existing_secret_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clouds.yaml");
        std::fs::write(&path, "old\n").unwrap();
        set_mode(&path, 0o644);
        // Staging alone must not expose the secret: the original is
        // untouched and the staged copy is already private.
        let staged = StagedFile::new(&path, "secret\n", true).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old\n");
        assert_eq!(mode(staged.temp.path()), 0o600);
        staged.commit().unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "secret\n");
        assert_eq!(mode(&path), 0o600);
    }

    #[cfg(unix)]
    #[test]
    fn write_keeps_mode_of_existing_public_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clouds.yaml");
        std::fs::write(&path, "old\n").unwrap();
        set_mode(&path, 0o640);
        StagedFile::new(&path, "new\n", false)
            .unwrap()
            .commit()
            .unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new\n");
        assert_eq!(mode(&path), 0o640);
    }

    #[test]
    fn uncommitted_write_leaves_target_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("clouds.yaml");
        std::fs::write(&path, "old\n").unwrap();
        drop(StagedFile::new(&path, "new\n", true).unwrap());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "old\n");
        assert_eq!(dir_entries(dir.path()), ["clouds.yaml"]);
    }

    #[cfg(unix)]
    #[test]
    fn write_through_symlink_keeps_the_link() {
        let dir = tempfile::tempdir().unwrap();
        let real = dir.path().join("real.yaml");
        let link = dir.path().join("clouds.yaml");
        std::fs::write(&real, "old\n").unwrap();
        std::os::unix::fs::symlink(&real, &link).unwrap();
        StagedFile::new(&link, "new\n", true)
            .unwrap()
            .commit()
            .unwrap();
        assert!(std::fs::symlink_metadata(&link).unwrap().is_symlink());
        assert_eq!(std::fs::read_to_string(&real).unwrap(), "new\n");
    }

    #[test]
    fn cloud_name_is_restricted() {
        for ok in ["openstack", "my-cloud_2.prod"] {
            assert_eq!(parse_cloud_name(ok).as_deref(), Ok(ok));
        }
        for bad in [
            "",
            "null",
            "true",
            "123",
            "1.5",
            "-",
            "a: b",
            "#x",
            "a\nb",
            "my cloud",
            "\"q\"",
            "*ref",
            "\u{e9}t\u{e9}",
        ] {
            assert!(parse_cloud_name(bad).is_err(), "accepted {bad:?}");
        }
    }

    #[test]
    fn overwrite_drops_auth_methods() {
        let existing = "\
clouds:
  mycloud:
    auth_type: v3multifactor
    auth_methods:
      - v3password
      - v3totp
    auth:
      auth_url: https://old:5000
  # the other one
  other:
    auth_url: https://other:5000
";
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(Some(existing), "mycloud", &entry, true, CLEARED_KEYS).unwrap();
        assert!(!out.contains("auth_methods"), "stale auth_methods:\n{out}");
        assert!(!out.contains("v3totp"), "stale auth_methods:\n{out}");
        assert!(out.contains("# the other one"), "{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth_type"],
            yaml_serde::Value::String("v3applicationcredential".into())
        );
    }

    #[test]
    fn secure_only_entry_is_a_collision_without_split() {
        let secure = "clouds:\n  mycloud:\n    auth:\n      password: old\n";
        let err = render_secure(Some(secure), "mycloud", None, false).unwrap_err();
        assert!(err.to_string().contains("--overwrite"), "{err}");
        // Other names, and a missing or empty secure file, leave it alone.
        assert!(
            render_secure(Some(secure), "other", None, false)
                .unwrap()
                .is_none()
        );
        assert!(
            render_secure(None, "mycloud", None, false)
                .unwrap()
                .is_none()
        );
        assert!(
            render_secure(Some("# empty\n"), "mycloud", None, true)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn overwrite_without_split_removes_secure_credential() {
        let secure = "\
# secrets
clouds:
  mycloud:
    region_name: RegionSecure
    auth_type: v3password
    auth:
      username: me
      password: old
  other:
    auth:
      password: keep
";
        let out = render_secure(Some(secure), "mycloud", None, true)
            .unwrap()
            .expect("secure.yaml is updated");
        assert!(out.starts_with("# secrets\n"), "{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        let mycloud = &doc["clouds"]["mycloud"];
        assert!(mycloud.get("auth").is_none(), "{out}");
        assert!(mycloud.get("auth_type").is_none(), "{out}");
        // Keys the user set by hand are kept, as in clouds.yaml.
        assert_eq!(
            mycloud["region_name"],
            yaml_serde::Value::String("RegionSecure".into())
        );
        assert_eq!(
            doc["clouds"]["other"]["auth"]["password"],
            yaml_serde::Value::String("keep".into())
        );
    }

    #[test]
    fn overwrite_with_split_drops_stale_secure_auth_type() {
        let secure = "\
clouds:
  mycloud:
    auth_type: v3password
    auth:
      password: old
";
        let (_, secure_entry) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            true,
        )
        .unwrap();
        let out = render_secure(Some(secure), "mycloud", secure_entry.as_ref(), true)
            .unwrap()
            .expect("secure.yaml is updated");
        assert!(!out.contains("auth_type"), "{out}");
        assert!(!out.contains("password"), "{out}");
        assert!(out.contains("application_credential_secret: sec"), "{out}");
    }

    /// The case from review: the old credential lives only in secure.yaml.
    /// After an overwrite without --split the loaded cloud must use the
    /// new one.
    #[test]
    fn overwrite_without_split_new_credential_wins_on_load() {
        let dir = tempfile::tempdir().unwrap();
        let clouds_path = dir.path().join("clouds.yaml");
        let secure_path = dir.path().join("secure.yaml");
        let secure = "\
clouds:
  mycloud:
    auth_type: v3applicationcredential
    auth:
      application_credential_id: old-id
      application_credential_secret: old-secret
";
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let clouds = render_target(None, "mycloud", &entry, true, CLEARED_KEYS).unwrap();
        let secure = render_secure(Some(secure), "mycloud", None, true)
            .unwrap()
            .expect("secure.yaml is updated");
        std::fs::write(&clouds_path, clouds).unwrap();
        std::fs::write(&secure_path, secure).unwrap();

        let cfg = ConfigFile::new_with_user_specified_configs(
            Some(clouds_path.to_str().unwrap()),
            Some(secure_path.to_str().unwrap()),
        )
        .unwrap();
        let cloud = cfg.get_cloud_config("mycloud").unwrap().unwrap();
        let auth = cloud.auth.unwrap();
        assert_eq!(auth.application_credential_id.as_deref(), Some("cid"));
        assert_eq!(
            auth.application_credential_secret
                .as_ref()
                .map(secrecy::ExposeSecret::expose_secret),
            Some("sec")
        );
    }

    /// A realistic hand-maintained clouds.yaml: header comment, an
    /// `&anchor` base entry, a `<<: *anchor` consumer, inline comments and
    /// a trailing comment.
    fn handwritten_clouds_yaml() -> &'static str {
        "\
# Managed by hand - please keep the comments!
clouds:
  base: &base
    region_name: RegionOne  # our only region
    interface: internal
  # the production cloud
  devstack:
    <<: *base
    auth_url: https://devstack:5000
# end of file
"
    }

    #[test]
    fn render_preserves_comments_and_anchors() {
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(
            Some(handwritten_clouds_yaml()),
            "mycloud",
            &entry,
            false,
            CLEARED_KEYS,
        )
        .unwrap();

        for expected in [
            "# Managed by hand - please keep the comments!",
            "&base",
            "# our only region",
            "# the production cloud",
            "<<: *base",
            "# end of file",
        ] {
            assert!(out.contains(expected), "{expected} must survive:\n{out}");
        }
        // The anchored entries must be byte-identical, not re-serialized
        // with the anchor expanded into inline keys.
        assert!(
            out.contains("  base: &base\n    region_name: RegionOne  # our only region\n"),
            "anchor block must be untouched:\n{out}"
        );
        assert!(
            out.contains("  devstack:\n    <<: *base\n    auth_url: https://devstack:5000\n"),
            "merge-key block must be untouched:\n{out}"
        );
        // And the new entry really landed.
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn overwrite_preserves_comments_and_keeps_hand_set_keys() {
        let existing = "\
clouds:
  # my cloud
  mycloud:
    auth_type: v3password
    interface: internal
    auth:
      auth_url: https://old:5000
      username: admin
      password: hunter2
  # the other one
  other:
    auth_url: https://other:5000
";
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(Some(existing), "mycloud", &entry, true, CLEARED_KEYS).unwrap();

        // Comments on both the edited entry and its neighbour survive.
        assert!(out.contains("# my cloud"), "{out}");
        assert!(out.contains("# the other one"), "{out}");
        // The old password credential is gone, not merged with the new one.
        assert!(!out.contains("username"), "stale username:\n{out}");
        assert!(!out.contains("hunter2"), "stale password:\n{out}");
        // A hand-set connection setting the new entry does not carry is kept.
        assert!(out.contains("interface: internal"), "{out}");

        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert_eq!(
            doc["clouds"]["mycloud"]["auth"]["application_credential_id"],
            yaml_serde::Value::String("cid".into())
        );
        assert_eq!(
            doc["clouds"]["mycloud"]["auth_type"],
            yaml_serde::Value::String("v3applicationcredential".into())
        );
    }

    #[test]
    fn render_into_empty_clouds_key() {
        // `clouds:` with nothing under it - `MergeInto` cannot handle this,
        // so `yaml_edit` falls back to building the mapping wholesale.
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(
            Some("# my clouds\nclouds:\n"),
            "mycloud",
            &entry,
            false,
            CLEARED_KEYS,
        )
        .unwrap();

        assert!(out.contains("# my clouds"), "{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn render_into_comment_only_file_keeps_the_header() {
        // Nothing to splice into, but the user's comments must not be lost
        // and the parser must not be handed a document with no root node.
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(
            Some("# just a header\n"),
            "mycloud",
            &entry,
            false,
            CLEARED_KEYS,
        )
        .unwrap();

        assert!(out.starts_with("# just a header"), "{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn render_into_blank_file_is_block_style() {
        // An existing but empty file behaves like a fresh one, and must not
        // come out in flow style (`clouds: { ... }`).
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(Some("\n  \n"), "mycloud", &entry, false, CLEARED_KEYS).unwrap();

        assert!(out.starts_with("clouds:\n  mycloud:\n"), "{out}");
        assert!(!out.contains('{'), "must not be flow style:\n{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn marker_only_file_is_treated_as_empty() {
        let (entry, _) = build_entries(
            &config_with(Some("https://keystone:5000")),
            "cid",
            "sec",
            false,
        )
        .unwrap();
        let out = render_target(Some("---\n"), "mycloud", &entry, false, CLEARED_KEYS).unwrap();
        assert!(out.starts_with("---\nclouds:\n  mycloud:\n"), "{out}");
        let doc: yaml_serde::Value = yaml_serde::from_str(&out).unwrap();
        assert!(doc["clouds"]["mycloud"]["auth"]["application_credential_id"].is_string());
    }

    #[test]
    fn plugin_secrets_in_options_are_not_inherited() {
        let mut config = config_with(Some("https://keystone:5000"));
        config
            .options
            .insert("client_secret".into(), config::Value::from("oidc-secret"));
        config
            .options
            .insert("identity_api_version".into(), config::Value::from("3"));
        for split in [false, true] {
            let (clouds, secure) = build_entries(&config, "cid", "sec", split).unwrap();
            assert!(!clouds.options.contains_key("client_secret"));
            assert!(clouds.options.contains_key("identity_api_version"));
            if let Some(secure) = secure {
                assert!(!secure.options.contains_key("client_secret"));
            }
        }
    }
}
