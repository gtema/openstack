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

//! `contexts.yaml` configuration.
//!
//! The file follows the structure of the kubernetes `kubeconfig`: independent lists of named
//! `servers`, `users` and `scopes` are combined into a `context` by referencing them by name.
//!
//! ```yaml
//! current-context: prod-admin
//! servers:
//!   - name: prod
//!     server:
//!       auth_url: https://keystone.example.com/v3
//! users:
//!   - name: alice
//!     user:
//!       user_name: alice
//!       domain:
//!         name: Default
//!       auth_helper: /usr/local/bin/osc-pass
//! scopes:
//!   - name: admin
//!     scope:
//!       project:
//!         name: admin
//!         domain:
//!           name: Default
//! contexts:
//!   - name: prod-admin
//!     context:
//!       server: prod
//!       user: alice
//!       scope: admin
//!       region_name: RegionOne  # optional
//! ```
//!
//! The file is **never merged** with `clouds.yaml`/`secure.yaml`.
//!
//! # Secrets
//!
//! Like the kubernetes `kubeconfig` (which embeds client certificates and keys) the file may
//! carry long-lived credentials (`password`, `client_secret`, `application_credential_secret`
//! and secrets declared by the authentication plugin) so that other tools can share the format
//! without any extra machinery. Such a file must be protected like `clouds.yaml`/`secure.yaml`:
//! when it contains secrets and is accessible by group or others a warning is collected in
//! [`ContextsFile::warnings`].
//!
//! Alternatively (or in addition) the secrets can be obtained at the authentication time with the
//! external command referenced by the `auth_helper` attribute of the user (see [`ExternalCmd`]),
//! i.e. `pass`, a keyring or a vault front-end. The file may then be kept free of secrets.
//!
//! Short-lived credentials (`token`, `access_token`, `passcode`, `jwt`) are never supported and
//! are refused with the [`ContextsError`].

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use super::{Auth, CloudConfig, ConfigError};
use crate::auth::auth_helper::ExternalCmd;

/// Errors which may occur when dealing with the `contexts.yaml`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ContextsError {
    /// File cannot be read or parsed.
    #[error("failed to parse contexts: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },

    /// Two entries of the same kind share the name.
    #[error("duplicate {kind} `{name}`")]
    DuplicateName {
        /// Kind of the entry (`server`, `user`, `scope`, `context`).
        kind: &'static str,
        /// Name of the entry.
        name: String,
    },

    /// Context refers to an entry that does not exist.
    #[error("context `{context}` refers to unknown {kind} `{name}`")]
    UnknownReference {
        /// Context name.
        context: String,
        /// Kind of the referenced entry.
        kind: &'static str,
        /// Name of the referenced entry.
        name: String,
    },

    /// Requested context does not exist.
    #[error("context `{0}` not found")]
    ContextNotFound(String),

    /// `current-context` is not set.
    #[error("`current-context` is not set")]
    NoCurrentContext,

    /// User carries a short-lived credential which is not supported.
    #[error("user `{user}` sets short-lived credential `{field}` which is not supported")]
    EphemeralInUser {
        /// User name.
        user: String,
        /// Name of the offending field.
        field: String,
    },

    /// User attribute is also provided by the server or scope.
    #[error("user `{user}` sets `{field}` which is defined by the server or scope")]
    ConflictingAttribute {
        /// User name.
        user: String,
        /// Name of the offending field.
        field: String,
    },

    /// Scope attributes of the user and the scope mix project, domain and system scopes.
    #[error("context `{0}` combines project, domain and system scope")]
    AmbiguousScope(String),

    /// Scope does not identify anything.
    #[error("scope `{0}` must set an `id` or `name` (or the system scope name)")]
    EmptyScope(String),

    /// User sets an attribute that belongs to the server or the context.
    #[error("user `{user}` sets `{field}` which belongs to the server or the context")]
    MisplacedAttribute {
        /// User name.
        user: String,
        /// Name of the offending field.
        field: String,
    },

    /// `auth_helper` is not a usable executable path.
    #[error("user `{user}` has invalid `auth_helper`: {reason}")]
    InvalidAuthHelper {
        /// User name.
        user: String,
        /// What is wrong.
        reason: &'static str,
    },

    /// Schema version of the file is not supported.
    #[error("unsupported contexts file `version: {found}` (supported: {CONTEXTS_SCHEMA_VERSION})")]
    UnsupportedVersion {
        /// Version found in the file.
        found: u32,
    },

    /// Explicitly requested file does not exist.
    #[error("contexts file {} is not found", path.display())]
    FileNotFound {
        /// Path of the file.
        path: PathBuf,
    },

    /// File naming an `auth_helper` can be modified by others.
    #[error(
        "contexts file {} is refused: it names an `auth_helper` to execute and can be modified by \
         group or others (i.e. created with the umask 002), restrict its permissions with \
         `chmod 600`",
        path.display()
    )]
    InsecureAuthHelper {
        /// Path of the file.
        path: PathBuf,
    },

    /// User attributes can not be converted to the authentication data.
    #[error("user `{user}` is invalid: {source}")]
    InvalidUser {
        /// User name.
        user: String,
        /// The source of the error.
        source: config::ConfigError,
    },

    /// Checking the configuration for sensitive data failed.
    #[error(transparent)]
    Config(#[from] ConfigError),
}

/// Current (and only supported) schema version of the `contexts.yaml`.
pub const CONTEXTS_SCHEMA_VERSION: u32 = 1;

/// `contexts.yaml` file.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContextsFile {
    /// Version of the file schema. Missing value is treated as [`CONTEXTS_SCHEMA_VERSION`] (`1`).
    pub version: Option<u32>,
    /// Name of the context used when none is requested explicitly.
    #[serde(rename = "current-context")]
    #[cfg_attr(feature = "json-schema", schemars(rename = "current-context"))]
    pub current_context: Option<String>,
    /// Servers (where to authenticate).
    #[serde(default)]
    pub servers: Vec<NamedServer>,
    /// Users (who authenticates).
    #[serde(default)]
    pub users: Vec<NamedUser>,
    /// Scopes (which project/domain/system to work with).
    #[serde(default)]
    pub scopes: Vec<NamedScope>,
    /// Contexts combining a server, a user and optionally a scope.
    #[serde(default)]
    pub contexts: Vec<NamedContext>,
    /// Non fatal findings of loading the file (i.e. exposed secrets). Also logged. The caller is
    /// expected to show them to the user.
    #[serde(skip)]
    #[cfg_attr(feature = "json-schema", schemars(skip))]
    pub warnings: Vec<String>,
}

/// Named [`Server`].
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct NamedServer {
    /// Server name.
    pub name: String,
    /// Server definition.
    pub server: Server,
}

/// Named [`User`].
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct NamedUser {
    /// User name.
    pub name: String,
    /// User definition.
    pub user: User,
}

/// Named [`Scope`].
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct NamedScope {
    /// Scope name.
    pub name: String,
    /// Scope definition.
    pub scope: Scope,
}

/// Named [`Context`].
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct NamedContext {
    /// Context name.
    pub name: String,
    /// Context definition.
    pub context: Context,
}

/// Where to authenticate.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Server {
    /// Authentication URL.
    pub auth_url: String,
    /// Authentication endpoint type (public/internal/admin).
    pub endpoint: Option<String>,
    /// Interface name to be used for endpoints selection.
    pub interface: Option<String>,
    /// Vendor Profile (by name from clouds-public.yaml).
    pub profile: Option<String>,
    /// Custom CA Certificate (PEM bundle or single DER certificate). See
    /// [`crate::config::CloudConfig::cacert`] for notes on `rustls` being
    /// stricter than OpenSSL.
    pub cacert: Option<String>,
    /// Verify SSL Certificates.
    pub verify: Option<bool>,
    /// Override for authentication caching.
    pub auth_cache: Option<bool>,
}

/// Who authenticates.
///
/// The user is identified by the `user_name` (`username` is accepted as well) or the `user_id`
/// and the optional `domain`. Besides the explicitly listed attributes every other key is an
/// authentication attribute (i.e. `application_credential_id`, `client_id`) with the exception of
/// secrets which are refused. Setting an attribute which is also given by an explicit field
/// (i.e. `user_domain_name` together with `domain`) is an error.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct User {
    /// User name.
    #[serde(alias = "username")]
    pub user_name: Option<String>,
    /// User ID.
    pub user_id: Option<String>,
    /// Domain of the user.
    pub domain: Option<DomainScope>,
    /// Authorization type.
    pub auth_type: Option<String>,
    /// Authorization methods (in the case when auth_type = `multifactor`).
    pub auth_methods: Option<Vec<String>>,
    /// External command providing secrets (see [`ExternalCmd`]).
    pub auth_helper: Option<String>,
    /// Authentication attributes.
    #[serde(flatten)]
    #[cfg_attr(feature = "json-schema", schemars(skip))]
    pub attributes: HashMap<String, config::Value>,
}

/// Which project/domain/system to work with.
///
/// Exactly one kind of scope can be given, so an ambiguous scope cannot be expressed.
#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// Project scope.
    Project(ProjectScope),
    /// Domain scope.
    Domain(DomainScope),
    /// System scope (name of the system, i.e. `all`).
    System(String),
}

/// Project scope.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ProjectScope {
    /// Project ID.
    pub id: Option<String>,
    /// Project name.
    pub name: Option<String>,
    /// Domain of the project.
    pub domain: Option<DomainScope>,
}

/// Domain (of a project, a user or the domain scope).
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DomainScope {
    /// Domain ID.
    pub id: Option<String>,
    /// Domain name.
    pub name: Option<String>,
}

impl Scope {
    /// Whether the scope fails to identify the project, domain or system.
    fn is_empty(&self) -> bool {
        let unnamed = |id: &Option<String>, name: &Option<String>| id.is_none() && name.is_none();
        match self {
            Scope::Project(p) => {
                unnamed(&p.id, &p.name)
                    || p.domain.as_ref().is_some_and(|d| unnamed(&d.id, &d.name))
            }
            Scope::Domain(d) => unnamed(&d.id, &d.name),
            Scope::System(s) => s.is_empty(),
        }
    }

    /// Authentication attributes this scope stands for.
    fn attributes(&self) -> Vec<(&'static str, &String)> {
        let mut attrs = Vec::new();
        fn add<'a>(
            attrs: &mut Vec<(&'static str, &'a String)>,
            key: &'static str,
            value: &'a Option<String>,
        ) {
            if let Some(value) = value {
                attrs.push((key, value));
            }
        }
        match self {
            Scope::Project(project) => {
                add(&mut attrs, "project_id", &project.id);
                add(&mut attrs, "project_name", &project.name);
                if let Some(domain) = &project.domain {
                    add(&mut attrs, "project_domain_id", &domain.id);
                    add(&mut attrs, "project_domain_name", &domain.name);
                }
            }
            Scope::Domain(domain) => {
                add(&mut attrs, "domain_id", &domain.id);
                add(&mut attrs, "domain_name", &domain.name);
            }
            Scope::System(system) => attrs.push(("system_scope", system)),
        }
        attrs
    }
}

/// Combination of the server, user and scope.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Context {
    /// Name of the server.
    pub server: String,
    /// Name of the user.
    pub user: String,
    /// Name of the scope (unscoped when omitted).
    pub scope: Option<String>,
    /// Region to use.
    pub region_name: Option<String>,
}

/// Context resolved into the connection configuration.
#[derive(Clone, Debug)]
pub struct ResolvedContext {
    /// Context name.
    pub name: String,
    /// Connection configuration (secrets are included when the user defines them in the file).
    pub cloud: CloudConfig,
    /// External command that provides the secrets.
    pub auth_helper: Option<String>,
}

impl ResolvedContext {
    /// [`AuthHelper`](crate::auth::auth_helper::AuthHelper) for the user of the context.
    pub fn auth_helper(&self) -> Option<ExternalCmd> {
        self.auth_helper.clone().map(ExternalCmd::strict)
    }
}

/// Human readable list of the directories searched for the `contexts.{yaml,yml}` file.
pub fn contexts_file_search_dirs() -> String {
    let mut dirs: Vec<String> = Vec::new();
    for path in contexts_file_search_paths() {
        if let Some(dir) = path.parent().map(|d| format!("`{}`", d.display()))
            && !dirs.contains(&dir)
        {
            dirs.push(dir);
        }
    }
    dirs.join(", ")
}

/// Candidate locations of the `contexts.{yaml,yml}` file, in search order.
///
/// Unlike `clouds.yaml` the current working directory is deliberately not
/// searched: the file names an `auth_helper` program that is executed, so it
/// must only come from locations controlled by the user or the administrator.
fn contexts_file_search_paths() -> Vec<PathBuf> {
    [
        dirs::config_dir().map(|val| val.join("openstack")),
        dirs::home_dir().map(|val| val.join(".config/openstack")),
        Some(PathBuf::from("/etc/openstack")),
    ]
    .into_iter()
    .flatten()
    .flat_map(|dir| {
        [".yaml", ".yml"]
            .into_iter()
            .map(move |suffix| dir.join(format!("contexts{suffix}")))
    })
    .collect()
}

/// Searches for a `contexts.{yaml,yml}` config file.
///
/// The following locations will be tried in order:
///
/// - `$XDG_CONFIG_HOME/openstack/contexts.{yaml,yml}`
/// - `$XDG_HOME/.config/openstack/contexts.{yaml,yml}`
/// - `/etc/openstack/contexts.{yaml,yml}`
///
/// The current working directory is intentionally not searched, since the file
/// can name an executable (`auth_helper`).
pub fn find_contexts_file() -> Option<PathBuf> {
    contexts_file_search_paths()
        .into_iter()
        .find(|path| path.is_file())
}

impl ContextsFile {
    /// Load and validate the file from the default location (see [`find_contexts_file`]).
    ///
    /// Returns `None` when there is no file.
    pub fn new() -> Result<Option<Self>, ContextsError> {
        find_contexts_file().map(Self::from_path).transpose()
    }

    /// Load and validate the file.
    ///
    /// Fails with [`ContextsError::InsecureAuthHelper`] when the file names an `auth_helper` and
    /// can be modified by group or others (anybody able to edit it could make the tool run an
    /// arbitrary command). When the file holds secrets and is accessible by group or others a
    /// warning is added to the [`warnings`](ContextsFile::warnings) (the caller shows them).
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ContextsError> {
        let path = path.as_ref();
        let mut file = Self::build(config::File::from(path))?;
        if file.users.iter().any(|u| u.user.auth_helper.is_some()) && is_writable_by_others(path) {
            return Err(ContextsError::InsecureAuthHelper {
                path: path.to_path_buf(),
            });
        }
        if file.contains_secrets() && is_accessible_by_others(path) {
            let msg = format!(
                "contexts file {} contains secrets and is accessible by group or others, \
                 restrict its permissions (i.e. `chmod 600`)",
                path.display()
            );
            file.warnings.push(msg);
        }
        Ok(file)
    }

    /// Whether any user carries a long-lived secret.
    pub fn contains_secrets(&self) -> bool {
        self.users.iter().any(|user| {
            build_cloud(&user.name, None, &user.name, &user.user, None)
                .ok()
                .and_then(|cloud| cloud.split_sensitive().ok())
                .is_some_and(|split| {
                    split.secure.auth.is_some() || !split.secure.options.is_empty()
                })
        })
    }

    /// Parse and validate YAML data.
    pub fn from_yaml(data: &str) -> Result<Self, ContextsError> {
        Self::build(config::File::from_str(data, config::FileFormat::Yaml))
    }

    fn build(source: impl config::Source + Send + Sync + 'static) -> Result<Self, ContextsError> {
        let file: Self = config::Config::builder()
            .add_source(source)
            .build()?
            .try_deserialize()?;
        file.validate()?;
        Ok(file)
    }

    /// Names of all contexts.
    pub fn get_available_contexts(&self) -> Vec<String> {
        self.contexts.iter().map(|c| c.name.clone()).collect()
    }

    /// Resolve the context by name.
    pub fn resolve(&self, name: &str) -> Result<ResolvedContext, ContextsError> {
        let context = &self
            .contexts
            .iter()
            .find(|c| c.name == name)
            .ok_or_else(|| ContextsError::ContextNotFound(name.to_string()))?
            .context;

        let server = self
            .servers
            .iter()
            .find(|s| s.name == context.server)
            .ok_or_else(|| ContextsError::UnknownReference {
                context: name.to_string(),
                kind: "server",
                name: context.server.clone(),
            })?;
        let user = self
            .users
            .iter()
            .find(|u| u.name == context.user)
            .ok_or_else(|| ContextsError::UnknownReference {
                context: name.to_string(),
                kind: "user",
                name: context.user.clone(),
            })?;
        let scope = context
            .scope
            .as_ref()
            .map(|scope_name| {
                self.scopes
                    .iter()
                    .find(|s| &s.name == scope_name)
                    .ok_or_else(|| ContextsError::UnknownReference {
                        context: name.to_string(),
                        kind: "scope",
                        name: scope_name.clone(),
                    })
            })
            .transpose()?;

        let mut cloud = build_cloud(
            name,
            Some(&server.server),
            &user.name,
            &user.user,
            scope.map(|s| &s.scope),
        )?;
        if let Some(region_name) = &context.region_name {
            cloud.region_name = Some(region_name.clone());
        }

        Ok(ResolvedContext {
            name: name.to_string(),
            cloud,
            auth_helper: user.user.auth_helper.clone(),
        })
    }

    /// Resolve the `current-context`.
    pub fn resolve_current(&self) -> Result<ResolvedContext, ContextsError> {
        self.resolve(
            self.current_context
                .as_deref()
                .ok_or(ContextsError::NoCurrentContext)?,
        )
    }

    /// Resolve the context selected by the user.
    ///
    /// * `name` - explicitly requested context.
    /// * `implicit` - nothing selects the cloud: use the `current-context` when the file sets
    ///   one (like `kubectl`) and return `None` otherwise.
    pub fn resolve_selected(
        &self,
        name: Option<&str>,
        implicit: bool,
    ) -> Result<Option<ResolvedContext>, ContextsError> {
        match name {
            Some(name) => self.resolve(name).map(Some),
            None if implicit && self.current_context.is_some() => self.resolve_current().map(Some),
            None => Ok(None),
        }
    }

    fn validate(&self) -> Result<(), ContextsError> {
        if let Some(found) = self.version
            && found != CONTEXTS_SCHEMA_VERSION
        {
            return Err(ContextsError::UnsupportedVersion { found });
        }
        check_unique("server", self.servers.iter().map(|x| x.name.as_str()))?;
        check_unique("user", self.users.iter().map(|x| x.name.as_str()))?;
        check_unique("scope", self.scopes.iter().map(|x| x.name.as_str()))?;
        check_unique("context", self.contexts.iter().map(|x| x.name.as_str()))?;

        for scope in &self.scopes {
            if scope.scope.is_empty() {
                return Err(ContextsError::EmptyScope(scope.name.clone()));
            }
        }
        // Users that no context uses must be valid as well (scopes are valid by construction)
        for user in &self.users {
            build_cloud(&user.name, None, &user.name, &user.user, None)?;
        }
        for context in &self.contexts {
            self.resolve(&context.name)?;
        }
        if let Some(current) = &self.current_context
            && !self.contexts.iter().any(|c| &c.name == current)
        {
            return Err(ContextsError::ContextNotFound(current.clone()));
        }
        Ok(())
    }
}

fn check_unique<'a>(
    kind: &'static str,
    names: impl Iterator<Item = &'a str>,
) -> Result<(), ContextsError> {
    let mut seen = HashSet::new();
    for name in names {
        if !seen.insert(name) {
            return Err(ContextsError::DuplicateName {
                kind,
                name: name.to_string(),
            });
        }
    }
    Ok(())
}

/// Whether the file is readable or writable by group or others.
#[cfg(unix)]
fn is_accessible_by_others(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.permissions().mode() & 0o077 != 0)
}

#[cfg(not(unix))]
fn is_accessible_by_others(_path: &Path) -> bool {
    false
}

/// Whether the file can be modified by group or others.
#[cfg(unix)]
fn is_writable_by_others(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).is_ok_and(|m| m.permissions().mode() & 0o022 != 0)
}

#[cfg(not(unix))]
fn is_writable_by_others(_path: &Path) -> bool {
    false
}

/// Fail when the given attribute names address more than one kind of scope.
fn check_scope_keys<'a>(
    name: &str,
    keys: impl IntoIterator<Item = &'a str>,
) -> Result<(), ContextsError> {
    let (mut project, mut domain, mut system) = (false, false, false);
    for key in keys {
        match key {
            "project_id" | "project_name" | "project_domain_id" | "project_domain_name" => {
                project = true
            }
            "domain_id" | "domain_name" => domain = true,
            "system_scope" => system = true,
            _ => {}
        }
    }
    if [project, domain, system].iter().filter(|x| **x).count() > 1 {
        return Err(ContextsError::AmbiguousScope(name.to_string()));
    }
    Ok(())
}

/// Attributes which belong to the server and are therefore not accepted for the user.
const SERVER_LEVEL_ATTRIBUTES: &[&str] = &[
    "region_name",
    "interface",
    "verify",
    "cacert",
    "profile",
    "auth_cache",
    "endpoint",
];

/// Assemble the [`CloudConfig`] and ensure it holds no secrets.
fn build_cloud(
    context_name: &str,
    server: Option<&Server>,
    user_name: &str,
    user: &User,
    scope: Option<&Scope>,
) -> Result<CloudConfig, ContextsError> {
    // Explicit nulls (`key: ~`) mean "not set"
    let mut attributes: HashMap<String, config::Value> = user
        .attributes
        .iter()
        .filter(|(_, v)| !matches!(v.kind, config::ValueKind::Nil))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    if let Some(field) = SERVER_LEVEL_ATTRIBUTES
        .iter()
        .find(|key| attributes.contains_key(**key))
    {
        return Err(ContextsError::MisplacedAttribute {
            user: user_name.to_string(),
            field: (*field).to_string(),
        });
    }
    if let Some(helper) = &user.auth_helper {
        if helper.trim().is_empty() {
            return Err(ContextsError::InvalidAuthHelper {
                user: user_name.to_string(),
                reason: "empty command",
            });
        }
        // Relative names would be looked up in the current directory or in the `PATH`
        if !Path::new(helper).is_absolute() {
            return Err(ContextsError::InvalidAuthHelper {
                user: user_name.to_string(),
                reason: "must be an absolute path of an executable (no arguments)",
            });
        }
        if helper.contains(char::is_whitespace) && !Path::new(helper).exists() {
            return Err(ContextsError::InvalidAuthHelper {
                user: user_name.to_string(),
                reason: "must be a path of an executable without arguments",
            });
        }
    }
    let mut provide = |key: &str, value: &Option<String>| -> Result<(), ContextsError> {
        if let Some(value) = value
            && attributes
                .insert(key.to_string(), config::Value::new(None, value.clone()))
                .is_some()
        {
            return Err(ContextsError::ConflictingAttribute {
                user: user_name.to_string(),
                field: key.to_string(),
            });
        }
        Ok(())
    };
    provide("username", &user.user_name)?;
    provide("user_id", &user.user_id)?;
    if let Some(domain) = &user.domain {
        provide("user_domain_id", &domain.id)?;
        provide("user_domain_name", &domain.name)?;
    }
    if let Some(server) = server {
        provide("auth_url", &Some(server.auth_url.clone()))?;
        provide("endpoint", &server.endpoint)?;
    }
    if let Some(scope) = scope {
        for (key, value) in scope.attributes() {
            provide(key, &Some(value.clone()))?;
        }
    }

    // The user may carry scope attributes itself (e.g. when the context has no scope), so the
    // exclusivity must hold for the merged result and not only for the scope entry.
    check_scope_keys(context_name, attributes.keys().map(String::as_str))?;

    let invalid = |source| ContextsError::InvalidUser {
        user: user_name.to_string(),
        source,
    };
    let auth: Auth = config::Value::new(None, config::ValueKind::Table(attributes.clone()))
        .try_deserialize()
        .map_err(invalid)?;

    // What is not part of the typed `Auth` is intended for the (custom) plugin and travels as
    // the cloud level option.
    let known: HashSet<String> = match serde_json::to_value(&auth).map_err(ConfigError::from)? {
        serde_json::Value::Object(map) => map.into_iter().map(|(k, _)| k).collect(),
        _ => HashSet::new(),
    };
    let options: HashMap<String, config::Value> = attributes
        .into_iter()
        .filter(|(key, _)| !known.contains(key))
        .collect();

    let mut cloud = CloudConfig {
        auth: Some(auth),
        auth_type: user.auth_type.clone(),
        auth_methods: user.auth_methods.clone(),
        name: Some(context_name.to_string()),
        options,
        ..Default::default()
    };
    cloud.interface = Some("public".into());
    if let Some(server) = server {
        cloud.profile = server.profile.clone();
        cloud.cacert = server.cacert.clone();
        cloud.verify = server.verify;
        cloud.auth_cache = server.auth_cache;
        if server.interface.is_some() {
            cloud.interface = server.interface.clone();
        }
    }

    // Long-lived secrets are allowed, short-lived credentials are not.
    match cloud.split_sensitive() {
        Ok(_) => {}
        Err(ConfigError::EphemeralField { field }) => {
            return Err(ContextsError::EphemeralInUser {
                user: user_name.to_string(),
                field,
            });
        }
        Err(err) => return Err(err.into()),
    }
    Ok(cloud)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
version: 1
current-context: prod-admin
servers:
  - name: prod
    server:
      auth_url: https://keystone.example.com/v3
      verify: false
users:
  - name: alice
    user:
      user_name: alice
      domain:
        name: Default
      auth_helper: /usr/local/bin/osc-pass
  - name: robot
    user:
      auth_type: v3applicationcredential
      application_credential_id: abc
scopes:
  - name: admin
    scope:
      project:
        name: admin
        domain:
          name: Default
contexts:
  - name: prod-admin
    context:
      server: prod
      user: alice
      scope: admin
      region_name: RegionOne
  - name: prod-robot
    context:
      server: prod
      user: robot
"#;

    fn err(data: &str) -> ContextsError {
        ContextsFile::from_yaml(data).unwrap_err()
    }

    #[test]
    fn version_defaults_to_current() {
        let file = ContextsFile::from_yaml(&SAMPLE.replace("version: 1\n", "")).unwrap();
        assert_eq!(None, file.version);
        assert_eq!(
            Some(CONTEXTS_SCHEMA_VERSION),
            ContextsFile::from_yaml(SAMPLE).unwrap().version
        );
    }

    #[test]
    fn rejects_unsupported_version() {
        let err = ContextsFile::from_yaml(&SAMPLE.replace("version: 1", "version: 2")).unwrap_err();
        assert!(
            matches!(err, ContextsError::UnsupportedVersion { found: 2 }),
            "{err}"
        );
    }

    #[test]
    fn resolves_current_context() {
        let file = ContextsFile::from_yaml(SAMPLE).unwrap();
        assert_eq!(file.get_available_contexts(), ["prod-admin", "prod-robot"]);
        let ctx = file.resolve_current().unwrap();
        assert_eq!(ctx.name, "prod-admin");
        assert_eq!(ctx.cloud.name.as_deref(), Some("prod-admin"));
        assert_eq!(ctx.cloud.region_name.as_deref(), Some("RegionOne"));
        assert_eq!(ctx.cloud.verify, Some(false));
        assert_eq!(ctx.cloud.interface.as_deref(), Some("public"));
        let auth = ctx.cloud.auth.as_ref().unwrap();
        assert_eq!(
            auth.auth_url.as_deref(),
            Some("https://keystone.example.com/v3")
        );
        assert_eq!(auth.username.as_deref(), Some("alice"));
        assert_eq!(auth.project_name.as_deref(), Some("admin"));
        assert_eq!(auth.project_domain_name.as_deref(), Some("Default"));
        assert!(auth.password.is_none());
        assert_eq!(ctx.auth_helper.as_deref(), Some("/usr/local/bin/osc-pass"));
        assert!(ctx.auth_helper().is_some());
    }

    #[test]
    fn scope_is_optional() {
        let ctx = ContextsFile::from_yaml(SAMPLE)
            .unwrap()
            .resolve("prod-robot")
            .unwrap();
        let auth = ctx.cloud.auth.as_ref().unwrap();
        assert_eq!(
            ctx.cloud.auth_type.as_deref(),
            Some("v3applicationcredential")
        );
        assert_eq!(auth.application_credential_id.as_deref(), Some("abc"));
        assert!(auth.project_name.is_none());
        assert!(ctx.auth_helper().is_none());
    }

    #[test]
    fn unknown_context() {
        let file = ContextsFile::from_yaml(SAMPLE).unwrap();
        assert!(matches!(
            file.resolve("nope"),
            Err(ContextsError::ContextNotFound(_))
        ));
        assert!(matches!(
            ContextsFile::default().resolve_current(),
            Err(ContextsError::NoCurrentContext)
        ));
    }

    #[test]
    fn long_lived_secrets_are_allowed() {
        for field in ["password", "client_secret", "application_credential_secret"] {
            let data = format!(
                "users:\n  - name: u\n    user:\n      username: u\n      {field}: s3cr3t\n"
            );
            let file = ContextsFile::from_yaml(&data).unwrap();
            assert!(file.contains_secrets(), "{field}");
        }
        assert!(!ContextsFile::from_yaml(SAMPLE).unwrap().contains_secrets());
    }

    #[test]
    fn secret_reaches_resolved_context() {
        let data = SAMPLE.replace(
            "user_name: alice",
            "user_name: alice\n      password: s3cr3t",
        );
        let ctx = ContextsFile::from_yaml(&data)
            .unwrap()
            .resolve("prod-admin")
            .unwrap();
        let auth = serde_json::to_value(ctx.cloud.auth.as_ref().unwrap()).unwrap();
        assert_eq!(auth["password"], "s3cr3t");
    }

    #[cfg(unix)]
    #[test]
    fn permissions_check() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contexts.yaml");
        std::fs::write(&path, SAMPLE).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(is_accessible_by_others(&path));
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert!(!is_accessible_by_others(&path));
    }

    #[test]
    fn short_lived_credentials_are_refused() {
        for field in ["token", "access_token", "passcode", "jwt"] {
            let data = format!("users:\n  - name: u\n    user:\n      {field}: abc\n");
            assert!(
                matches!(err(&data), ContextsError::EphemeralInUser { field: f, .. } if f == field),
                "{field}"
            );
        }
    }

    #[test]
    fn unused_users_are_validated() {
        let data = "users:\n  - name: u\n    user:\n      token: x\ncontexts: []\n";
        assert!(matches!(err(data), ContextsError::EphemeralInUser { .. }));
    }

    #[test]
    fn dangling_references() {
        let data = SAMPLE.replace("scope: admin", "scope: missing");
        assert!(matches!(
            err(&data),
            ContextsError::UnknownReference { kind: "scope", .. }
        ));
        let data = SAMPLE.replace("current-context: prod-admin", "current-context: x");
        assert!(matches!(err(&data), ContextsError::ContextNotFound(_)));
    }

    #[test]
    fn duplicate_names() {
        let data = SAMPLE.replace("name: robot", "name: alice");
        assert!(matches!(
            err(&data),
            ContextsError::DuplicateName { kind: "user", .. }
        ));
    }

    #[test]
    fn explicit_user_fields() {
        let file = ContextsFile::from_yaml(&SAMPLE.replace(
            "user_name: alice\n      domain:\n        name: Default\n",
            "user_id: 123\n      domain:\n        id: d1\n        name: Default\n",
        ))
        .unwrap();
        let auth = file.resolve("prod-admin").unwrap().cloud.auth.unwrap();
        assert_eq!(auth.user_id.as_deref(), Some("123"));
        assert_eq!(auth.username, None);
        assert_eq!(auth.user_domain_id.as_deref(), Some("d1"));
        assert_eq!(auth.user_domain_name.as_deref(), Some("Default"));
        // `username` is the same as `user_name`
        let file = ContextsFile::from_yaml(&SAMPLE.replace("user_name: alice", "username: alice"))
            .unwrap();
        let auth = file.resolve("prod-admin").unwrap().cloud.auth.unwrap();
        assert_eq!(auth.username.as_deref(), Some("alice"));
    }

    #[test]
    fn explicit_user_field_conflicting_with_attribute_is_refused() {
        let data = SAMPLE.replace(
            "user_name: alice\n",
            "user_name: alice\n      user_domain_name: Other\n",
        );
        assert!(matches!(
            err(&data),
            ContextsError::ConflictingAttribute { .. }
        ));
    }

    #[test]
    fn user_scope_attribute_mixing_with_scope_is_refused() {
        let data = SAMPLE.replace(
            "user_name: alice\n",
            "user_name: alice\n      domain_name: Default\n",
        );
        assert!(matches!(err(&data), ContextsError::AmbiguousScope(_)));
    }

    #[test]
    fn user_scope_attribute_without_scope_is_allowed() {
        let data = SAMPLE.replace(
            "      application_credential_id: abc\n",
            "      application_credential_id: abc\n      project_name: p\n",
        );
        ContextsFile::from_yaml(&data)
            .unwrap()
            .resolve("prod-robot")
            .unwrap();
    }

    #[test]
    fn mixed_scope_cannot_be_expressed() {
        let data = SAMPLE.replace(
            "      project:\n",
            "      domain:\n        name: Default\n      project:\n",
        );
        assert!(matches!(err(&data), ContextsError::Parse { .. }));
    }

    #[test]
    fn domain_and_system_scopes() {
        for (scope, key, value) in [
            ("domain:\n        id: d1", "domain_id", "d1"),
            ("system: all", "system_scope", "all"),
        ] {
            let start = SAMPLE.find("      project:").unwrap();
            let end = SAMPLE.find("contexts:\n  - name").unwrap();
            let data = format!("{}      {}\n{}", &SAMPLE[..start], scope, &SAMPLE[end..]);
            let ctx = ContextsFile::from_yaml(&data)
                .unwrap()
                .resolve("prod-admin")
                .unwrap();
            let auth = serde_json::to_value(ctx.cloud.auth.as_ref().unwrap()).unwrap();
            assert_eq!(auth[key], value, "{key}");
        }
    }

    #[test]
    fn project_scope_attributes() {
        let ctx = ContextsFile::from_yaml(SAMPLE)
            .unwrap()
            .resolve("prod-admin")
            .unwrap();
        let auth = serde_json::to_value(ctx.cloud.auth.as_ref().unwrap()).unwrap();
        assert_eq!(auth["project_name"], "admin");
        assert_eq!(auth["project_domain_name"], "Default");
    }

    #[test]
    fn user_cannot_override_scope_or_server() {
        let data = SAMPLE.replace(
            "user_name: alice",
            "user_name: alice\n      project_name: other",
        );
        assert!(matches!(
            err(&data),
            ContextsError::ConflictingAttribute { .. }
        ));
    }

    #[test]
    fn unknown_keys_are_refused() {
        let data = SAMPLE.replace("verify: false", "verifyy: false");
        assert!(matches!(err(&data), ContextsError::Parse { .. }));
    }

    #[test]
    fn plugin_attributes_become_options() {
        let data = "servers:\n  - name: s\n    server:\n      auth_url: http://k\n\
                    users:\n  - name: u\n    user:\n      auth_type: custom\n      realm: corp\n\
                    contexts:\n  - name: c\n    context:\n      server: s\n      user: u\n";
        let ctx = ContextsFile::from_yaml(data).unwrap().resolve("c").unwrap();
        assert!(ctx.cloud.options.contains_key("realm"));
    }

    #[test]
    fn search_paths_skip_current_dir() {
        let cwd = std::env::current_dir().unwrap();
        let paths = contexts_file_search_paths();
        assert!(!paths.is_empty());
        assert!(
            paths
                .iter()
                .all(|p| p.parent() != Some(cwd.as_path()) || cwd.ends_with("openstack"))
        );
        assert!(paths.contains(&PathBuf::from("/etc/openstack/contexts.yaml")));
    }

    #[test]
    fn null_attributes_are_not_set() {
        let data = SAMPLE.replace("user_name: alice", "user_name: alice\n      password: ~");
        let file = ContextsFile::from_yaml(&data).unwrap();
        assert!(!file.contains_secrets());
        file.resolve("prod-admin").unwrap();
    }

    #[test]
    fn server_level_attribute_in_user_is_refused() {
        let data = SAMPLE.replace("user_name: alice", "user_name: alice\n      region_name: X");
        assert!(matches!(
            err(&data),
            ContextsError::MisplacedAttribute { field, .. } if field == "region_name"
        ));
    }

    #[test]
    fn invalid_auth_helper_is_refused() {
        for helper in [
            "\"\"",
            "pass show openstack",
            "pass",
            "./helper",
            "bin/helper",
        ] {
            let data = SAMPLE.replace("/usr/local/bin/osc-pass", helper);
            assert!(
                matches!(err(&data), ContextsError::InvalidAuthHelper { .. }),
                "{helper}"
            );
        }
    }

    #[test]
    fn loads_from_file() {
        let dir = tempfile::tempdir().unwrap();
        let yaml = dir.path().join("contexts.yaml");
        std::fs::write(&yaml, SAMPLE).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&yaml, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        let file = ContextsFile::from_path(&yaml).unwrap();
        assert_eq!(file.get_available_contexts(), ["prod-admin", "prod-robot"]);
    }

    #[test]
    fn region_comes_from_the_context() {
        let file = ContextsFile::from_yaml(SAMPLE).unwrap();
        assert_eq!(
            file.resolve("prod-admin")
                .unwrap()
                .cloud
                .region_name
                .as_deref(),
            Some("RegionOne")
        );
        assert!(
            file.resolve("prod-robot")
                .unwrap()
                .cloud
                .region_name
                .is_none()
        );
    }

    #[test]
    fn region_is_not_a_server_attribute() {
        let data = SAMPLE.replace(
            "auth_url: https://keystone.example.com/v3",
            "auth_url: https://keystone.example.com/v3\n      region_name: X",
        );
        assert!(matches!(err(&data), ContextsError::Parse { .. }));
    }

    #[test]
    fn selection() {
        let file = ContextsFile::from_yaml(SAMPLE).unwrap();
        let name = |r: Option<ResolvedContext>| r.map(|x| x.name);
        assert_eq!(
            name(file.resolve_selected(Some("prod-robot"), false).unwrap()).as_deref(),
            Some("prod-robot")
        );
        assert_eq!(
            name(file.resolve_selected(None, true).unwrap()).as_deref(),
            Some("prod-admin")
        );
        assert!(file.resolve_selected(None, false).unwrap().is_none());

        let mut file = file;
        file.current_context = None;
        assert!(file.resolve_selected(None, true).unwrap().is_none());
    }

    #[test]
    fn empty_scope_is_rejected() {
        for scope in [
            "project: {}",
            "project: {domain: {name: D}}",
            "domain: {}",
            "system: ''",
        ] {
            let data = format!(
                "servers: [{{name: s, server: {{auth_url: 'http://x'}}}}]\n\
                 users: [{{name: u, user: {{username: u}}}}]\n\
                 scopes: [{{name: sc, scope: {{{scope}}}}}]\n\
                 contexts: [{{name: c, context: {{server: s, user: u, scope: sc}}}}]\n"
            );
            assert!(
                matches!(err(&data), ContextsError::EmptyScope(_)),
                "{scope}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn writable_file_with_auth_helper_is_refused() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contexts.yaml");
        std::fs::write(
            &path,
            "users: [{name: u, user: {username: u, auth_helper: /bin/true}}]\n",
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o664)).unwrap();
        assert!(matches!(
            ContextsFile::from_path(&path),
            Err(ContextsError::InsecureAuthHelper { .. })
        ));
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(ContextsFile::from_path(&path).is_ok());
        // no helper: writable file is not a problem
        std::fs::write(&path, "users: [{name: u, user: {username: u}}]\n").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o664)).unwrap();
        assert!(ContextsFile::from_path(&path).is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn exposed_secrets_are_reported_as_warnings() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contexts.yaml");
        std::fs::write(
            &path,
            "users: [{name: u, user: {username: u, password: p}}]\n",
        )
        .unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert_eq!(ContextsFile::from_path(&path).unwrap().warnings.len(), 1);
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert!(ContextsFile::from_path(&path).unwrap().warnings.is_empty());
    }

    #[cfg(unix)]
    #[test]
    fn writable_by_others_detected() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("contexts.yaml");
        std::fs::write(&path, "").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o664)).unwrap();
        assert!(is_writable_by_others(&path));
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        assert!(!is_writable_by_others(&path));
    }
}
