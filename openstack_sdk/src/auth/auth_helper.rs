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

//! Authentication helper.
//!
//! Authentication data may not be present in the configuration file, not provided during the
//! calling operation or simply not known in advance. When authentication requires a TOTP or other
//! temporary token - may be during the re-authentication or session renewal - the is no other way
//! rather than the client to provide a callback that the authentication routine may invoke to
//! request for such additional authentication data. This module defines such interface.

use async_trait::async_trait;
use dialoguer::{Input, Password};
use secrecy::SecretString;
use std::process::Command;
use thiserror::Error;
use tracing::warn;

/// Authentication helper errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthHelperError {
    /// Dialoguer error
    #[error("error in user communication: {0}")]
    Dialoguer(#[from] dialoguer::Error),

    /// No support in non-interactive mode
    #[error("auth_helper is not supported in the non interactive mode")]
    NotSupported,

    /// UTF8 error
    #[error("error parsing the data as utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),

    /// Implementer error
    #[error("{0}")]
    Other(String),

    /// Other errors
    #[error("unknown error in the authentication helper")]
    Unknown,
}

/// Authentication helper trait for providing certain functionality, such as interactive querying
/// the user for the username, password, token and similar.
#[async_trait]
pub trait AuthHelper {
    /// Interactive query to get the regular not sensitive authentication data (i.e. username,
    /// project name, ...)
    async fn get(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError>;

    /// Interactive query to get the sensitive data (i.e. password or token)
    async fn get_secret(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError>;
}

/// Cli prompts AuthHelper.
///
/// AuthHelper based on the regular interactive prompts using the `dialoguer` crate.
#[derive(Clone, Default)]
pub struct Dialoguer {
    pub cloud_name: Option<String>,
}

#[async_trait]
impl AuthHelper for Dialoguer {
    async fn get(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        let prompt = if let Some(connection) = &connection_name {
            format!("Please provide the {key} for the cloud `{connection}`")
        } else {
            format!("Please provide the {key}")
        };
        Ok(Input::new().with_prompt(prompt).interact()?)
    }

    async fn get_secret(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        let prompt = if let Some(connection) = &connection_name {
            format!("Please provide the {key} for the cloud `{connection}`")
        } else {
            format!("Please provide the {key}")
        };
        let secret = Password::new().with_prompt(prompt).interact()?;
        Ok(SecretString::from(secret))
    }
}

/// A NonInteractive mode AuthHelper.
#[derive(Clone, Default)]
pub struct Noop {}

#[async_trait]
impl AuthHelper for Noop {
    async fn get(
        &mut self,
        _key: String,
        _connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        Err(AuthHelperError::NotSupported)
    }

    async fn get_secret(
        &mut self,
        _key: String,
        _connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        Err(AuthHelperError::NotSupported)
    }
}

/// External command AuthHelper.
///
/// Invoke external command to obtain a secret.
#[derive(Clone, Debug)]
pub struct ExternalCmd {
    /// External command
    pub command: String,
}

impl ExternalCmd {
    /// Construct new External command AuthHelper.
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

#[async_trait]
impl AuthHelper for ExternalCmd {
    async fn get(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        let mut command = Command::new(&self.command);
        command.arg(key);
        if let Some(connection) = &connection_name {
            command.arg(connection);
        }
        match command.output() {
            Ok(res) => {
                if !res.stderr.is_empty() {
                    warn!(
                        "{:?} written to stderr during invocation: {:?}",
                        self.command,
                        String::from_utf8(res.stderr)?.trim_end_matches('\n')
                    );
                }
                let stdout: String = String::from_utf8(res.stdout)?;
                Ok(stdout.trim_end_matches('\n').into())
            }
            Err(e) => {
                return Err(AuthHelperError::Other(e.to_string()));
            }
        }
    }

    async fn get_secret(
        &mut self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        let mut command = Command::new(&self.command);
        command.arg(key);
        if let Some(connection) = &connection_name {
            command.arg(connection);
        }
        match command.output() {
            Ok(res) => {
                if !res.stderr.is_empty() {
                    warn!(
                        "{:?} written to stderr during invocation: {:?}",
                        self.command,
                        String::from_utf8(res.stderr)?.trim_end_matches('\n')
                    );
                }
                let stdout: String = String::from_utf8(res.stdout)?;
                Ok(stdout.trim_end_matches('\n').into())
            }
            Err(e) => {
                return Err(AuthHelperError::Other(e.to_string()));
            }
        }
    }
}
