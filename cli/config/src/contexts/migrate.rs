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

//! Migrate `clouds.yaml` entries into `contexts.yaml`.

use std::path::PathBuf;

use clap::Args;
use eyre::{WrapErr, eyre};
use serde_json::{Map, Value, json};
use zeroize::Zeroizing;

use openstack_cli_core::cli::CliArgs;
use openstack_cli_core::error::OpenStackCliError;
use openstack_sdk_core::config::contexts::ContextsFile;
use openstack_sdk_core::config::{CloudConfig, ConfigFile};

use crate::clouds::add::{StagedFile, read_existing};
use crate::yaml_edit::YamlDocument;

/// Sections of the contexts file in the order they are written.
const SECTIONS: [&str; 4] = ["servers", "users", "scopes", "contexts"];

/// Short-lived credentials that `contexts.yaml` refuses.
const EPHEMERAL: [&str; 4] = ["token", "passcode", "access_token", "jwt"];

/// Migrate cloud entries of `clouds.yaml`/`secure.yaml` into `contexts.yaml`.
///
/// Every cloud becomes a server, a user, (when it is scoped) a scope and a
/// context, all named like the cloud. Clouds whose name is already used in
/// the target file are skipped, and so are clouds that cannot be expressed
/// (the reason is reported). `clouds.yaml` itself is not modified.
///
/// The user entries carry the credentials (a password, an application
/// credential secret), so a newly created file is only readable by you.
/// Short-lived credentials (`token`, `passcode`, `access_token`, `jwt`) are
/// not migrated. Settings without a place in `contexts.yaml` (i.e.
/// `compute_api_version`) are dropped with a warning.
#[derive(Debug, Args)]
pub struct MigrateCommand {
    /// Clouds to migrate (default: all).
    #[arg(value_name = "CLOUD")]
    clouds: Vec<String>,

    /// Path to the target `contexts.yaml` (default: `--os-context-file` or
    /// the standard search locations).
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,

    /// Make this context the `current-context` when the file sets none.
    #[arg(long, value_name = "CLOUD")]
    set_current: Option<String>,

    /// Only show what would be written.
    #[arg(action = clap::ArgAction::SetTrue, long)]
    dry_run: bool,
}

/// Entries one cloud turns into.
#[derive(Debug)]
struct Entries {
    server: Value,
    user: Value,
    scope: Option<Value>,
    context: Value,
    warnings: Vec<String>,
}

impl MigrateCommand {
    /// Perform command action
    pub fn take_action<C: CliArgs>(&self, parsed_args: &C) -> Result<(), OpenStackCliError> {
        let connection = &parsed_args.global_opts().connection;
        let cfg = ConfigFile::new_with_user_specified_configs(
            connection.os_client_config_file.as_deref(),
            connection.os_client_secure_file.as_deref(),
        )?;
        let mut names = cfg.get_available_clouds();
        names.sort();
        if !self.clouds.is_empty() {
            for wanted in &self.clouds {
                if !names.contains(wanted) {
                    return Err(eyre!("cloud `{wanted}` is not configured").into());
                }
            }
            names.retain(|n| self.clouds.contains(n));
        }
        if names.is_empty() {
            return Err(eyre!("no clouds to migrate").into());
        }

        let path = super::resolve_contexts_path(
            self.file.as_deref(),
            connection.os_context_file.as_deref(),
        );
        let existing = read_existing(&path)?;
        let current = match &existing {
            Some(text) => ContextsFile::from_yaml(text)
                .map_err(|err| eyre!("{} is not a valid contexts file: {err}", path.display()))?,
            None => ContextsFile::default(),
        };

        // Sections to write: existing items followed by the migrated ones.
        let mut sections: Vec<(&str, Vec<Value>)> = SECTIONS
            .iter()
            .map(|s| section_items(existing.as_deref(), s).map(|items| (*s, items)))
            .collect::<Result<_, _>>()?;
        let mut migrated = Vec::new();

        for name in &names {
            if used_name(&current, name) {
                println!(
                    "Skipped `{name}`: the name is already used in {}",
                    path.display()
                );
                continue;
            }
            let Some(cloud) = cfg.get_cloud_config(name)? else {
                continue;
            };
            let entries = match build_entries(name, &cloud) {
                Ok(entries) => entries,
                Err(reason) => {
                    println!("Skipped `{name}`: {reason}");
                    continue;
                }
            };
            // Each cloud must stand on its own as a valid file.
            let single = render_new(&entries)?;
            if let Err(err) = ContextsFile::from_yaml(&single) {
                println!("Skipped `{name}`: {err}");
                continue;
            }
            for warning in &entries.warnings {
                eprintln!("warning: {name}: {warning}");
            }
            sections[0].1.push(entries.server);
            sections[1].1.push(entries.user);
            if let Some(scope) = entries.scope {
                sections[2].1.push(scope);
            }
            sections[3].1.push(entries.context);
            migrated.push(name.clone());
        }

        if migrated.is_empty() {
            println!("Nothing to migrate");
            return Ok(());
        }

        let current_context = match &self.set_current {
            Some(name) if !migrated.contains(name) => {
                return Err(
                    eyre!("`--set-current {name}` is not among the migrated clouds").into(),
                );
            }
            Some(name) if current.current_context.is_none() => Some(name.clone()),
            _ => None,
        };

        let content = Zeroizing::new(render(
            existing.as_deref(),
            &sections,
            current_context.as_deref(),
        )?);
        // Whatever was assembled must be a valid file.
        let result = ContextsFile::from_yaml(&content).map_err(|err| eyre!("{err}"))?;

        if self.dry_run {
            println!("{}", content.as_str());
            return Ok(());
        }
        StagedFile::new(&path, &content, result.contains_secrets())?
            .commit()
            .wrap_err_with(|| format!("cannot update {}", path.display()))?;
        println!(
            "Migrated {} cloud(s) into {}: {}",
            migrated.len(),
            path.display(),
            migrated.join(", ")
        );
        Ok(())
    }
}

/// Whether any entry kind of the file already uses `name`.
fn used_name(file: &ContextsFile, name: &str) -> bool {
    file.servers.iter().any(|x| x.name == name)
        || file.users.iter().any(|x| x.name == name)
        || file.scopes.iter().any(|x| x.name == name)
        || file.contexts.iter().any(|x| x.name == name)
}

/// Items of the section in the existing document.
fn section_items(existing: Option<&str>, section: &str) -> Result<Vec<Value>, eyre::Report> {
    let Some(text) = existing else {
        return Ok(Vec::new());
    };
    let doc: Value = yaml_serde::from_str(text).wrap_err("cannot parse the contexts file")?;
    Ok(match doc.get(section) {
        Some(Value::Array(items)) => items.clone(),
        _ => Vec::new(),
    })
}

/// Render the document: spliced into the existing one (keeping what is
/// outside of the changed sections) or from scratch.
fn render(
    existing: Option<&str>,
    sections: &[(&str, Vec<Value>)],
    current_context: Option<&str>,
) -> Result<String, eyre::Report> {
    if let Some(text) = existing {
        let mut doc = YamlDocument::parse(text)?;
        if doc.has_root_mapping() {
            for (name, items) in sections {
                if !items.is_empty() {
                    doc.set_top_level(name, items)?;
                }
            }
            if let Some(current) = current_context {
                doc.set_top_level("current-context", &current)?;
            }
            return Ok(doc.source().to_string());
        }
    }
    let mut root = Map::new();
    if let Some(current) = current_context {
        root.insert("current-context".into(), json!(current));
    }
    for (name, items) in sections {
        if !items.is_empty() {
            root.insert((*name).into(), Value::Array(items.clone()));
        }
    }
    Ok(yaml_serde::to_string(&Value::Object(root))?)
}

/// Render the entries of one cloud as a stand-alone file.
fn render_new(entries: &Entries) -> Result<String, eyre::Report> {
    let sections = vec![
        ("servers", vec![entries.server.clone()]),
        ("users", vec![entries.user.clone()]),
        ("scopes", entries.scope.iter().cloned().collect()),
        ("contexts", vec![entries.context.clone()]),
    ];
    render(None, &sections, None)
}

/// Translate a cloud into contexts entries (all named `name`).
fn build_entries(name: &str, cloud: &CloudConfig) -> Result<Entries, String> {
    let Value::Object(mut cloud_map) = serde_json::to_value(cloud).map_err(|e| e.to_string())?
    else {
        return Err("the cloud is not a mapping".into());
    };
    let mut auth = match cloud_map.remove("auth") {
        Some(Value::Object(auth)) => auth,
        _ => Map::new(),
    };
    let mut warnings = Vec::new();

    let take = |m: &mut Map<String, Value>, key: &str| m.remove(key).filter(|v| !v.is_null());

    // Server
    let mut server = Map::new();
    let auth_url = take(&mut auth, "auth_url").ok_or("the cloud has no `auth_url`")?;
    server.insert("auth_url".into(), auth_url);
    if let Some(v) = take(&mut auth, "endpoint") {
        server.insert("endpoint".into(), v);
    }
    for key in ["interface", "profile", "cacert", "verify", "auth_cache"] {
        if let Some(v) = take(&mut cloud_map, key) {
            server.insert(key.into(), v);
        }
    }

    // Scope
    let domain_of = |m: &mut Map<String, Value>, id: &str, name: &str| {
        let mut d = Map::new();
        if let Some(v) = take(m, id) {
            d.insert("id".into(), v);
        }
        if let Some(v) = take(m, name) {
            d.insert("name".into(), v);
        }
        (!d.is_empty()).then_some(Value::Object(d))
    };
    let project_domain = domain_of(&mut auth, "project_domain_id", "project_domain_name");
    let project_id = take(&mut auth, "project_id");
    let project_name = take(&mut auth, "project_name");
    let domain_scope = domain_of(&mut auth, "domain_id", "domain_name");
    let system = take(&mut auth, "system_scope");
    let scope = if project_id.is_some() || project_name.is_some() {
        if domain_scope.is_some() || system.is_some() {
            warnings.push("domain/system scope is ignored, the project scope wins".into());
        }
        let mut p = Map::new();
        if let Some(v) = project_id {
            p.insert("id".into(), v);
        }
        if let Some(v) = project_name {
            p.insert("name".into(), v);
        }
        if let Some(v) = project_domain {
            p.insert("domain".into(), v);
        }
        Some(json!({"name": name, "scope": {"project": p}}))
    } else if let Some(d) = domain_scope {
        Some(json!({"name": name, "scope": {"domain": d}}))
    } else {
        system.map(|s| json!({"name": name, "scope": {"system": s}}))
    };

    // User
    let mut user = Map::new();
    if let Some(v) = take(&mut auth, "username") {
        user.insert("user_name".into(), v);
    }
    if let Some(v) = take(&mut auth, "user_id") {
        user.insert("user_id".into(), v);
    }
    if let Some(d) = domain_of(&mut auth, "user_domain_id", "user_domain_name") {
        user.insert("domain".into(), d);
    }
    for key in ["auth_type", "auth_methods"] {
        if let Some(v) = take(&mut cloud_map, key) {
            user.insert(key.into(), v);
        }
    }
    for key in EPHEMERAL {
        if take(&mut auth, key).is_some() {
            warnings.push(format!("short-lived credential `{key}` is not migrated"));
        }
    }
    for (key, value) in auth {
        if !value.is_null() {
            user.insert(key, value);
        }
    }

    // Everything else of the cloud has no place in contexts.yaml.
    let region = take(&mut cloud_map, "region_name");
    let mut dropped: Vec<String> = cloud_map
        .into_iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, _)| k)
        .collect();
    dropped.sort();
    if !dropped.is_empty() {
        warnings.push(format!("settings not migrated: {}", dropped.join(", ")));
    }

    let mut context = Map::new();
    context.insert("server".into(), json!(name));
    context.insert("user".into(), json!(name));
    if scope.is_some() {
        context.insert("scope".into(), json!(name));
    }
    if let Some(r) = region {
        context.insert("region_name".into(), r);
    }

    Ok(Entries {
        server: json!({"name": name, "server": server}),
        user: json!({"name": name, "user": user}),
        scope,
        context: json!({"name": name, "context": context}),
        warnings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cloud(yaml: &str) -> CloudConfig {
        let cfg = config::Config::builder()
            .add_source(config::File::from_str(yaml, config::FileFormat::Yaml))
            .build()
            .unwrap();
        cfg.try_deserialize().unwrap()
    }

    #[test]
    fn password_cloud_roundtrips() {
        let c = cloud(
            "auth:\n  auth_url: https://k/v3\n  username: alice\n  password: secret\n  \
             user_domain_name: Default\n  project_name: admin\n  project_domain_name: Default\n\
             region_name: RegionOne\ncompute_api_version: '2'\n",
        );
        let entries = build_entries("prod", &c).unwrap();
        assert_eq!(entries.warnings.len(), 1);
        let text = render_new(&entries).unwrap();
        let file = ContextsFile::from_yaml(&text).unwrap();
        let resolved = file.resolve("prod").unwrap();
        assert_eq!(resolved.cloud.region_name.as_deref(), Some("RegionOne"));
        let auth = resolved.cloud.auth.unwrap();
        assert_eq!(auth.username.as_deref(), Some("alice"));
        assert_eq!(auth.project_name.as_deref(), Some("admin"));
        assert!(file.contains_secrets());
    }

    #[test]
    fn missing_auth_url_is_refused() {
        assert!(build_entries("x", &cloud("auth:\n  username: a\n")).is_err());
    }

    #[test]
    fn appends_to_existing_file() {
        let existing = "current-context: a\nservers:\n  - name: a\n    server:\n      auth_url: https://a/v3\n\
                        users:\n  - name: a\n    user:\n      user_name: a\n      password: p\n\
                        contexts:\n  - name: a\n    context:\n      server: a\n      user: a\n";
        let c = cloud("auth:\n  auth_url: https://b/v3\n  username: b\n  password: q\n");
        let e = build_entries("b", &c).unwrap();
        let mut sections: Vec<(&str, Vec<Value>)> = SECTIONS
            .iter()
            .map(|s| (*s, section_items(Some(existing), s).unwrap()))
            .collect();
        sections[0].1.push(e.server);
        sections[1].1.push(e.user);
        sections[3].1.push(e.context);
        let out = render(Some(existing), &sections, None).unwrap();
        let file = ContextsFile::from_yaml(&out).unwrap();
        assert_eq!(file.contexts.len(), 2);
        assert_eq!(file.current_context.as_deref(), Some("a"));
    }
}
