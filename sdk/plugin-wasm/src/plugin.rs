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
//! Every conforming module exports:
//!
//! - `plugin_abi_version(_: string) -> string` — must return the literal `"1"`.
//! - `auth_supported_methods(_: string) -> string` — JSON array of auth method
//!   names, e.g. `["v3myauth"]`.
//! - `auth_api_version(_: string) -> string` — JSON `[major, minor]`.
//! - `auth_requirements(hints: string) -> string` — `hints` is a JSON value or
//!   the literal `null`; returns a JSON Schema object describing required
//!   fields, in the same shape [`OpenStackAuthType::requirements`] expects.
//!
//! and then exactly one of the two ABI flavors below, detected at load time
//! via `extism::Plugin::function_exists`:
//!
//! ### `auth` flavor
//!
//! - `auth(request: string) -> string` — `request` is a JSON object
//!   `{"identity_url", "values", "scope", "hints"}`; returns either
//!   `{"ok": {"token": "...", "auth_info": <AuthResponse|null>}}` or
//!   `{"error": "human readable message"}`.
//!
//!   Only `auth` may perform outbound HTTP, and only via the host-provided
//!   `identity_http_request` import — never directly.
//!
//! ### `sso` flavor
//!
//! For interactive, browser-based (WebSSO-style) plugins. Neither export may
//! perform any I/O: the host owns the callback listener, the anti-CSRF
//! `state` check, and the browser-opening step (via
//! `openstack_sdk_websso_host`); the guest only ever sees already-validated
//! strings in and returns strings out.
//!
//! - `sso_build_request(request: string) -> string` — `request` is
//!   `{"identity_url", "callback_url", "values", "scope", "hints"}`, where
//!   `callback_url` is the host-bound local callback URL (with the
//!   anti-CSRF `state` token already embedded) the guest must have the
//!   identity provider redirect back to. Returns
//!   `{"url": "https://...", "redirect_host": "host:port"}`: `url` is the
//!   page to open in the user's browser, and `redirect_host` is the
//!   `host:port` authority the guest configured as the identity provider's
//!   redirect target. The host verifies, before opening any browser, that
//!   `url`'s scheme is `https` and that `redirect_host` exactly matches the
//!   host-bound callback listener's own authority — a plugin that returns a
//!   URL on a different (undeclared) redirect host is rejected outright,
//!   with no override.
//! - `sso_parse_callback(callback: string) -> string` — `callback` is
//!   `{"params": {...}}`, the form fields from the already
//!   state-validated callback POST; returns the same
//!   `{"ok": ...} | {"error": ...}` shape as `auth`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use dialoguer::Confirm;
use extism::{Function, Manifest, Plugin, UserData, ValType, Wasm};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

use openstack_sdk_auth_core::{
    Auth, AuthError, AuthResponse, AuthToken, AuthTokenScope, OpenStackAuthType,
};
use openstack_sdk_websso_host::{BrowserOpenPolicy, CallbackServer};

use crate::error::WasmPluginError;
use crate::host::{self, HostContextState};

/// A guest module implements exactly one of these two ABI flavors,
/// detected at load time. See the module docs for the exports each one
/// requires.
#[derive(Clone, Debug, PartialEq, Eq)]
enum AbiFlavor {
    /// Non-interactive: a single `auth` export handles the whole flow.
    Auth,
    /// Interactive/browser-based: `sso_build_request` + `sso_parse_callback`.
    Sso,
}

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

#[derive(Serialize)]
struct SsoBuildRequestMsg {
    identity_url: String,
    callback_url: String,
    values: BTreeMap<String, String>,
    scope: Option<serde_json::Value>,
    hints: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct SsoBuildResponseMsg {
    url: String,
    redirect_host: String,
}

#[derive(Serialize)]
struct SsoCallbackMsg {
    params: BTreeMap<String, String>,
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
    flavor: AbiFlavor,
}

impl std::fmt::Debug for WasmAuthPlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WasmAuthPlugin")
            .field("name", &self.name)
            .field("source", &self.source)
            .field("supported_methods", &self.supported_methods)
            .field("api_version", &self.api_version)
            .field("flavor", &self.flavor)
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

        let has_auth = plugin.function_exists("auth");
        let has_sso = plugin.function_exists("sso_build_request")
            && plugin.function_exists("sso_parse_callback");
        let flavor = match (has_auth, has_sso) {
            (true, false) => AbiFlavor::Auth,
            (false, true) => AbiFlavor::Sso,
            (true, true) => {
                return Err(WasmPluginError::InvalidAbi {
                    name,
                    reason: "plugin exports both `auth` and the SSO entry points (`sso_build_request`/`sso_parse_callback`); a module must implement exactly one ABI flavor".into(),
                });
            }
            (false, false) => {
                return Err(WasmPluginError::InvalidAbi {
                    name,
                    reason: "plugin exports neither `auth` nor the SSO entry points (`sso_build_request`+`sso_parse_callback`)".into(),
                });
            }
        };

        Ok(Self {
            name,
            source: path.to_path_buf(),
            inner: Arc::new(Mutex::new(plugin)),
            host_ctx,
            supported_methods,
            api_version: (major, minor),
            flavor,
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
        match self.flavor {
            AbiFlavor::Auth => self.auth_via_auth(identity_url, values, scope, hints).await,
            AbiFlavor::Sso => self.auth_via_sso(identity_url, values, scope, hints).await,
        }
    }
}

impl WasmAuthPlugin {
    async fn auth_via_auth(
        &self,
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

    /// Run the `sso` ABI flavor: bind a host-owned callback listener, ask
    /// the guest (a pure computation) to build the identity-provider URL to
    /// open, validate that URL before ever opening a browser, wait for the
    /// already state-validated callback, then hand the callback's fields
    /// back to the guest (again pure) to turn into a token.
    ///
    /// The guest never sees a socket, a browser-opening capability, or the
    /// raw (unvalidated) callback request — every step it participates in
    /// is a JSON-in, JSON-out call over data the host has already checked.
    async fn auth_via_sso(
        &self,
        identity_url: &url::Url,
        values: &std::collections::HashMap<String, SecretString>,
        scope: Option<&AuthTokenScope>,
        hints: Option<&serde_json::Value>,
    ) -> Result<Auth, AuthError> {
        let callback_port = values
            .get("callback_port")
            .and_then(|v| v.expose_secret().parse::<u16>().ok());

        let server = CallbackServer::bind(callback_port)
            .await
            .map_err(|source| {
                AuthError::plugin(WasmPluginError::Host {
                    name: self.name.clone(),
                    source,
                })
            })?;

        let request = SsoBuildRequestMsg {
            identity_url: identity_url.to_string(),
            callback_url: server.callback_url().to_string(),
            values: values
                .iter()
                .map(|(k, v)| (k.clone(), v.expose_secret().to_string()))
                .collect(),
            scope: scope.map(serde_json::to_value).transpose()?,
            hints: hints.cloned(),
        };
        let request_json = serde_json::to_string(&request)?;

        let build_output = self
            .call_guest("sso_build_request", request_json)
            .await
            .map_err(AuthError::plugin)?;

        // The plugin's declared redirect target must be exactly the
        // callback listener the host itself just bound. A mismatch means
        // the plugin is trying to point the identity provider's redirect
        // somewhere the host never bound a listener on — always rejected,
        // no override.
        let sso_url =
            validate_sso_build_response(&self.name, &build_output, &server.redirect_host())
                .map_err(AuthError::plugin)?;

        let confirmation = Confirm::new()
            .with_prompt(format!(
                "A default browser is going to be opened at `{}`. Do you want to continue?",
                sso_url.as_str()
            ))
            .interact()
            .map_err(WasmPluginError::from)
            .map_err(AuthError::plugin)?;
        if !confirmation {
            return Err(AuthError::plugin(WasmPluginError::InvalidRedirect {
                name: self.name.clone(),
                reason: "user declined to open the browser".into(),
            }));
        }

        openstack_sdk_websso_host::open_browser(
            &sso_url,
            BrowserOpenPolicy {
                require_https: true,
            },
        )
        .map_err(|source| {
            AuthError::plugin(WasmPluginError::Host {
                name: self.name.clone(),
                source,
            })
        })?;

        let params = server
            .wait_for_callback(Duration::from_secs(120))
            .await
            .map_err(|source| {
                AuthError::plugin(WasmPluginError::Host {
                    name: self.name.clone(),
                    source,
                })
            })?;

        let callback = SsoCallbackMsg {
            params: params.into_iter().collect(),
        };
        let callback_json = serde_json::to_string(&callback)?;

        let output = self
            .call_guest("sso_parse_callback", callback_json)
            .await
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

    /// Call a pure (no host-function-using) guest export off the async
    /// runtime, since a WASM call can run for up to the plugin's configured
    /// timeout and must not stall the executor.
    async fn call_guest(
        &self,
        function: &'static str,
        input: String,
    ) -> Result<String, WasmPluginError> {
        let name = self.name.clone();
        let inner = self.inner.clone();
        tokio::task::spawn_blocking(move || -> Result<String, WasmPluginError> {
            let mut plugin = inner.lock().map_err(|_| WasmPluginError::HostContext {
                name: name.clone(),
                function,
                reason: "plugin lock poisoned".into(),
            })?;
            plugin
                .call(function, input.as_str())
                .map_err(|source| WasmPluginError::Call {
                    name,
                    function,
                    source,
                })
        })
        .await
        .map_err(|source| WasmPluginError::Join {
            name: self.name.clone(),
            function,
            source,
        })?
    }
}

/// Parse and validate a guest's `sso_build_request` response: the returned
/// `url` must be well-formed and `https`, and the declared `redirect_host`
/// must exactly match `expected_redirect_host` (the host-bound callback
/// listener's own authority). This is the one guest-response deserialization
/// path in this module with security-relevant validation logic beyond a
/// plain type check, so it's kept as a standalone, panic-free function that
/// both [`WasmAuthPlugin::auth_via_sso`] and the `fuzzing`-feature entry
/// point below can exercise directly.
fn validate_sso_build_response(
    name: &str,
    build_output: &str,
    expected_redirect_host: &str,
) -> Result<url::Url, WasmPluginError> {
    let build: SsoBuildResponseMsg = serde_json::from_str(build_output).map_err(|source| {
        WasmPluginError::MalformedAuthResponse {
            name: name.to_string(),
            source,
        }
    })?;

    let sso_url =
        url::Url::parse(&build.url).map_err(|source| WasmPluginError::InvalidRedirect {
            name: name.to_string(),
            reason: format!("`sso_build_request` returned an unparsable url: {source}"),
        })?;
    if sso_url.scheme() != "https" {
        return Err(WasmPluginError::InvalidRedirect {
            name: name.to_string(),
            reason: format!(
                "`sso_build_request` returned a non-https url (scheme was `{}`)",
                sso_url.scheme()
            ),
        });
    }
    if build.redirect_host != expected_redirect_host {
        return Err(WasmPluginError::RedirectHostMismatch {
            name: name.to_string(),
            declared: build.redirect_host,
            expected: expected_redirect_host.to_string(),
        });
    }
    Ok(sso_url)
}

/// Fuzz target entry point for the otherwise-private
/// [`validate_sso_build_response`]. `expected_redirect_host` is itself fuzzed
/// input rather than a fixed value, so the mismatch path is exercised too.
///
/// Only compiled with the `fuzzing` feature; not part of the stable public
/// API.
#[cfg(feature = "fuzzing")]
pub fn fuzz_validate_sso_build_response(build_output: &str, expected_redirect_host: &str) {
    let _ = validate_sso_build_response("fuzz", build_output, expected_redirect_host);
}

/// Fuzz target entry point for parsing the otherwise-private `AuthResultMsg`
/// every guest response (`auth`, `sso_parse_callback`) is deserialized
/// through.
///
/// Only compiled with the `fuzzing` feature; not part of the stable public
/// API.
#[cfg(feature = "fuzzing")]
pub fn fuzz_parse_auth_result(output: &str) {
    let _ = serde_json::from_str::<AuthResultMsg>(output);
}
