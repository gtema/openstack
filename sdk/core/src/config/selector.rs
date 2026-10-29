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

//! Selection of the connection configuration.
//!
//! The connection may be described in the `clouds.yaml`/`secure.yaml`, in the `contexts.yaml` or
//! in the environment variables. [`ConnectionSelector`] holds what the user asked for and
//! [`ConnectionSelector::resolve`] applies the precedence in a single place:
//!
//! 1. explicit context (`context`)
//! 2. environment variables (`from_env`)
//! 3. explicit cloud (`cloud`)
//! 4. `current-context` of the `contexts.yaml` when nothing above is given and nothing else
//!    selects or overrides the connection, including explicit `clouds.yaml`/`secure.yaml`
//!    files (implicit context, same as `kubectl`; an unusable file is ignored with a warning)
//! 5. nothing: [`SelectionError::NoSelection`] with the clouds the caller may let the user
//!    choose from
//!
//! The `region_name` is applied to the selected configuration.
//!
//! # Examples
//!
//! Explicit cloud from the `clouds.yaml`:
//!
//! ```no_run
//! use openstack_sdk_core::config::selector::ConnectionSelector;
//!
//! let resolved = ConnectionSelector {
//!     cloud: Some("devstack".into()),
//!     ..Default::default()
//! }
//! .resolve()
//! .unwrap();
//! let cloud_config = resolved.cloud;
//! ```
//!
//! Explicit context from the `contexts.yaml`, with the secrets requested from the `auth_helper`
//! of the user:
//!
//! ```no_run
//! # use openstack_sdk_core::config::selector::ConnectionSelector;
//! let resolved = ConnectionSelector {
//!     context: Some("prod-admin".into()),
//!     region_name: Some("RegionTwo".into()),
//!     ..Default::default()
//! }
//! .resolve()
//! .unwrap();
//! // `ExternalCmd` to be used as the `AuthHelper` when connecting
//! let helper = resolved.auth_helper();
//! ```
//!
//! Environment variables (`OS_*`):
//!
//! ```no_run
//! # use openstack_sdk_core::config::selector::ConnectionSelector;
//! let resolved = ConnectionSelector {
//!     from_env: true,
//!     env_cloud_name: Some("my-env".into()),
//!     ..Default::default()
//! }
//! .resolve()
//! .unwrap();
//! ```
//!
//! Nothing requested: `current-context` is used when set, otherwise the caller gets the names
//! to choose from:
//!
//! ```no_run
//! use openstack_sdk_core::config::selector::{ConnectionSelector, SelectionError};
//!
//! match ConnectionSelector::default().resolve() {
//!     Ok(resolved) => println!("{}", resolved.name),
//!     Err(SelectionError::NoSelection { clouds, contexts, .. }) => {
//!         println!("choose one of {clouds:?} or contexts {contexts:?}")
//!     }
//!     Err(err) => return Err(err.into()),
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::path::PathBuf;

use std::collections::HashMap;
use thiserror::Error;

use tracing::debug;

use super::clouds::load_trusted_profiles;
use super::contexts::{ContextsError, ContextsFile, contexts_file_search_dirs};
use super::{CloudConfig, ConfigError, ConfigFile};
use crate::auth::auth_helper::ExternalCmd;

/// Where the selected connection comes from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionSource {
    /// `clouds.yaml` (and `secure.yaml`).
    Clouds,
    /// `contexts.yaml`.
    Contexts,
    /// Environment variables.
    Env,
}

/// Errors of the connection selection.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SelectionError {
    /// Cloud configuration error.
    #[error(transparent)]
    Config(#[from] ConfigError),

    /// `contexts.yaml` error.
    #[error(transparent)]
    Contexts(#[from] ContextsError),

    /// Context requested but there is no `contexts.yaml`.
    #[error("`contexts.yaml` file is not found in {}", contexts_file_search_dirs())]
    ContextsFileNotFound,

    /// Requested cloud is not defined.
    #[error("cloud connection `{0}` cannot be found")]
    ConnectionNotFound(String),

    /// Nothing selects the connection.
    #[error("connection is not selected")]
    NoSelection {
        /// Sorted names of the clouds available in the `clouds.yaml`.
        clouds: Vec<String>,
        /// Names of the contexts available in the `contexts.yaml`.
        contexts: Vec<String>,
        /// Problems met while looking for the connections (i.e. unusable `contexts.yaml`).
        warnings: Vec<String>,
    },
}

/// What the user asked for.
#[derive(Clone, Debug, Default)]
pub struct ConnectionSelector {
    /// Name of the context (explicit).
    pub context: Option<String>,
    /// Name of the cloud in the `clouds.yaml`.
    pub cloud: Option<String>,
    /// Build the configuration from the environment variables.
    pub from_env: bool,
    /// Name of the connection built from the environment variables.
    pub env_cloud_name: Option<String>,
    /// Project is overridden by the user (disables the implicit context).
    pub project_override: bool,
    /// Region overriding the one of the selected connection (disables the implicit context).
    ///
    /// Explicit `clouds.yaml`/`secure.yaml` files (below) disable it as well.
    pub region_name: Option<String>,
    /// Never use the `current-context` unless a context is requested explicitly.
    pub skip_implicit_context: bool,
    /// Path of the `contexts.yaml` to use instead of searching the default locations (same as
    /// `--kubeconfig` of `kubectl`). A missing file is an error. It does not prevent the use of
    /// the `current-context`.
    pub contexts_file: Option<PathBuf>,
    /// Additional `clouds.yaml` file.
    pub config_file: Option<PathBuf>,
    /// Additional `secure.yaml` file.
    pub secure_file: Option<PathBuf>,
}

/// Selected connection.
#[derive(Clone, Debug)]
pub struct ResolvedConnection {
    /// Name of the connection (cloud, context or the env connection name).
    pub name: String,
    /// Connection configuration.
    pub cloud: CloudConfig,
    /// Where it comes from.
    pub source: ConnectionSource,
    /// External command providing the secrets (contexts only).
    pub auth_helper: Option<String>,
    /// Problems which did not prevent the selection (i.e. unusable `contexts.yaml` which was
    /// ignored, secrets exposed by file permissions). The caller is expected to show them to the
    /// user: the log level may hide the log records.
    pub warnings: Vec<String>,
}

impl ResolvedConnection {
    /// [`AuthHelper`](crate::auth::auth_helper::AuthHelper) configured for the connection.
    pub fn auth_helper(&self) -> Option<ExternalCmd> {
        self.auth_helper.clone().map(ExternalCmd::strict)
    }
}

impl ConnectionSelector {
    /// Whether nothing selects or overrides the connection explicitly.
    fn is_implicit(&self) -> bool {
        self.context.is_none()
            && self.cloud.is_none()
            && !self.from_env
            && self.env_cloud_name.is_none()
            && !self.project_override
            && self.region_name.is_none()
            && self.config_file.is_none()
            && self.secure_file.is_none()
            && !self.skip_implicit_context
    }

    /// Select the connection.
    pub fn resolve(&self) -> Result<ResolvedConnection, SelectionError> {
        let load_contexts = || match &self.contexts_file {
            Some(path) if !path.is_file() => {
                Err(ContextsError::FileNotFound { path: path.clone() })
            }
            Some(path) => ContextsFile::from_path(path).map(Some),
            None => ContextsFile::new(),
        };
        self.resolve_with(load_contexts, load_trusted_profiles, || {
            ConfigFile::new_with_user_specified_configs(
                self.config_file.as_ref(),
                self.secure_file.as_ref(),
            )
        })
    }

    /// Selection with the configuration sources injected (they are loaded only when needed).
    fn resolve_with(
        &self,
        load_contexts: impl FnOnce() -> Result<Option<ContextsFile>, ContextsError>,
        load_profiles: impl FnOnce() -> Result<HashMap<String, CloudConfig>, ConfigError>,
        load_clouds: impl FnOnce() -> Result<ConfigFile, ConfigError>,
    ) -> Result<ResolvedConnection, SelectionError> {
        let mut resolved = self.select_with(load_contexts, load_profiles, load_clouds)?;
        if let Some(region_name) = &self.region_name {
            resolved.cloud.region_name = Some(region_name.clone());
        }
        Ok(resolved)
    }

    fn select_with(
        &self,
        load_contexts: impl FnOnce() -> Result<Option<ContextsFile>, ContextsError>,
        load_profiles: impl FnOnce() -> Result<HashMap<String, CloudConfig>, ConfigError>,
        load_clouds: impl FnOnce() -> Result<ConfigFile, ConfigError>,
    ) -> Result<ResolvedConnection, SelectionError> {
        let implicit = self.is_implicit();
        let mut load_contexts = Some(load_contexts);
        let mut warnings = Vec::new();
        let mut contexts_names = Vec::new();
        if let Some(load) = load_contexts.take_if(|_| self.context.is_some() || implicit) {
            // A broken file must not prevent using the `clouds.yaml` unless a context was asked
            let found = load().and_then(|file| {
                file.map(|file| {
                    contexts_names = file.get_available_contexts();
                    warnings.extend(file.warnings.iter().cloned());
                    file.resolve_selected(self.context.as_deref(), implicit)
                })
                .transpose()
                .map(Option::flatten)
            });
            match found {
                Ok(Some(mut ctx)) => {
                    debug!("Using the contexts.yaml for the cloud connection");
                    if let Some(profile_name) = ctx.cloud.profile.clone() {
                        // Vendor profiles are the only part of the `clouds*.yaml` files which
                        // is used with a context. They are read from the trusted locations only
                        // and must not weaken the connection security defined by the context.
                        match load_profiles() {
                            Ok(profiles) => match profiles.get(&profile_name) {
                                Some(profile) => {
                                    let (verify, cacert, options) = (
                                        ctx.cloud.verify,
                                        ctx.cloud.cacert.clone(),
                                        ctx.cloud.options.clone(),
                                    );
                                    ctx.cloud.update(profile);
                                    ctx.cloud.verify = verify;
                                    ctx.cloud.cacert = cacert;
                                    ctx.cloud.options = options;
                                }
                                None => {
                                    warnings.push(format!(
                                        "Profile `{profile_name}` of the context `{}` is not found in the clouds-public.yaml",
                                        ctx.name
                                    ));
                                }
                            },
                            Err(err) => {
                                warnings.push(format!(
                                    "Cannot load the profile `{profile_name}` of the context `{}`: {err}",
                                    ctx.name
                                ));
                            }
                        }
                    }
                    return Ok(ResolvedConnection {
                        source: ConnectionSource::Contexts,
                        auth_helper: ctx.auth_helper.clone(),
                        name: ctx.name,
                        cloud: ctx.cloud,
                        warnings,
                    });
                }
                Ok(None) if self.context.is_some() => {
                    return Err(SelectionError::ContextsFileNotFound);
                }
                Ok(None) => {}
                // Explicitly requested context or file must be usable
                Err(err) if self.context.is_some() || self.contexts_file.is_some() => {
                    return Err(err.into());
                }
                Err(err) => {
                    let msg = format!("Ignoring the contexts.yaml: {err}");
                    warnings.push(msg);
                }
            }
        }
        if self.from_env {
            debug!("Using environment variables for the cloud connection");
            let name = self
                .env_cloud_name
                .clone()
                .unwrap_or_else(|| String::from("envvars"));
            let mut cloud = CloudConfig::from_env()?;
            cloud.name = Some(name.clone());
            return Ok(ResolvedConnection {
                name,
                cloud,
                source: ConnectionSource::Env,
                auth_helper: None,
                warnings,
            });
        }
        let cfg = load_clouds()?;
        match &self.cloud {
            Some(name) => {
                let cloud = cfg
                    .get_cloud_config(name)?
                    .ok_or_else(|| SelectionError::ConnectionNotFound(name.clone()))?;
                Ok(ResolvedConnection {
                    name: name.clone(),
                    cloud,
                    source: ConnectionSource::Clouds,
                    auth_helper: None,
                    warnings,
                })
            }
            None => {
                let mut clouds = cfg.get_available_clouds();
                clouds.sort();
                // The caller may let the user choose: the contexts must be offered regardless of
                // the options which prevented using the `current-context`.
                if let Some(load) = load_contexts.take() {
                    match load() {
                        Ok(Some(file)) => {
                            contexts_names = file.get_available_contexts();
                            warnings.extend(file.warnings);
                        }
                        Ok(None) => {}
                        Err(err) => {
                            let msg = format!("Ignoring the contexts.yaml: {err}");
                            warnings.push(msg);
                        }
                    }
                }
                Err(SelectionError::NoSelection {
                    clouds,
                    contexts: contexts_names,
                    warnings,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const CONTEXTS: &str = r#"
current-context: ctx
servers: [{name: s, server: {auth_url: 'http://x/v3'}}]
users: [{name: u, user: {username: u, password: p, auth_helper: /bin/true}}]
contexts:
  - {name: ctx, context: {server: s, user: u, region_name: R1}}
  - {name: other, context: {server: s, user: u}}
"#;

    fn contexts(current: bool) -> Result<Option<ContextsFile>, ContextsError> {
        let mut file = ContextsFile::from_yaml(CONTEXTS)?;
        if !current {
            file.current_context = None;
        }
        Ok(Some(file))
    }

    fn clouds() -> Result<ConfigFile, ConfigError> {
        Ok(ConfigFile {
            cache: None,
            clouds: Some(HashMap::from([
                ("b".to_string(), CloudConfig::default()),
                ("a".to_string(), CloudConfig::default()),
            ])),
            public_clouds: None,
        })
    }

    fn no_profiles() -> Result<HashMap<String, CloudConfig>, ConfigError> {
        Ok(HashMap::new())
    }

    fn select(
        sel: &ConnectionSelector,
        contexts_file: Result<Option<ContextsFile>, ContextsError>,
    ) -> Result<ResolvedConnection, SelectionError> {
        sel.select_with(|| contexts_file, no_profiles, clouds)
    }

    #[test]
    fn implicit_context() {
        let r = select(&ConnectionSelector::default(), contexts(true)).unwrap();
        assert_eq!(r.source, ConnectionSource::Contexts);
        assert_eq!(r.name, "ctx");
        assert_eq!(r.auth_helper.as_deref(), Some("/bin/true"));
    }

    #[test]
    fn implicit_context_needs_current_context() {
        match select(&ConnectionSelector::default(), contexts(false)) {
            Err(SelectionError::NoSelection {
                clouds, contexts, ..
            }) => {
                assert_eq!(clouds, ["a", "b"]);
                assert_eq!(contexts, ["ctx", "other"]);
            }
            other => panic!("{other:?}"),
        }
        assert!(matches!(
            select(&ConnectionSelector::default(), Ok(None)),
            Err(SelectionError::NoSelection { .. })
        ));
    }

    #[test]
    fn server_profile_is_merged() {
        let file = || {
            Ok(Some(
                ContextsFile::from_yaml(
                    r#"
servers: [{name: s, server: {auth_url: 'http://x/v3', profile: vendor}}]
users: [{name: u, user: {username: u, password: p}}]
contexts: [{name: ctx, context: {server: s, user: u}}]
"#,
                )
                .unwrap(),
            ))
        };
        let sel = ConnectionSelector {
            context: Some("ctx".into()),
            ..Default::default()
        };
        let vendor = CloudConfig {
            region_name: Some("VR".into()),
            verify: Some(false),
            cacert: Some("/evil".into()),
            ..Default::default()
        };
        let with = |public: HashMap<String, CloudConfig>| {
            sel.select_with(file, move || Ok(public), clouds).unwrap()
        };
        let r = with(HashMap::from([("vendor".to_string(), vendor)]));
        assert_eq!(r.cloud.region_name.as_deref(), Some("VR"));
        assert!(r.warnings.is_empty());
        // security relevant settings are never taken from the profile
        assert_eq!(r.cloud.verify, None);
        assert_eq!(r.cloud.cacert, None);
        let r = with(HashMap::new());
        assert_eq!(r.cloud.region_name, None);
        assert!(r.warnings[0].contains("`vendor`"));
    }

    #[test]
    fn explicit_context_wins() {
        let sel = ConnectionSelector {
            context: Some("other".into()),
            ..Default::default()
        };
        let r = select(&sel, contexts(true)).unwrap();
        assert_eq!(
            (r.name.as_str(), r.source),
            ("other", ConnectionSource::Contexts)
        );

        assert!(matches!(
            select(&sel, Ok(None)),
            Err(SelectionError::ContextsFileNotFound)
        ));
        let sel = ConnectionSelector {
            context: Some("missing".into()),
            ..Default::default()
        };
        assert!(matches!(
            select(&sel, contexts(true)),
            Err(SelectionError::Contexts(ContextsError::ContextNotFound(_)))
        ));
    }

    #[test]
    fn cloud_disables_implicit_context() {
        let sel = ConnectionSelector {
            cloud: Some("a".into()),
            ..Default::default()
        };
        let r = select(&sel, contexts(true)).unwrap();
        assert_eq!((r.name.as_str(), r.source), ("a", ConnectionSource::Clouds));
        assert!(r.auth_helper.is_none());

        let sel = ConnectionSelector {
            cloud: Some("zzz".into()),
            ..Default::default()
        };
        assert!(matches!(
            select(&sel, contexts(true)),
            Err(SelectionError::ConnectionNotFound(_))
        ));
    }

    #[test]
    fn overrides_disable_implicit_context() {
        for sel in [
            ConnectionSelector {
                project_override: true,
                ..Default::default()
            },
            ConnectionSelector {
                env_cloud_name: Some("n".into()),
                ..Default::default()
            },
        ] {
            assert!(matches!(
                select(&sel, contexts(true)),
                Err(SelectionError::NoSelection { .. })
            ));
        }
    }

    #[test]
    fn files_and_skip_disable_implicit_context() {
        for sel in [
            ConnectionSelector {
                config_file: Some("c.yaml".into()),
                ..Default::default()
            },
            ConnectionSelector {
                secure_file: Some("s.yaml".into()),
                ..Default::default()
            },
            ConnectionSelector {
                skip_implicit_context: true,
                ..Default::default()
            },
        ] {
            assert!(matches!(
                select(&sel, contexts(true)),
                Err(SelectionError::NoSelection { .. })
            ));
        }
        // but an explicit context still works
        let sel = ConnectionSelector {
            skip_implicit_context: true,
            context: Some("ctx".into()),
            ..Default::default()
        };
        assert!(select(&sel, contexts(true)).is_ok());
    }

    #[test]
    fn broken_contexts_file_is_ignored_when_implicit() {
        let broken = || Err(ContextsError::NoCurrentContext);
        match ConnectionSelector::default().select_with(broken, no_profiles, clouds) {
            Err(SelectionError::NoSelection { warnings, .. }) => {
                assert_eq!(warnings.len(), 1);
                assert!(warnings[0].contains("Ignoring the contexts.yaml"));
            }
            other => panic!("{other:?}"),
        }
        let sel = ConnectionSelector {
            context: Some("ctx".into()),
            ..Default::default()
        };
        assert!(matches!(
            sel.select_with(broken, no_profiles, clouds),
            Err(SelectionError::Contexts(_))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn explicit_contexts_file() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("my-contexts.yaml");
        std::fs::write(&path, CONTEXTS.replace("/bin/true", "/bin/false")).unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();

        // the file is used (also implicitly through its `current-context`)
        for context in [None, Some("other".to_string())] {
            let sel = ConnectionSelector {
                context: context.clone(),
                contexts_file: Some(path.clone()),
                ..Default::default()
            };
            let r = sel.resolve().unwrap();
            assert_eq!(r.source, ConnectionSource::Contexts);
            assert_eq!(r.name, context.unwrap_or("ctx".into()));
            assert_eq!(r.auth_helper.as_deref(), Some("/bin/false"));
        }

        // the missing file is an error, also when the context is implicit
        let sel = ConnectionSelector {
            contexts_file: Some(dir.path().join("missing.yaml")),
            ..Default::default()
        };
        assert!(matches!(
            sel.resolve(),
            Err(SelectionError::Contexts(ContextsError::FileNotFound { .. }))
        ));
    }

    #[test]
    fn contexts_are_offered_when_overrides_disable_implicit() {
        let sel = ConnectionSelector {
            region_name: Some("R".into()),
            ..Default::default()
        };
        match select(&sel, contexts(true)) {
            Err(SelectionError::NoSelection { contexts, .. }) => {
                assert_eq!(contexts, ["ctx", "other"]);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn file_warnings_are_passed_on() {
        let mut file = ContextsFile::from_yaml(CONTEXTS).unwrap();
        file.warnings.push("exposed".into());
        let r = select(&ConnectionSelector::default(), Ok(Some(file))).unwrap();
        assert_eq!(r.warnings, ["exposed"]);
    }

    #[test]
    fn sources_are_loaded_lazily() {
        let sel = ConnectionSelector {
            cloud: Some("a".into()),
            ..Default::default()
        };
        // contexts are not consulted for an explicit cloud
        assert!(
            sel.select_with(|| panic!("contexts loaded"), no_profiles, clouds)
                .is_ok()
        );
        // clouds are not consulted for a context
        assert!(
            ConnectionSelector::default()
                .select_with(|| contexts(true), no_profiles, || panic!("clouds loaded"))
                .is_ok()
        );
    }

    #[test]
    fn region_is_applied() {
        let sel = ConnectionSelector {
            context: Some("ctx".into()),
            region_name: Some("R2".into()),
            ..Default::default()
        };
        let selected = sel
            .select_with(|| contexts(true), no_profiles, clouds)
            .unwrap();
        assert_eq!(selected.cloud.region_name.as_deref(), Some("R1"));
        let resolved = sel
            .resolve_with(|| contexts(true), no_profiles, clouds)
            .unwrap();
        assert_eq!(resolved.cloud.region_name.as_deref(), Some("R2"));
    }
}
