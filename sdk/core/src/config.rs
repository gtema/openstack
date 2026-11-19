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

//! OpenStack configuration handling.
//!
//! The connection to a cloud is described with a [`CloudConfig`]. It may be built from:
//!
//! - `clouds.yaml`/`secure.yaml` files, see the [`clouds`] module ([`ConfigFile`])
//! - `contexts.yaml` file with the kubeconfig-like structure, see the [`contexts`] module
//!   ([`ContextsFile`](contexts::ContextsFile))
//! - environment variables with the `OS_` prefix ([`CloudConfig::from_env`])
//! - manually, populating the structure directly
//!
//! The [`selector`] module applies the precedence between those sources in a single place and
//! should be preferred by the applications.
//!
//! # Examples
//!
//! Let the [`ConnectionSelector`](selector::ConnectionSelector) choose the source. The cloud
//! is taken from the `clouds.yaml`, the explicit context from the `contexts.yaml` or, when nothing
//! is requested, the `current-context` of the latter:
//!
//! ```no_run
//! use openstack_sdk_core::config::selector::{ConnectionSelector, SelectionError};
//!
//! let selector = ConnectionSelector {
//!     cloud: Some("devstack".into()),
//!     ..Default::default()
//! };
//! match selector.resolve() {
//!     Ok(resolved) => println!("using {} ({:?})", resolved.name, resolved.source),
//!     Err(SelectionError::NoSelection { clouds, contexts, .. }) => {
//!         println!("select one of {clouds:?} or {contexts:?}")
//!     }
//!     Err(err) => eprintln!("{err}"),
//! }
//! ```
//!
//! Use the `clouds.yaml` directly:
//!
//! ```rust
//! let cfg = openstack_sdk_core::config::ConfigFile::new().unwrap();
//! let profile = cfg.get_cloud_config("devstack").unwrap();
//! ```
//!
//! Use the `contexts.yaml` directly (`None` when the file does not exist):
//!
//! ```no_run
//! use openstack_sdk_core::config::contexts::ContextsFile;
//!
//! if let Some(file) = ContextsFile::new().unwrap() {
//!     let resolved = file.resolve("prod-admin").unwrap();
//!     let cloud_config = resolved.cloud;
//! }
//! ```
//!
//! [`CloudConfig`] object can be constructed directly from environment variables with the `OS_`
//! prefix:
//!
//! ```rust
//! # use openstack_sdk_core::config::CloudConfig;
//! let cfg = CloudConfig::from_env().unwrap();
//! ```
//!
//! or populated by hand:
//!
//! ```rust
//! use openstack_sdk_core::config::{Auth, CloudConfig};
//!
//! let cfg = CloudConfig {
//!     name: Some("manual".into()),
//!     auth: Some(Auth {
//!         auth_url: Some("https://keystone.example.com/v3".into()),
//!         username: Some("alice".into()),
//!         ..Default::default()
//!     }),
//!     ..Default::default()
//! };
//! ```

use std::collections::{HashMap, HashSet};
use std::fmt;

use sha2::{Digest, Sha256};

use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error;

use crate::auth::plugin_sensitive_fields;

pub mod clouds;
pub mod contexts;
pub mod selector;

pub use clouds::{
    CacheConfig, ConfigFile, ConfigFileBuilder, ConfigFileBuilderError, find_clouds_file,
    find_config_files_specified_in_env, find_secure_file, find_vendor_file,
};

/// Errors which may occur when dealing with OpenStack connection
/// configuration data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigError {
    /// Cloud configuration cannot be found in the configuration files.
    #[error("Cloud {0} not found")]
    CloudNotFound(String),

    /// Cloud configuration refers to the undefined profile.
    #[error("Profile {} not found", profile_name)]
    MissingProfile { profile_name: String },

    /// Unknown error.
    #[error("unknown error")]
    Unknown,

    /// Configuration parsing error.
    #[error("failed to deserialize config: {}", source)]
    Parse {
        /// The source of the error.
        #[from]
        source: config::ConfigError,
    },

    /// Config builder error.
    #[error(transparent)]
    ConfigFileBuilder {
        /// The source of the error.
        #[from]
        source: ConfigFileBuilderError,
    },

    /// Config contains a short-lived credential which must not be persisted.
    #[error("config contains ephemeral field `{field}` which cannot be persisted")]
    EphemeralField {
        /// Name of the offending field.
        field: String,
    },

    /// Splitting the config into public and sensitive parts failed.
    #[error("failed to split config: {}", source)]
    Split {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

impl ConfigError {
    /// Construct [ConfigError::Parse] error from [config::ConfigError]
    pub fn parse(source: config::ConfigError) -> Self {
        ConfigError::Parse { source }
    }
}

/// Authentication data.
///
/// Sensitive fields are wrapped into the
/// [SensitiveString](https://docs.rs/secrecy/0.10.3/secrecy/type.SecretString.html) to prevent
/// accidental exposure in logs.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Clone, Default, Deserialize, Serialize)]
pub struct Auth {
    /// Authentication URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    /// Authentication endpoint type (public/internal/admin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// Auth Token.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub token: Option<SecretString>,

    /// Auth User.Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Auth User.ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Auth User.Domain.Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_domain_name: Option<String>,
    /// Auth User.Domain.ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_domain_id: Option<String>,
    /// Auth User password.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub password: Option<SecretString>,

    /// Auth (totp) MFA passcode.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub passcode: Option<SecretString>,

    /// `Domain` scope Domain.ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    /// `Domain` scope Domain.Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// `Project` scope Project.ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// `Project` scope Project.Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    /// `Project` scope Project.Domain.ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_domain_id: Option<String>,
    /// `Project` scope Project.Domain.Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_domain_name: Option<String>,

    /// `Federation` protocol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// `Federation` identity provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    /// `Federation` attribute mapping name to be applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_mapping_name: Option<String>,
    /// OIDC access token.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub access_token: Option<SecretString>,
    /// OIDC access token type (when not access_token).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_type: Option<String>,
    /// OIDC/Discovery endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token_endpoint: Option<String>,
    /// OIDC client ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// OIDC client secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub client_secret: Option<SecretString>,
    /// OIDC discovery document endpoint URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_endpoint: Option<String>,
    /// OIDC scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// JWT token.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub jwt: Option<SecretString>,

    /// `Application Credential` ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_credential_id: Option<String>,
    /// `Application Credential` Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_credential_name: Option<String>,
    /// `Application Credential` Secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_secret_string")]
    #[cfg_attr(feature = "json-schema", schemars(with = "Option<String>"))]
    pub application_credential_secret: Option<SecretString>,

    /// `System scope`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_scope: Option<String>,
}

fn serialize_secret_string<S>(
    secret: &Option<SecretString>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(val) = secret {
        serializer.serialize_str(val.expose_secret())
    } else {
        serializer.serialize_none()
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Auth")
            .field("auth_url", &self.auth_url)
            .field("domain_id", &self.domain_id)
            .field("domain_name", &self.domain_name)
            .field("project_id", &self.project_id)
            .field("project_name", &self.project_name)
            .field("project_domain_id", &self.project_domain_id)
            .field("project_domain_name", &self.project_domain_name)
            .field("username", &self.username)
            .field("user_domain_id", &self.user_domain_id)
            .field("user_domain_name", &self.user_domain_name)
            .field("protocol", &self.protocol)
            .field("identity_provider", &self.identity_provider)
            .field("mapping_name", &self.attribute_mapping_name)
            .field("access_token_type", &self.access_token_type)
            .field("client_id", &self.client_id)
            .field("client_secret", &"<secret>")
            .field("access_token_endpoint", &self.access_token_endpoint)
            .field("discovery_endpoint", &self.discovery_endpoint)
            .field("scope", &self.scope)
            .field("application_credential_id", &self.application_credential_id)
            .field(
                "application_credential_name",
                &self.application_credential_name,
            )
            .field("system_scope", &self.system_scope)
            .finish()
    }
}

/// Configuration object representing a single connection to the concrete cloud.
///
/// Connection to the cloud uses this object.
///
/// Serializing produces a `clouds.yaml` (or `secure.yaml`) cloud entry: unset fields are omitted,
/// and secrets in [`Auth`] are written in clear text. [`name`](CloudConfig::name) is not
/// serialized, since in a config file the name is the key of the entry.
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[derive(Deserialize, Serialize, Default, Clone)]
pub struct CloudConfig {
    /// Authorization data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    /// Authorization type. While it can be enum it would make hard to extend SDK with custom implementations.
    /// Known values: `password`, `v3password`, `multifactor`, `application_credential`, `token`, `v2password`, `jwt`, `websso`, `applicationcredential`, `tokenv3`, `federation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// Authorization methods (in the case when auth_type = `multifactor`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_methods: Option<Vec<String>>,

    /// Vendor Profile (by name from clouds-public.yaml or TBD: URL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// Interface name to be used for endpoints selection.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_public")]
    pub interface: Option<String>,
    /// Region name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,

    /// Alternative connection name which is may be used to provide some meaningful name when
    /// [CloudConfig] is constructed directly without `clouds.yaml` file.
    #[serde(skip_serializing)]
    pub name: Option<String>,

    /// Custom CA Certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// Verify SSL Certificates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify: Option<bool>,

    /// Override for authentication caching.
    ///
    /// If `Some(true)`, enables; if `Some(false)`, disables.
    /// If `None`, falls back to the global `cache.auth` setting in the
    /// `clouds.yaml` ConfigFile (default: `true`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_cache: Option<bool>,

    /// Catch-all for additional configuration fields not explicitly typed.
    /// Any extra YAML keys at this level are captured here.
    #[serde(flatten, serialize_with = "serialize_options")]
    #[cfg_attr(feature = "json-schema", schemars(skip))]
    pub options: HashMap<String, config::Value>,
}

/// Serialize the [`CloudConfig::options`] with sorted keys, since [`config::Value`] does not
/// implement [`Serialize`].
fn serialize_options<S>(
    options: &HashMap<String, config::Value>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut entries: Vec<_> = options.iter().collect();
    entries.sort_by_key(|(key, _)| *key);
    serializer.collect_map(entries.into_iter().map(|(k, v)| (k, ConfigValue(v))))
}

/// [`Serialize`] adapter for the [`config::Value`].
struct ConfigValue<'a>(&'a config::Value);

impl Serialize for ConfigValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0.kind {
            config::ValueKind::Nil => serializer.serialize_none(),
            config::ValueKind::Boolean(val) => serializer.serialize_bool(*val),
            config::ValueKind::I64(val) => serializer.serialize_i64(*val),
            config::ValueKind::I128(val) => serializer.serialize_i128(*val),
            config::ValueKind::U64(val) => serializer.serialize_u64(*val),
            config::ValueKind::U128(val) => serializer.serialize_u128(*val),
            config::ValueKind::Float(val) => serializer.serialize_f64(*val),
            config::ValueKind::String(val) => serializer.serialize_str(val),
            config::ValueKind::Table(table) => {
                let mut entries: Vec<_> = table.iter().collect();
                entries.sort_by_key(|(key, _)| *key);
                serializer.collect_map(entries.into_iter().map(|(k, v)| (k, ConfigValue(v))))
            }
            config::ValueKind::Array(items) => {
                serializer.collect_seq(items.iter().map(ConfigValue))
            }
        }
    }
}

fn default_public() -> Option<String> {
    Some(String::from("public"))
}

impl fmt::Debug for CloudConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CloudConfig")
            .field("auth", &self.auth)
            .field("auth_type", &self.auth_type)
            .finish()
    }
}

/// Get a user authentication hash.
///
/// Hashes a canonical, field-tagged serialization of the identity-relevant
/// config fields with SHA-256. Tagging each field with its name (rather than
/// hashing values back-to-back) keeps e.g. `username=Some("x"), user_id=None`
/// from colliding with `username=None, user_id=Some("x")`, and SHA-256 (unlike
/// `DefaultHasher`) is stable across Rust toolchain versions, so cache files
/// keyed by this hash survive a compiler bump.
pub fn get_config_identity_hash(config: &CloudConfig) -> String {
    fn push_field(canonical: &mut String, name: &str, value: Option<&str>) {
        canonical.push_str(name);
        canonical.push('=');
        if let Some(v) = value {
            canonical.push_str(v);
        }
        canonical.push('\0');
    }

    let mut canonical = String::new();
    if let Some(auth) = &config.auth {
        push_field(&mut canonical, "auth_url", auth.auth_url.as_deref());
        push_field(&mut canonical, "username", auth.username.as_deref());
        push_field(&mut canonical, "user_id", auth.user_id.as_deref());
        push_field(
            &mut canonical,
            "user_domain_id",
            auth.user_domain_id.as_deref(),
        );
        push_field(
            &mut canonical,
            "user_domain_name",
            auth.user_domain_name.as_deref(),
        );
        push_field(
            &mut canonical,
            "identity_provider",
            auth.identity_provider.as_deref(),
        );
        push_field(&mut canonical, "protocol", auth.protocol.as_deref());
        push_field(
            &mut canonical,
            "attribute_mapping_name",
            auth.attribute_mapping_name.as_deref(),
        );
        push_field(
            &mut canonical,
            "access_token_type",
            auth.access_token_type.as_deref(),
        );
        push_field(
            &mut canonical,
            "application_credential_name",
            auth.application_credential_name.as_deref(),
        );
        push_field(
            &mut canonical,
            "application_credential_id",
            auth.application_credential_id.as_deref(),
        );
        push_field(&mut canonical, "system_scope", auth.system_scope.as_deref());
    }
    push_field(&mut canonical, "profile", config.profile.as_deref());

    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    hasher
        .finalize()
        .iter()
        .fold(String::with_capacity(64), |mut s, b| {
            use std::fmt::Write;
            let _ = write!(s, "{:02x}", b);
            s
        })
}

/// CloudConfig struct implementation.
impl CloudConfig {
    /// Construct CloudConfig from environment variables.
    ///
    /// Historically `keystoneauth` python library parses relevant environment variables.
    /// `OpenStackSDK` attempts to structure the cloud configuration introducing an explicit "auth"
    /// object holding the authentication data. Pretty unfortunately this structure gets very
    /// broken: `OS_USER_ID` is placed under the auth, `OS_AUTH_URL` is the auth_url under the
    /// auth, `OS_AUTH_TYPE` is auth_type on the config level and so on.
    ///
    /// To keep this still manageable without keeping at full mapping, this function parses
    /// environment variables with the `OS_` prefix into the [`CloudConfig`] but uses
    /// [`auth`](CloudConfig::auth) property separately populated out of all environment variables
    /// with the same `OS_` prefix.
    pub fn from_env() -> Result<Self, ConfigError> {
        let config_builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("os"))
            .build()?;

        let mut cfg = config_builder
            .try_deserialize::<Self>()
            .map_err(ConfigError::parse)?;

        let auth_builder = config::Config::builder()
            .add_source(config::Environment::with_prefix("os"))
            .build()?;

        cfg.auth = Some(
            auth_builder
                .try_deserialize::<Auth>()
                .map_err(ConfigError::parse)?,
        );

        Ok(cfg)
    }

    /// Update unset CloudConfig with values from the `update` var.
    pub fn update(&mut self, update: &CloudConfig) {
        if let Some(update_auth) = &update.auth {
            let auth = self.auth.get_or_insert(Auth::default());
            if auth.auth_url.is_none() && update_auth.auth_url.is_some() {
                auth.auth_url.clone_from(&update_auth.auth_url);
            }
            if auth.domain_id.is_none() && update_auth.domain_id.is_some() {
                auth.domain_id.clone_from(&update_auth.domain_id);
            }
            if auth.domain_name.is_none() && update_auth.domain_name.is_some() {
                auth.domain_name.clone_from(&update_auth.domain_name);
            }
            if auth.endpoint.is_none() && update_auth.endpoint.is_some() {
                auth.endpoint.clone_from(&update_auth.endpoint);
            }
            if auth.password.is_none() && update_auth.password.is_some() {
                auth.password.clone_from(&update_auth.password);
            }
            if auth.project_id.is_none() && update_auth.project_id.is_some() {
                auth.project_id.clone_from(&update_auth.project_id);
            }
            if auth.project_name.is_none() && update_auth.project_name.is_some() {
                auth.project_name.clone_from(&update_auth.project_name);
            }
            if auth.project_domain_id.is_none() && update_auth.project_domain_id.is_some() {
                auth.project_domain_id
                    .clone_from(&update_auth.project_domain_id);
            }
            if auth.project_domain_name.is_none() && update_auth.project_domain_name.is_some() {
                auth.project_domain_name
                    .clone_from(&update_auth.project_domain_name);
            }
            if auth.token.is_none() && update_auth.token.is_some() {
                auth.token.clone_from(&update_auth.token);
            }
            if auth.username.is_none() && update_auth.username.is_some() {
                auth.username.clone_from(&update_auth.username);
            }
            if auth.user_domain_name.is_none() && update_auth.user_domain_name.is_some() {
                auth.user_domain_name
                    .clone_from(&update_auth.user_domain_name);
            }
            if auth.user_domain_id.is_none() && update_auth.user_domain_id.is_some() {
                auth.user_domain_id.clone_from(&update_auth.user_domain_id);
            }
            if auth.protocol.is_none() && update_auth.protocol.is_some() {
                auth.protocol.clone_from(&update_auth.protocol);
            }
            if auth.identity_provider.is_none() && update_auth.identity_provider.is_some() {
                auth.identity_provider
                    .clone_from(&update_auth.identity_provider);
            }
            if auth.attribute_mapping_name.is_none() && update_auth.attribute_mapping_name.is_some()
            {
                auth.attribute_mapping_name
                    .clone_from(&update_auth.attribute_mapping_name);
            }
            if auth.access_token_type.is_none() && update_auth.access_token_type.is_some() {
                auth.access_token_type
                    .clone_from(&update_auth.access_token_type);
            }
            if auth.access_token.is_none() && update_auth.access_token.is_some() {
                auth.access_token.clone_from(&update_auth.access_token);
            }
            if auth.jwt.is_none() && update_auth.jwt.is_some() {
                auth.jwt.clone_from(&update_auth.jwt);
            }
            if auth.application_credential_id.is_none()
                && update_auth.application_credential_id.is_some()
            {
                auth.application_credential_id
                    .clone_from(&update_auth.application_credential_id);
            }
            if auth.application_credential_name.is_none()
                && update_auth.application_credential_name.is_some()
            {
                auth.application_credential_name
                    .clone_from(&update_auth.application_credential_name);
            }
            if auth.application_credential_secret.is_none()
                && update_auth.application_credential_secret.is_some()
            {
                auth.application_credential_secret
                    .clone_from(&update_auth.application_credential_secret);
            }
            if auth.system_scope.is_none() && update_auth.system_scope.is_some() {
                auth.system_scope.clone_from(&update_auth.system_scope);
            }
        }
        if self.auth_type.is_none() && update.auth_type.is_some() {
            self.auth_type.clone_from(&update.auth_type);
        }
        if self.profile.is_none() && update.profile.is_some() {
            self.profile.clone_from(&update.profile);
        }
        if self.interface.is_none() && update.interface.is_some() {
            self.interface.clone_from(&update.interface);
        }
        if self.region_name.is_none() && update.region_name.is_some() {
            self.region_name.clone_from(&update.region_name);
        }
        if self.cacert.is_none() && update.cacert.is_some() {
            self.cacert.clone_from(&update.cacert);
        }
        if self.verify.is_none() && update.verify.is_some() {
            self.verify.clone_from(&update.verify);
        }
        let current_keys: HashSet<String> = self.options.keys().cloned().collect();
        self.options.extend(
            update
                .options
                .clone()
                .into_iter()
                .filter(|x| !current_keys.contains(&x.0)),
        );
    }

    /// Get sensitive config values.
    ///
    /// This method allows getting list of sensitive values from the config which need to be
    /// censored from logs.
    pub fn get_sensitive_values(&self) -> Vec<&str> {
        let mut res = Vec::new();
        if let Some(auth) = &self.auth {
            if let Some(val) = &auth.password {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.application_credential_secret {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.token {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.passcode {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.access_token {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.client_secret {
                res.push(val.expose_secret());
            }
            if let Some(val) = &auth.jwt {
                res.push(val.expose_secret());
            }
        }
        res
    }

    /// Split the config into the public and the sensitive parts.
    ///
    /// The result is meant to be stored in `clouds.yaml` (public) and `secure.yaml` (secure),
    /// which are merged back on load. A key is sensitive when it is a known secret of the
    /// [`Auth`] (`password`, `client_secret`, `application_credential_secret`) or when the
    /// authentication plugin for the [`auth_type`](CloudConfig::auth_type) declares it as such in
    /// its requirements schema (see [`plugin_sensitive_fields`]). Only plugins linked in are
    /// consulted; use [`CloudConfig::split_sensitive_with`] to provide the knowledge about other
    /// plugins (i.e. wasm).
    ///
    /// # Errors
    ///
    /// Ephemeral credentials (`token`, `access_token`, `passcode`, `jwt`) are short-lived and
    /// must not be persisted. Splitting a config which has any of them set (in `auth` or in
    /// `options`) fails with [`ConfigError::EphemeralField`]: it is up to the caller to remove
    /// them explicitly.
    pub fn split_sensitive(&self) -> Result<SplitConfig, ConfigError> {
        self.split_sensitive_with(&plugin_sensitive_fields(self))
    }

    /// Split the config into the public and the sensitive parts treating additionally the keys
    /// from `extra` as sensitive.
    ///
    /// `extra` is typically the result of
    /// [`sensitive_fields_from_schema`](crate::auth::sensitive_fields_from_schema) of the plugin
    /// requirements. Keys are matched in the `auth` section and in `options`. See
    /// [`CloudConfig::split_sensitive`] for details.
    pub fn split_sensitive_with(
        &self,
        extra: &HashSet<String>,
    ) -> Result<SplitConfig, ConfigError> {
        let is_sensitive =
            |key: &str| PERSISTED_SECRET_AUTH_FIELDS.contains(&key) || extra.contains(key);

        let mut public = self.clone();
        let mut secure = CloudConfig::default();

        if let Some(auth) = &self.auth {
            // Auth serializes its secrets in clear text
            let serde_json::Value::Object(values) = serde_json::to_value(auth)? else {
                unreachable!("Auth is always serialized as an object");
            };
            let mut public_auth = serde_json::Map::new();
            let mut secure_auth = serde_json::Map::new();
            for (key, value) in values.into_iter().filter(|(_, v)| !v.is_null()) {
                if EPHEMERAL_AUTH_FIELDS.contains(&key.as_str()) {
                    return Err(ConfigError::EphemeralField { field: key });
                }
                if is_sensitive(&key) {
                    secure_auth.insert(key, value);
                } else {
                    public_auth.insert(key, value);
                }
            }
            public.auth = Some(serde_json::from_value(serde_json::Value::Object(
                public_auth,
            ))?);
            if !secure_auth.is_empty() {
                secure.auth = Some(serde_json::from_value(serde_json::Value::Object(
                    secure_auth,
                ))?);
            }
        }

        // `options` captures all unknown keys, i.e. also secrets set by custom plugins or
        // arriving from the environment variables.
        public.options.clear();
        for (key, value) in &self.options {
            if EPHEMERAL_AUTH_FIELDS.contains(&key.as_str()) {
                return Err(ConfigError::EphemeralField { field: key.clone() });
            }
            if is_sensitive(key) {
                secure.options.insert(key.clone(), value.clone());
            } else {
                public.options.insert(key.clone(), value.clone());
            }
        }

        Ok(SplitConfig { public, secure })
    }
}

/// [`Auth`] keys which are always sensitive and need to be persisted into the secure config.
const PERSISTED_SECRET_AUTH_FIELDS: &[&str] =
    &["password", "client_secret", "application_credential_secret"];

/// [`Auth`] keys with short-lived secrets: configs holding them are refused for splitting.
const EPHEMERAL_AUTH_FIELDS: &[&str] = &["token", "access_token", "passcode", "jwt"];

/// Result of the [`CloudConfig::split_sensitive`].
#[derive(Debug, Clone, Default)]
pub struct SplitConfig {
    /// Part of the configuration without the sensitive data (`clouds.yaml`).
    pub public: CloudConfig,
    /// Sensitive part of the configuration only (`secure.yaml`).
    pub secure: CloudConfig,
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;
    use std::env;
    use std::io::Write;
    use tempfile::Builder;

    use super::*;

    #[test]
    fn test_sensitive_values() {
        let mut cloud_file = Builder::new().suffix(".yaml").tempfile().unwrap();

        const CLOUD_DATA: &str = r#"
            clouds:
              fake_cloud:
                auth:
                  auth_url: http://fake.com
                  username: override_me
                  password: pwd
                  application_credential_secret: app_cred_secret
                  token: tkn
                  passcode: pcd
        "#;

        write!(cloud_file, "{CLOUD_DATA}").unwrap();

        let cfg = ConfigFile::builder()
            .add_source(cloud_file.path())
            .unwrap()
            .build()
            .unwrap();

        let profile = cfg
            .get_cloud_config("fake_cloud")
            .unwrap()
            .expect("Profile exists");
        assert_eq!(
            vec!["pwd", "app_cred_secret", "tkn", "pcd"],
            profile.get_sensitive_values()
        );
    }

    #[test]
    fn test_cloud_config_from_env() {
        // TODO: Audit that the environment access only happens in single-threaded code.
        unsafe {
            env::set_var("OS_AUTH_URL", "auth_url");
            env::set_var("OS_ENDPOINT", "endpoint");
            env::set_var("OS_TOKEN", "token");

            env::set_var("OS_USERNAME", "username");
            env::set_var("OS_USER_ID", "user_id");
            env::set_var("OS_USER_DOMAIN_NAME", "user_domain_name");
            env::set_var("OS_USER_DOMAIN_ID", "user_domain_id");
            env::set_var("OS_PASSWORD", "password");

            env::set_var("OS_PASSCODE", "passcode");
            env::set_var("OS_DOMAIN_ID", "domain_id");
            env::set_var("OS_DOMAIN_NAME", "domain_name");
            env::set_var("OS_PROJECT_ID", "project_id");
            env::set_var("OS_PROJECT_NAME", "project_name");
            env::set_var("OS_PROJECT_DOMAIN_ID", "project_domain_id");
            env::set_var("OS_PROJECT_DOMAIN_NAME", "project_domain_name");

            env::set_var("OS_PROTOCOL", "protocol");
            env::set_var("OS_IDENTITY_PROVIDER", "identity_provider");
            env::set_var("OS_ACCESS_TOKEN", "access_token");

            env::set_var("OS_APPLICATION_CREDENTIAL_ID", "application_credential_id");
            env::set_var(
                "OS_APPLICATION_CREDENTIAL_NAME",
                "application_credential_name",
            );
            env::set_var(
                "OS_APPLICATION_CREDENTIAL_SECRET",
                "application_credential_secret",
            );

            env::set_var("OS_AUTH_TYPE", "auth_type");
            env::set_var("OS_REGION_NAME", "region_name");

            env::set_var("OS_SYSTEM_SCOPE", "system_scope");
        }

        let cc = CloudConfig::from_env().unwrap();
        let auth = cc.auth.unwrap();
        assert_eq!("auth_url", auth.auth_url.unwrap());
        assert_eq!("endpoint", auth.endpoint.unwrap());
        assert_eq!("token", auth.token.unwrap().expose_secret());
        assert_eq!("username", auth.username.unwrap());
        assert_eq!("user_id", auth.user_id.unwrap());
        assert_eq!("user_domain_name", auth.user_domain_name.unwrap());
        assert_eq!("user_domain_id", auth.user_domain_id.unwrap());
        assert_eq!("password", auth.password.unwrap().expose_secret());
        assert_eq!("project_id", auth.project_id.unwrap());
        assert_eq!("project_name", auth.project_name.unwrap());
        assert_eq!("project_domain_name", auth.project_domain_name.unwrap());
        assert_eq!("project_domain_id", auth.project_domain_id.unwrap());
        assert_eq!(
            "application_credential_id",
            auth.application_credential_id.unwrap()
        );
        assert_eq!(
            "application_credential_name",
            auth.application_credential_name.unwrap()
        );
        assert_eq!(
            "application_credential_secret",
            auth.application_credential_secret.unwrap().expose_secret()
        );
        assert_eq!("system_scope", auth.system_scope.unwrap());
        assert_eq!("auth_type", cc.auth_type.unwrap());
        assert_eq!("region_name", cc.region_name.unwrap());
    }

    /// Fails to compile when a field is added to the [`Auth`]: decide whether the new field is
    /// sensitive/ephemeral and update `PERSISTED_SECRET_AUTH_FIELDS`, `EPHEMERAL_AUTH_FIELDS` and
    /// [`CloudConfig::get_sensitive_values`] accordingly.
    #[test]
    fn test_auth_fields_classified() {
        let Auth {
            auth_url: _,
            endpoint: _,
            token: _,
            username: _,
            user_id: _,
            user_domain_name: _,
            user_domain_id: _,
            password: _,
            passcode: _,
            domain_id: _,
            domain_name: _,
            project_id: _,
            project_name: _,
            project_domain_id: _,
            project_domain_name: _,
            protocol: _,
            identity_provider: _,
            attribute_mapping_name: _,
            access_token: _,
            access_token_type: _,
            access_token_endpoint: _,
            client_id: _,
            client_secret: _,
            discovery_endpoint: _,
            scope: _,
            jwt: _,
            application_credential_id: _,
            application_credential_name: _,
            application_credential_secret: _,
            system_scope: _,
        } = Auth::default();
    }

    fn secret(val: &str) -> Option<SecretString> {
        Some(SecretString::from(val))
    }

    #[test]
    fn test_split_sensitive() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                auth_url: Some("http://foo".into()),
                username: Some("u".into()),
                password: secret("pass"),
                client_secret: secret("cs"),
                application_credential_secret: secret("acs"),
                ..Default::default()
            }),
            auth_type: Some("password".into()),
            region_name: Some("r".into()),
            ..Default::default()
        };
        let split = cfg.split_sensitive_with(&HashSet::new()).unwrap();

        let public = split.public.auth.unwrap();
        assert_eq!(public.auth_url.as_deref(), Some("http://foo"));
        assert_eq!(public.username.as_deref(), Some("u"));
        assert!(public.password.is_none());
        assert!(public.client_secret.is_none());
        assert!(public.application_credential_secret.is_none());
        assert_eq!(split.public.auth_type.as_deref(), Some("password"));
        assert_eq!(split.public.region_name.as_deref(), Some("r"));

        let secure = split.secure.auth.unwrap();
        assert_eq!(secure.password.as_ref().unwrap().expose_secret(), "pass");
        assert_eq!(secure.client_secret.as_ref().unwrap().expose_secret(), "cs");
        assert_eq!(
            secure
                .application_credential_secret
                .as_ref()
                .unwrap()
                .expose_secret(),
            "acs"
        );
        assert!(secure.auth_url.is_none());
        assert!(secure.username.is_none());
        assert!(split.secure.auth_type.is_none());
        assert!(split.secure.region_name.is_none());
    }

    #[test]
    fn test_split_sensitive_refuses_ephemeral() {
        for field in EPHEMERAL_AUTH_FIELDS {
            let mut auth = Auth::default();
            match *field {
                "token" => auth.token = secret("x"),
                "access_token" => auth.access_token = secret("x"),
                "passcode" => auth.passcode = secret("x"),
                "jwt" => auth.jwt = secret("x"),
                other => panic!("unhandled ephemeral field {other}"),
            }
            let cfg = CloudConfig {
                auth: Some(auth),
                ..Default::default()
            };
            assert!(matches!(
                cfg.split_sensitive_with(&HashSet::new()),
                Err(ConfigError::EphemeralField { field: f }) if f == *field
            ));
        }
        let cfg = CloudConfig {
            options: HashMap::from([("token".to_string(), ::config::Value::from("t"))]),
            ..Default::default()
        };
        assert!(matches!(
            cfg.split_sensitive_with(&HashSet::new()),
            Err(ConfigError::EphemeralField { .. })
        ));
    }

    #[test]
    fn test_split_sensitive_extra_and_options() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                // plugin declares a normally public field as secret
                scope: Some("s".into()),
                username: Some("u".into()),
                ..Default::default()
            }),
            options: HashMap::from([
                ("custom_secret".to_string(), ::config::Value::from("x")),
                ("custom_public".to_string(), ::config::Value::from("y")),
                // arriving from the environment
                ("password".to_string(), ::config::Value::from("p")),
            ]),
            ..Default::default()
        };
        let extra = HashSet::from(["custom_secret".to_string(), "scope".to_string()]);
        let split = cfg.split_sensitive_with(&extra).unwrap();

        assert_eq!(
            split.public.options.keys().collect::<Vec<_>>(),
            ["custom_public"]
        );
        let mut secure_keys: Vec<_> = split.secure.options.keys().collect();
        secure_keys.sort();
        assert_eq!(secure_keys, ["custom_secret", "password"]);

        let public_auth = split.public.auth.unwrap();
        assert!(public_auth.scope.is_none());
        assert_eq!(public_auth.username.as_deref(), Some("u"));
        assert_eq!(split.secure.auth.unwrap().scope.as_deref(), Some("s"));
    }

    #[test]
    fn test_split_sensitive_no_secrets() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                username: Some("u".into()),
                ..Default::default()
            }),
            ..Default::default()
        };
        let split = cfg.split_sensitive_with(&HashSet::new()).unwrap();
        assert!(split.secure.auth.is_none());
        assert!(split.secure.options.is_empty());
        assert_eq!(split.public.auth.unwrap().username.as_deref(), Some("u"));
        assert!(
            CloudConfig::default()
                .split_sensitive()
                .unwrap()
                .public
                .auth
                .is_none()
        );
    }

    #[test]
    fn test_split_sensitive_merge_roundtrip() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                auth_url: Some("http://foo".into()),
                username: Some("u".into()),
                password: secret("pass"),
                ..Default::default()
            }),
            auth_type: Some("password".into()),
            ..Default::default()
        };
        let split = cfg.split_sensitive_with(&HashSet::new()).unwrap();

        let mut merged = split.public;
        merged.update(&split.secure);
        let auth = merged.auth.unwrap();
        assert_eq!(auth.auth_url.as_deref(), Some("http://foo"));
        assert_eq!(auth.username.as_deref(), Some("u"));
        assert_eq!(auth.password.unwrap().expose_secret(), "pass");
        assert_eq!(merged.auth_type.as_deref(), Some("password"));
    }

    #[test]
    fn test_get_sensitive_values_all_secrets() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                client_secret: secret("cs"),
                jwt: secret("jwt"),
                ..Default::default()
            }),
            ..Default::default()
        };
        let vals = cfg.get_sensitive_values();
        assert!(vals.contains(&"cs"));
        assert!(vals.contains(&"jwt"));
    }

    #[test]
    fn test_serialize_omits_unset_fields() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                auth_url: Some("http://foo".into()),
                application_credential_secret: secret("acs"),
                ..Default::default()
            }),
            auth_type: Some("v3applicationcredential".into()),
            name: Some("runtime_name".into()),
            verify: Some(false),
            options: HashMap::from([
                ("z_opt".to_string(), ::config::Value::from(1)),
                ("a_opt".to_string(), ::config::Value::from("x")),
            ]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&cfg).unwrap(),
            r#"{"auth":{"auth_url":"http://foo","application_credential_secret":"acs"},"auth_type":"v3applicationcredential","verify":false,"a_opt":"x","z_opt":1}"#
        );
        assert_eq!(
            serde_json::to_string(&CloudConfig::default()).unwrap(),
            "{}"
        );
    }

    #[test]
    fn test_serialize_nested_options() {
        let cfg = CloudConfig {
            options: HashMap::from([
                (
                    "table".to_string(),
                    ::config::Value::from(::config::Map::from([
                        ("b".to_string(), ::config::Value::from(true)),
                        ("a".to_string(), ::config::Value::from(1.5)),
                    ])),
                ),
                (
                    "list".to_string(),
                    ::config::Value::from(vec![
                        ::config::Value::from("x"),
                        ::config::Value::from(2),
                    ]),
                ),
            ]),
            ..Default::default()
        };
        assert_eq!(
            serde_json::to_string(&cfg).unwrap(),
            r#"{"list":["x",2],"table":{"a":1.5,"b":true}}"#
        );
    }

    /// The split config written into the clouds and secure files loads back into the original.
    #[test]
    fn test_split_sensitive_serialize_roundtrip() {
        let cfg = CloudConfig {
            auth: Some(Auth {
                auth_url: Some("http://foo".into()),
                username: Some("u".into()),
                password: secret("pass-1"),
                ..Default::default()
            }),
            auth_type: Some("password".into()),
            region_name: Some("r".into()),
            options: HashMap::from([
                ("custom_secret".to_string(), ::config::Value::from("cs-1")),
                ("custom_port".to_string(), ::config::Value::from(8080)),
            ]),
            ..Default::default()
        };
        let extra = HashSet::from(["custom_secret".to_string()]);
        let split = cfg.split_sensitive_with(&extra).unwrap();

        // JSON is valid YAML: no YAML serializer is needed to write the files
        let mut cloud_file = Builder::new().suffix(".yaml").tempfile().unwrap();
        let mut secure_file = Builder::new().suffix(".yaml").tempfile().unwrap();
        let public = serde_json::to_string(&split.public).unwrap();
        let secure = serde_json::to_string(&split.secure).unwrap();
        assert!(!public.contains("pass-1") && !public.contains("cs-1"));
        write!(cloud_file, r#"{{"clouds": {{"c": {public}}}}}"#).unwrap();
        write!(secure_file, r#"{{"clouds": {{"c": {secure}}}}}"#).unwrap();

        let loaded = ConfigFile::builder()
            .add_source(cloud_file.path())
            .unwrap()
            .add_source(secure_file.path())
            .unwrap()
            .build()
            .unwrap()
            .get_cloud_config("c")
            .unwrap()
            .expect("cloud exists");

        let auth = loaded.auth.unwrap();
        assert_eq!(auth.auth_url.as_deref(), Some("http://foo"));
        assert_eq!(auth.username.as_deref(), Some("u"));
        assert_eq!(auth.password.unwrap().expose_secret(), "pass-1");
        assert_eq!(loaded.auth_type.as_deref(), Some("password"));
        assert_eq!(loaded.region_name.as_deref(), Some("r"));
        assert_eq!(
            loaded.options["custom_secret"]
                .clone()
                .into_string()
                .unwrap(),
            "cs-1"
        );
        assert_eq!(
            loaded.options["custom_port"].clone().into_int().unwrap(),
            8080
        );
    }
}
