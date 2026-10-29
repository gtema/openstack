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
//!       region_name: RegionOne
//! users:
//!   - name: alice
//!     user:
//!       username: alice
//!       user_domain_name: Default
//!       auth_helper: /usr/local/bin/osc-pass
//! scopes:
//!   - name: admin
//!     scope:
//!       project_name: admin
//!       project_domain_name: Default
//! contexts:
//!   - name: prod-admin
//!     context:
//!       server: prod
//!       user: alice
//!       scope: admin
//! ```
//!
//! The file is **never merged** with `clouds.yaml`/`secure.yaml`.
//!
//! # Secrets
//!
//! The file must not contain any secret. Long-lived credentials (`password`, `client_secret`,
//! `application_credential_secret` and secrets declared by the authentication plugin) and the
//! short-lived ones (`token`, `access_token`, `passcode`, `jwt`) are refused with the
//! [`ContextsError`]. Secrets are obtained at the authentication time with the external command
//! referenced by the `auth_helper` attribute of the user (see [`ExternalCmd`]), i.e. `pass`,
//! a keyring or a vault front-end. Since the file carries no secret there is no need to restrict
//! its permissions or to exclude it from backups and VCS.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;
use thiserror::Error;

use super::{Auth, CloudConfig, ConfigError, get_config_file_search_paths};
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

    /// User carries a secret which must be provided with the `auth_helper` instead.
    #[error(
        "user `{user}` sets secret `{field}`: contexts file must not contain secrets, use `auth_helper`"
    )]
    SecretInUser {
        /// User name.
        user: String,
        /// Name of the offending field.
        field: String,
    },

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

    /// Scope mixes project, domain and system scopes.
    #[error("scope `{0}` combines project, domain and system scope")]
    AmbiguousScope(String),

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

/// `contexts.yaml` file.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContextsFile {
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
    /// Region name.
    pub region_name: Option<String>,
    /// Vendor Profile (by name from clouds-public.yaml).
    pub profile: Option<String>,
    /// Custom CA Certificate.
    pub cacert: Option<String>,
    /// Verify SSL Certificates.
    pub verify: Option<bool>,
    /// Override for authentication caching.
    pub auth_cache: Option<bool>,
}

/// Who authenticates.
///
/// Besides the explicitly listed attributes every other key is an authentication attribute (i.e.
/// `username`, `user_domain_name`, `application_credential_id`, `client_id`) with the exception of
/// secrets which are refused.
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct User {
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
#[derive(Clone, Debug, Default, Deserialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct Scope {
    /// `Project` scope Project.ID.
    pub project_id: Option<String>,
    /// `Project` scope Project.Name.
    pub project_name: Option<String>,
    /// `Project` scope Project.Domain.ID.
    pub project_domain_id: Option<String>,
    /// `Project` scope Project.Domain.Name.
    pub project_domain_name: Option<String>,
    /// `Domain` scope Domain.ID.
    pub domain_id: Option<String>,
    /// `Domain` scope Domain.Name.
    pub domain_name: Option<String>,
    /// `System scope`.
    pub system_scope: Option<String>,
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
}

/// Context resolved into the connection configuration.
#[derive(Clone, Debug)]
pub struct ResolvedContext {
    /// Context name.
    pub name: String,
    /// Connection configuration. It never contains secrets.
    pub cloud: CloudConfig,
    /// External command that provides the secrets.
    pub auth_helper: Option<String>,
}

impl ResolvedContext {
    /// [`AuthHelper`](crate::auth::auth_helper::AuthHelper) for the user of the context.
    pub fn auth_helper(&self) -> Option<ExternalCmd> {
        self.auth_helper.clone().map(ExternalCmd::new)
    }
}

/// Searches for a `contexts.{yaml,yml,json}` config file.
///
/// The following locations will be tried in order:
///
/// - `./contexts.{yaml,yml,json}` (current working directory)
/// - `$XDG_CONFIG_HOME/openstack/contexts.{yaml,yml,json}`
/// - `$XDG_HOME/.config/openstack/contexts.{yaml,yml,json}`
/// - `/etc/openstack/contexts.{yaml,yml,json}`
pub fn find_contexts_file() -> Option<PathBuf> {
    get_config_file_search_paths("contexts")
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
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ContextsError> {
        Self::build(config::File::from(path.as_ref()))
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

        Ok(ResolvedContext {
            name: name.to_string(),
            cloud: build_cloud(
                name,
                Some(&server.server),
                &user.name,
                &user.user,
                scope.map(|s| (s.name.as_str(), &s.scope)),
            )?,
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

    fn validate(&self) -> Result<(), ContextsError> {
        check_unique("server", self.servers.iter().map(|x| x.name.as_str()))?;
        check_unique("user", self.users.iter().map(|x| x.name.as_str()))?;
        check_unique("scope", self.scopes.iter().map(|x| x.name.as_str()))?;
        check_unique("context", self.contexts.iter().map(|x| x.name.as_str()))?;

        // Users and scopes that no context uses must be valid as well
        for user in &self.users {
            build_cloud(&user.name, None, &user.name, &user.user, None)?;
        }
        for scope in &self.scopes {
            check_scope(&scope.name, &scope.scope)?;
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

fn check_scope(name: &str, scope: &Scope) -> Result<(), ContextsError> {
    let project = [
        &scope.project_id,
        &scope.project_name,
        &scope.project_domain_id,
        &scope.project_domain_name,
    ]
    .iter()
    .any(|x| x.is_some());
    let domain = scope.domain_id.is_some() || scope.domain_name.is_some();
    let system = scope.system_scope.is_some();
    if [project, domain, system].iter().filter(|x| **x).count() > 1 {
        return Err(ContextsError::AmbiguousScope(name.to_string()));
    }
    Ok(())
}

/// Assemble the [`CloudConfig`] and ensure it holds no secrets.
fn build_cloud(
    context_name: &str,
    server: Option<&Server>,
    user_name: &str,
    user: &User,
    scope: Option<(&str, &Scope)>,
) -> Result<CloudConfig, ContextsError> {
    let mut attributes = user.attributes.clone();
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
    if let Some(server) = server {
        provide("auth_url", &Some(server.auth_url.clone()))?;
        provide("endpoint", &server.endpoint)?;
    }
    if let Some((scope_name, scope)) = scope {
        check_scope(scope_name, scope)?;
        provide("project_id", &scope.project_id)?;
        provide("project_name", &scope.project_name)?;
        provide("project_domain_id", &scope.project_domain_id)?;
        provide("project_domain_name", &scope.project_domain_name)?;
        provide("domain_id", &scope.domain_id)?;
        provide("domain_name", &scope.domain_name)?;
        provide("system_scope", &scope.system_scope)?;
    }

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
        cloud.region_name = server.region_name.clone();
        cloud.cacert = server.cacert.clone();
        cloud.verify = server.verify;
        cloud.auth_cache = server.auth_cache;
        if server.interface.is_some() {
            cloud.interface = server.interface.clone();
        }
    }

    match cloud.split_sensitive() {
        Ok(split) => {
            let secret = split
                .secure
                .auth
                .as_ref()
                .and_then(|auth| serde_json::to_value(auth).ok())
                .and_then(|val| val.as_object().and_then(|o| o.keys().next().cloned()))
                .or_else(|| split.secure.options.keys().next().cloned());
            if let Some(field) = secret {
                return Err(ContextsError::SecretInUser {
                    user: user_name.to_string(),
                    field,
                });
            }
        }
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
current-context: prod-admin
servers:
  - name: prod
    server:
      auth_url: https://keystone.example.com/v3
      region_name: RegionOne
      verify: false
users:
  - name: alice
    user:
      username: alice
      user_domain_name: Default
      auth_helper: /usr/local/bin/osc-pass
  - name: robot
    user:
      auth_type: v3applicationcredential
      application_credential_id: abc
scopes:
  - name: admin
    scope:
      project_name: admin
      project_domain_name: Default
contexts:
  - name: prod-admin
    context:
      server: prod
      user: alice
      scope: admin
  - name: prod-robot
    context:
      server: prod
      user: robot
"#;

    fn err(data: &str) -> ContextsError {
        ContextsFile::from_yaml(data).unwrap_err()
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
    fn secrets_are_refused() {
        for field in ["password", "client_secret", "application_credential_secret"] {
            let data = format!(
                "users:\n  - name: u\n    user:\n      username: u\n      {field}: s3cr3t\n"
            );
            assert!(
                matches!(err(&data), ContextsError::SecretInUser { field: f, .. } if f == field),
                "{field}"
            );
        }
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
        let data = "users:\n  - name: u\n    user:\n      password: x\ncontexts: []\n";
        assert!(matches!(err(data), ContextsError::SecretInUser { .. }));
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
    fn mixed_scope_is_refused() {
        let data = SAMPLE.replace(
            "project_domain_name: Default",
            "project_domain_name: Default\n      domain_name: Default",
        );
        assert!(matches!(err(&data), ContextsError::AmbiguousScope(_)));
    }

    #[test]
    fn user_cannot_override_scope_or_server() {
        let data = SAMPLE.replace(
            "username: alice",
            "username: alice\n      project_name: other",
        );
        assert!(matches!(
            err(&data),
            ContextsError::ConflictingAttribute { .. }
        ));
    }

    #[test]
    fn unknown_keys_are_refused() {
        let data = SAMPLE.replace("region_name: RegionOne", "region: RegionOne");
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
}
