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

use std::path::PathBuf;

use thiserror::Error;

/// Errors that may occur while loading or invoking a WASM auth plugin.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WasmPluginError {
    /// The plugin directory could not be identified (no home/data dir).
    #[error("plugin data directory cannot be identified")]
    PluginDirCannotBeIdentified,

    /// I/O error while reading/writing the plugin directory or file.
    #[error("I/O error accessing {}: {}", path.display(), source)]
    Io {
        /// Path being accessed.
        path: PathBuf,
        /// The source of the error.
        #[source]
        source: std::io::Error,
    },

    /// The `.wasm` file could not be loaded/instantiated by the Extism runtime.
    #[error("failed to load wasm plugin from {}: {}", path.display(), source)]
    Load {
        /// Path of the plugin that failed to load.
        path: PathBuf,
        /// The underlying Extism error.
        #[source]
        source: extism::Error,
    },

    /// A required guest export is missing or returned malformed data.
    #[error("plugin {name} does not implement a valid plugin ABI: {reason}")]
    InvalidAbi {
        /// Plugin (file stem) name.
        name: String,
        /// Human readable reason.
        reason: String,
    },

    /// Calling into the plugin failed (trapped, timed out, or returned an error).
    #[error("plugin {name} call to `{function}` failed: {source}")]
    Call {
        /// Plugin name.
        name: String,
        /// Guest function that was called.
        function: &'static str,
        /// The underlying Extism error.
        #[source]
        source: extism::Error,
    },

    /// The plugin's `auth` export returned data that couldn't be parsed as a
    /// token response.
    #[error("plugin {name} returned a malformed auth response: {source}")]
    MalformedAuthResponse {
        /// Plugin name.
        name: String,
        /// The source of the error.
        #[source]
        source: serde_json::Error,
    },

    /// A plugin with the same name is already installed.
    #[error("a plugin named `{0}` is already installed")]
    AlreadyInstalled(String),

    /// No plugin file exists at the given path.
    #[error("plugin file not found: {}", .0.display())]
    NotFound(PathBuf),

    /// Failed to build the sandboxed HTTP client handed to the plugin's host
    /// function for the duration of a single call.
    #[error("failed to build plugin http client: {}", source)]
    HttpClient {
        /// The source of the error.
        #[source]
        source: reqwest::Error,
    },

    /// The blocking task running the guest call could not be joined (panicked
    /// or was cancelled).
    #[error("plugin {name} call to `{function}` task failed: {source}")]
    Join {
        /// Plugin name.
        name: String,
        /// Guest function that was being called.
        function: &'static str,
        /// The underlying join error.
        #[source]
        source: tokio::task::JoinError,
    },

    /// Internal per-call host state (identity origin / HTTP client) could not
    /// be accessed.
    #[error("plugin {name} host context unavailable for `{function}`: {reason}")]
    HostContext {
        /// Plugin name.
        name: String,
        /// Guest function that was being called.
        function: &'static str,
        /// Human readable reason.
        reason: String,
    },

    /// The in-process plugin registry could not be accessed or updated.
    #[error("plugin registry error: {0}")]
    Registry(String),

    /// The plugin lockfile at `path` could not be parsed or serialized.
    #[error("malformed plugin lockfile at {}: {}", path.display(), source)]
    LockfileFormat {
        /// Path of the lockfile.
        path: PathBuf,
        /// The underlying JSON error.
        #[source]
        source: serde_json::Error,
    },

    /// The installed file's content no longer matches the hash recorded in
    /// the lockfile when it was installed.
    #[error(
        "plugin {name}@{version} failed integrity verification: expected sha256 {expected}, found {actual}"
    )]
    HashMismatch {
        /// Plugin name.
        name: String,
        /// Plugin version.
        version: String,
        /// The hash recorded in the lockfile at install time.
        expected: String,
        /// The hash computed from the file currently on disk.
        actual: String,
    },

    /// The requested `name`/`version` is not present in the lockfile.
    #[error("no installed plugin matches {name}{}", version.as_ref().map(|v| format!("@{v}")).unwrap_or_default())]
    NotInstalled {
        /// Plugin name.
        name: String,
        /// Specific version requested, if any; `None` means "any/all
        /// versions of `name`".
        version: Option<String>,
    },

    /// The registry index could not be fetched.
    #[error("failed to fetch plugin registry index from {url}: {source}")]
    RegistryFetch {
        /// Registry index URL.
        url: String,
        /// The underlying HTTP error.
        #[source]
        source: reqwest::Error,
    },

    /// The registry index was fetched but could not be parsed, or declared
    /// an unsupported `schema_version`.
    #[error("malformed plugin registry index at {url}: {reason}")]
    RegistryFormat {
        /// Registry index URL.
        url: String,
        /// Human readable reason.
        reason: String,
    },

    /// A downloaded plugin's content does not match the sha256 declared for
    /// it in the registry index.
    #[error(
        "downloaded plugin {name}@{version} failed checksum verification: expected sha256 {expected}, got {actual}"
    )]
    ChecksumMismatch {
        /// Plugin name.
        name: String,
        /// Plugin version.
        version: String,
        /// The sha256 declared in the registry index.
        expected: String,
        /// The sha256 computed from the downloaded bytes.
        actual: String,
    },

    /// The requested `name`/`version` is not present in the registry index.
    #[error("no plugin matches {name}{} in the registry index", version.as_ref().map(|v| format!("@{v}")).unwrap_or_default())]
    NotInIndex {
        /// Plugin name.
        name: String,
        /// Specific version requested, if any.
        version: Option<String>,
    },

    /// Install/update was refused because the plugin's provenance could not
    /// be verified and `--allow-unsigned` was not given.
    #[error(
        "refusing to install {name}@{version} without provenance verification: {reason} (pass --allow-unsigned to override)"
    )]
    Untrusted {
        /// Plugin name.
        name: String,
        /// Plugin version.
        version: String,
        /// Human readable reason verification did not succeed.
        reason: String,
    },

    /// The GitHub attestations API could not be reached or returned an
    /// unexpected response.
    #[error("failed to fetch attestations for {owner}/{repo}: {source}")]
    AttestationFetch {
        /// Repository owner.
        owner: String,
        /// Repository name.
        repo: String,
        /// The underlying HTTP error.
        #[source]
        source: reqwest::Error,
    },

    /// A fetched attestation bundle failed cryptographic or identity
    /// verification.
    #[error("attestation verification failed: {reason}")]
    AttestationVerification {
        /// Human readable reason.
        reason: String,
    },

    /// The shared WebSSO host service (callback listener, CSRF check, or
    /// browser-opening step) reported an error while running the SSO ABI
    /// flow.
    #[error("plugin {name} SSO flow failed: {source}")]
    Host {
        /// Plugin name.
        name: String,
        /// The underlying host-service error.
        #[source]
        source: openstack_sdk_websso_host::WebssoHostError,
    },

    /// Error using the interactive confirmation prompt during the SSO flow.
    #[error("error using the dialoguer: {}", source)]
    Dialoguer {
        /// The error source.
        #[from]
        source: dialoguer::Error,
    },

    /// `sso_build_request` returned a URL that failed host-side validation:
    /// unparsable, not `https://`, or resolving to an SSRF-denylisted
    /// address (loopback, link-local, private, multicast/reserved,
    /// unspecified).
    #[error("plugin {name} `sso_build_request` returned an invalid redirect: {reason}")]
    InvalidRedirect {
        /// Plugin name.
        name: String,
        /// Human readable reason.
        reason: String,
    },

    /// `sso_build_request`'s declared `redirect_host` didn't match the
    /// host-bound callback listener's own authority — the plugin tried to
    /// point the identity provider's redirect somewhere the host never
    /// bound a listener on. Always rejected; there is no override.
    #[error(
        "plugin {name} declared SSO redirect host `{declared}` but the host-bound callback listener is `{expected}`; refusing to open the browser"
    )]
    RedirectHostMismatch {
        /// Plugin name.
        name: String,
        /// The `redirect_host` the plugin's `sso_build_request` response
        /// declared.
        declared: String,
        /// The actual authority of the host-bound callback listener.
        expected: String,
    },
}
