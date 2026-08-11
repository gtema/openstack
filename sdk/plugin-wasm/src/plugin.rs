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

//! [`WasmAuthPlugin`] adapts a single Extism (WASM) module to the
//! [`OpenStackAuthType`] trait so it can be used anywhere a compiled-in auth
//! plugin can.
//!
//! ## Guest ABI (version 1)
//!
//! A conforming module exports:
//!
//! - `plugin_abi_version(_: string) -> string` — must return the literal `"1"`.
//! - `auth_supported_methods(_: string) -> string` — JSON array of auth method
//!   names, e.g. `["v3myauth"]`.
//! - `auth_api_version(_: string) -> string` — JSON `[major, minor]`.
//! - `auth_requirements(hints: string) -> string` — `hints` is a JSON value or
//!   the literal `null`; returns a JSON Schema object describing required
//!   fields, in the same shape [`OpenStackAuthType::requirements`] expects.
//! - `auth(request: string) -> string` — `request` is a JSON object
//!   `{"identity_url", "values", "scope", "hints"}`; returns either
//!   `{"ok": {"token": "...", "auth_info": <AuthResponse|null>}}` or
//!   `{"error": "human readable message"}`.
//!
//! Only `auth` may perform outbound HTTP, and only via the host-provided
//! `identity_http_request` import — never directly. Every other export must be
//! a pure computation over its input.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use extism::{Function, Manifest, Plugin, UserData, ValType, Wasm};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthResponse, AuthToken, AuthTokenScope, OpenStackAuthType,
};

use crate::error::WasmPluginError;
use crate::host::{self, HostContextState};

/// WASM modules are given at most this much linear memory.
const DEFAULT_MAX_MEMORY_PAGES: u32 = 256; // 256 * 64KiB = 16MiB
/// A single guest call (including any `identity_http_request` round trips it
/// makes) is aborted if it runs longer than this.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(20);
/// The only ABI version this adapter currently understands.
const SUPPORTED_ABI_VERSION: &str = "1";

#[derive(Serialize)]
struct AuthRequestMsg {
    identity_url: String,
    values: BTreeMap<String, String>,
    scope: Option<serde_json::Value>,
    hints: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum AuthResultMsg {
    Ok {
        token: String,
        auth_info: Box<Option<AuthResponse>>,
    },
    Error {
        error: String,
    },
}

/// A single loaded WASM auth plugin, wrapping an [`extism::Plugin`] instance.
///
/// Cheaply cloneable: the underlying plugin and its per-call host state are
/// held behind `Arc`s, so a `WasmAuthPlugin` can be shared across the registry
/// and concurrent auth attempts (calls into the same instance still serialize
/// on its internal lock, matching Extism's own single-call-at-a-time
/// requirement).
#[derive(Clone)]
pub struct WasmAuthPlugin {
    name: String,
    source: PathBuf,
    inner: Arc<Mutex<Plugin>>,
    host_ctx: UserData<HostContextState>,
    supported_methods: Vec<&'static str>,
    api_version: (u8, u8),
}

impl std::fmt::Debug for WasmAuthPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmAuthPlugin")
            .field("name", &self.name)
            .field("source", &self.source)
            .field("supported_methods", &self.supported_methods)
            .field("api_version", &self.api_version)
            .finish_non_exhaustive()
    }
}

impl WasmAuthPlugin {
    /// Load and validate a `.wasm` auth plugin from disk.
    ///
    /// The module is sandboxed: no filesystem, no environment, no WASI at
    /// all, no host-provided HTTP capability other than the restricted
    /// `identity_http_request` import, a bounded memory limit and a
    /// per-call timeout. Loading eagerly probes the guest ABI
    /// (`plugin_abi_version`, `auth_supported_methods`, `auth_api_version`)
    /// so a malformed plugin is rejected at install/load time rather than on
    /// first use.
    pub fn load(path: &Path) -> Result<Self, WasmPluginError> {
        if !path.is_file() {
            return Err(WasmPluginError::NotFound(path.to_path_buf()));
        }
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("plugin")
            .to_string();

        let manifest = Manifest::new([Wasm::file(path)])
            .disallow_all_hosts()
            .with_memory_max(DEFAULT_MAX_MEMORY_PAGES)
            .with_timeout(DEFAULT_TIMEOUT);

        let host_ctx: UserData<HostContextState> = UserData::new(HostContextState::default());
        let functions = vec![Function::new(
            "identity_http_request",
            [ValType::I64],
            [ValType::I64],
            host_ctx.clone(),
            host::identity_http_request,
        )];

        let mut plugin =
            Plugin::new(manifest, functions, false).map_err(|source| WasmPluginError::Load {
                path: path.to_path_buf(),
                source,
            })?;

        let abi_version: String =
            plugin
                .call("plugin_abi_version", "")
                .map_err(|source| WasmPluginError::Call {
                    name: name.clone(),
                    function: "plugin_abi_version",
                    source,
                })?;
        if abi_version.trim() != SUPPORTED_ABI_VERSION {
            return Err(WasmPluginError::InvalidAbi {
                name,
                reason: format!(
                    "unsupported plugin_abi_version `{}`, expected `{SUPPORTED_ABI_VERSION}`",
                    abi_version.trim()
                ),
            });
        }

        let methods_json: String = plugin
            .call("auth_supported_methods", "")
            .map_err(|source| WasmPluginError::Call {
                name: name.clone(),
                function: "auth_supported_methods",
                source,
            })?;
        let methods: Vec<String> =
            serde_json::from_str(&methods_json).map_err(|e| WasmPluginError::InvalidAbi {
                name: name.clone(),
                reason: format!("auth_supported_methods did not return a JSON string array: {e}"),
            })?;
        if methods.is_empty() {
            return Err(WasmPluginError::InvalidAbi {
                name,
                reason: "auth_supported_methods returned an empty list".into(),
            });
        }
        // Leaked intentionally: plugins are process-lifetime singletons once
        // loaded into the registry in this phase (no hot unload yet), so this
        // is a one-time, bounded allocation per installed plugin rather than a
        // per-call or unbounded leak.
        let supported_methods: Vec<&'static str> = methods
            .into_iter()
            .map(|m| -> &'static str { Box::leak(m.into_boxed_str()) })
            .collect();

        let api_version_json: String =
            plugin
                .call("auth_api_version", "")
                .map_err(|source| WasmPluginError::Call {
                    name: name.clone(),
                    function: "auth_api_version",
                    source,
                })?;
        let (major, minor): (u8, u8) =
            serde_json::from_str(&api_version_json).map_err(|e| WasmPluginError::InvalidAbi {
                name: name.clone(),
                reason: format!("auth_api_version did not return a JSON [major, minor] pair: {e}"),
            })?;

        Ok(Self {
            name,
            source: path.to_path_buf(),
            inner: Arc::new(Mutex::new(plugin)),
            host_ctx,
            supported_methods,
            api_version: (major, minor),
        })
    }

    /// The plugin's name (its `.wasm` file stem).
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The path this plugin was loaded from.
    pub fn source(&self) -> &Path {
        &self.source
    }

    /// The auth method names this plugin declares support for.
    pub fn supported_methods(&self) -> &[&'static str] {
        &self.supported_methods
    }

    /// The Identity API version (major, minor) declared by the guest ABI.
    pub fn api_version(&self) -> (u8, u8) {
        self.api_version
    }
}

#[async_trait]
impl OpenStackAuthType for WasmAuthPlugin {
    fn get_supported_auth_methods(&self) -> Vec<&'static str> {
        self.supported_methods.clone()
    }

    fn requirements(
        &self,
        hints: Option<&serde_json::Value>,
    ) -> Result<serde_json::Value, AuthError> {
        let hints_json = match hints {
            Some(v) => serde_json::to_string(v)?,
            None => "null".to_string(),
        };
        let mut plugin = self.inner.lock().map_err(|_| {
            AuthError::plugin(WasmPluginError::HostContext {
                name: self.name.clone(),
                function: "auth_requirements",
                reason: "plugin lock poisoned".into(),
            })
        })?;
        let out: String = plugin
            .call("auth_requirements", hints_json.as_str())
            .map_err(|source| WasmPluginError::Call {
                name: self.name.clone(),
                function: "auth_requirements",
                source,
            })
            .map_err(AuthError::plugin)?;
        Ok(serde_json::from_str(&out)?)
    }

    fn api_version(&self) -> (u8, u8) {
        self.api_version
    }

    async fn auth(
        &self,
        _http_client: &reqwest::Client,
        identity_url: &url::Url,
        values: &std::collections::HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let request = AuthRequestMsg {
            identity_url: identity_url.to_string(),
            values: values
                .iter()
                .map(|(k, v)| (k.clone(), v.expose_secret().to_string()))
                .collect(),
            scope: scope.map(serde_json::to_value).transpose()?,
            hints: hints.cloned(),
        };
        let request_json = serde_json::to_string(&request)?;

        let mut origin = identity_url.clone();
        origin.set_path("");
        origin.set_query(None);
        origin.set_fragment(None);

        let name = self.name.clone();
        let inner = self.inner.clone();
        let host_ctx = self.host_ctx.clone();

        let output = tokio::task::spawn_blocking(move || -> Result<String, WasmPluginError> {
            // A dedicated blocking client, scoped to this single call. Built
            // inside the blocking task: `reqwest::blocking::Client` spins up
            // its own mini Tokio runtime internally, which panics if built
            // from within an already-running async context.
            let client = reqwest::blocking::Client::builder()
                .build()
                .map_err(|source| WasmPluginError::HttpClient { source })?;
            {
                let state = host_ctx.get().map_err(|e| WasmPluginError::HostContext {
                    name: name.clone(),
                    function: "auth",
                    reason: e.to_string(),
                })?;
                let mut state = state.lock().map_err(|_| WasmPluginError::HostContext {
                    name: name.clone(),
                    function: "auth",
                    reason: "host context lock poisoned".into(),
                })?;
                state.identity_origin = Some(origin);
                state.client = Some(client);
            }
            let mut plugin = inner.lock().map_err(|_| WasmPluginError::HostContext {
                name: name.clone(),
                function: "auth",
                reason: "plugin lock poisoned".into(),
            })?;
            plugin
                .call("auth", request_json.as_str())
                .map_err(|source| WasmPluginError::Call {
                    name,
                    function: "auth",
                    source,
                })
        })
        .await
        .map_err(|source| WasmPluginError::Join {
            name: self.name.clone(),
            function: "auth",
            source,
        })
        .map_err(AuthError::plugin)?
        .map_err(AuthError::plugin)?;

        let parsed: AuthResultMsg = serde_json::from_str(&output).map_err(|source| {
            AuthError::plugin(WasmPluginError::MalformedAuthResponse {
                name: self.name.clone(),
                source,
            })
        })?;

        match parsed {
            AuthResultMsg::Ok { token, auth_info } => {
                Ok(Auth::AuthToken(Box::new(AuthToken::new(token, *auth_info))))
            }
            AuthResultMsg::Error { error } => Err(AuthError::UnknownAuth {
                code: 0,
                message: Some(error),
            }),
        }
    }
}
