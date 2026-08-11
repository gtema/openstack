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
}
