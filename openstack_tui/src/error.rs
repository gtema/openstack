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

use thiserror::Error;

use crate::action;

#[derive(Debug, Error)]
pub enum TuiError {
    #[error("ID of entry cannot be determined")]
    EntryIdNotPresent(serde_json::Value),

    /// Json [de]serialization error.
    #[error("json serde error: {}", source)]
    JsonError {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },

    /// Error sending the Action.
    #[error("error sending action: {}", source)]
    ActionSenderError {
        /// The source of the error.
        #[from]
        source: tokio::sync::mpsc::error::SendError<action::Action>,
    },

    /// Error sending the AuthAction.
    #[error("error sending action: {}", source)]
    AuthActionSenderError {
        /// The source of the error.
        #[from]
        source: tokio::sync::mpsc::error::SendError<
            tokio::sync::oneshot::Sender<crate::cloud_worker::AuthAction>,
        >,
    },

    /// OpenStack error.
    #[error(transparent)]
    OpenStackError {
        /// The source of the error.
        #[from]
        source: openstack_sdk::OpenStackError,
    },

    #[error(transparent)]
    CloudWorker {
        /// The source of the error.
        #[from]
        source: crate::cloud_worker::CloudWorkerError,
    },

    /// IO communication error
    #[error("`IO` error: {}", source)]
    IO {
        /// The error source
        #[from]
        source: std::io::Error,
    },

    /// SDK config error.
    #[error("cloud configs error: {}", source)]
    SdkConfig {
        /// The source of the error.
        #[from]
        source: openstack_sdk::config::ConfigError,
    },

    /// TryFromInt conversion error.
    #[error("integer overflow: {}", source)]
    TryFromInt {
        /// The source of the error.
        #[from]
        source: std::num::TryFromIntError,
    },

    /// URL parsing error
    #[error(transparent)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },

    /// Others.
    #[error(transparent)]
    Other(#[from] eyre::Report),
}
