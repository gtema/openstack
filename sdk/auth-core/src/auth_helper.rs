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
use secrecy::SecretString;
use std::process::Command;
use std::sync::Arc;
use thiserror::Error;
use tracing::warn;

/// Authentication helper errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthHelperError {
    /// Dialoguer error
    #[cfg(feature = "interactive")]
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
pub trait AuthHelper: Send + Sync {
    /// Clone this helper into a boxed trait object.
    fn clone_box(&self) -> Box<dyn AuthHelper>;

    /// Interactive query to get the regular not sensitive authentication data (i.e. username,
    /// project name, ...)
    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError>;

    /// Interactive query to get the sensitive data (i.e. password or token)
    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError>;
}

/// Cli prompts AuthHelper.
///
/// AuthHelper based on the regular interactive prompts using the `dialoguer` crate.
#[cfg(feature = "interactive")]
#[derive(Clone, Default)]
pub struct Dialoguer {
    pub cloud_name: Option<String>,
}

#[cfg(feature = "interactive")]
#[async_trait]
impl AuthHelper for Dialoguer {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        Box::new(self.clone())
    }

    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        let prompt = if let Some(connection) = &connection_name {
            format!("Please provide the {key} for the cloud `{connection}`")
        } else {
            format!("Please provide the {key}")
        };
        Ok(dialoguer::Input::new().with_prompt(prompt).interact()?)
    }

    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        let prompt = if let Some(connection) = &connection_name {
            format!("Please provide the {key} for the cloud `{connection}`")
        } else {
            format!("Please provide the {key}")
        };
        let secret = dialoguer::Password::new().with_prompt(prompt).interact()?;
        Ok(SecretString::from(secret))
    }
}

/// A NonInteractive mode AuthHelper.
#[derive(Clone, Default)]
pub struct Noop {}

#[async_trait]
impl AuthHelper for Noop {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        Box::new(self.clone())
    }

    async fn get(
        &self,
        _key: String,
        _connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        Err(AuthHelperError::NotSupported)
    }

    async fn get_secret(
        &self,
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
    /// Treat a failure or an empty output of the command as an error.
    pub strict: bool,
}

impl ExternalCmd {
    /// Construct new External command AuthHelper.
    ///
    /// The output of the command is returned as is, whatever the exit status is (the behavior of
    /// the `--auth-helper-cmd`).
    pub fn new(command: String) -> Self {
        Self {
            command,
            strict: false,
        }
    }

    /// Construct a strict External command AuthHelper.
    ///
    /// A command which cannot be started, exits with a failure or prints nothing is an error:
    /// an empty secret is never useful and must not be sent to the server. Combined with the
    /// [`Fallback`] this lets the user be asked for the value instead.
    pub fn strict(command: String) -> Self {
        Self {
            command,
            strict: true,
        }
    }
}

impl ExternalCmd {
    /// Run the command asking for the `key` and return what it printed.
    ///
    /// In the [`strict`](ExternalCmd::strict) mode a command which exits with a failure or prints
    /// nothing is an error.
    fn run(&self, key: String, connection_name: Option<String>) -> Result<String, AuthHelperError> {
        let mut command = Command::new(&self.command);
        command.arg(&key);
        if let Some(connection) = &connection_name {
            command.arg(connection);
        }
        let res = command
            .output()
            .map_err(|e| AuthHelperError::Other(format!("{:?}: {e}", self.command)))?;
        if !res.stderr.is_empty() {
            warn!(
                "{:?} written to stderr during invocation: {:?}",
                self.command,
                String::from_utf8_lossy(&res.stderr).trim_end_matches('\n')
            );
        }
        if self.strict && !res.status.success() {
            return Err(AuthHelperError::Other(format!(
                "{:?} requested for `{key}` failed with {}",
                self.command, res.status
            )));
        }
        let stdout: String = String::from_utf8(res.stdout)?;
        let value = stdout.trim_end_matches('\n');
        if self.strict && value.is_empty() {
            return Err(AuthHelperError::Other(format!(
                "{:?} returned nothing for `{key}`",
                self.command
            )));
        }
        Ok(value.into())
    }
}

#[async_trait]
impl AuthHelper for ExternalCmd {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        Box::new(self.clone())
    }

    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        self.run(key, connection_name)
    }

    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        self.run(key, connection_name).map(SecretString::from)
    }
}

/// AuthHelper asking the `secondary` when the `primary` fails.
///
/// Allows, for example, to get the stored secrets from an [`ExternalCmd`] and fall back to the
/// interactive prompt for what it cannot provide (i.e. the MFA passcode).
pub struct Fallback {
    primary: Box<dyn AuthHelper>,
    secondary: Box<dyn AuthHelper>,
}

impl Fallback {
    /// Construct the helper.
    pub fn new(primary: impl AuthHelper + 'static, secondary: impl AuthHelper + 'static) -> Self {
        Self {
            primary: Box::new(primary),
            secondary: Box::new(secondary),
        }
    }
}

#[async_trait]
impl AuthHelper for Fallback {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        Box::new(Self {
            primary: self.primary.clone_box(),
            secondary: self.secondary.clone_box(),
        })
    }

    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        match self.primary.get(key.clone(), connection_name.clone()).await {
            Ok(val) => Ok(val),
            Err(err) => {
                warn!("{err}, asking for `{key}` in a different way");
                self.secondary.get(key, connection_name).await
            }
        }
    }

    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        match self
            .primary
            .get_secret(key.clone(), connection_name.clone())
            .await
        {
            Ok(val) => Ok(val),
            Err(err) => {
                warn!("{err}, asking for `{key}` in a different way");
                self.secondary.get_secret(key, connection_name).await
            }
        }
    }
}

/// Forward the blanket impl to the wrapped trait object.
///
/// The calls must explicitly deref through the smart pointer (`&**self`): a bare
/// `self.method()` resolves back to this very impl (`Box<dyn AuthHelper>: AuthHelper`)
/// and recurses forever. `&**self` is a `&dyn AuthHelper` and dispatches via the vtable
/// to the concrete implementation.
#[async_trait]
impl AuthHelper for Box<dyn AuthHelper + 'static> {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        (**self).clone_box()
    }

    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        (**self).get(key, connection_name).await
    }

    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        (**self).get_secret(key, connection_name).await
    }
}

#[async_trait]
impl AuthHelper for Arc<dyn AuthHelper + 'static> {
    fn clone_box(&self) -> Box<dyn AuthHelper> {
        (**self).clone_box()
    }

    async fn get(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<String, AuthHelperError> {
        (**self).get(key, connection_name).await
    }

    async fn get_secret(
        &self,
        key: String,
        connection_name: Option<String>,
    ) -> Result<SecretString, AuthHelperError> {
        (**self).get_secret(key, connection_name).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::ExposeSecret;

    #[derive(Clone)]
    struct Fixed(String);

    #[async_trait]
    impl AuthHelper for Fixed {
        fn clone_box(&self) -> Box<dyn AuthHelper> {
            Box::new(self.clone())
        }

        async fn get(
            &self,
            _key: String,
            _connection_name: Option<String>,
        ) -> Result<String, AuthHelperError> {
            Ok(self.0.clone())
        }

        async fn get_secret(
            &self,
            _key: String,
            _connection_name: Option<String>,
        ) -> Result<SecretString, AuthHelperError> {
            Ok(SecretString::from(self.0.clone()))
        }
    }

    // Regression: the `Box`/`Arc<dyn AuthHelper>` blanket impls used to call
    // `self.method()`, which resolved back to themselves and recursed until the
    // stack overflowed. They must forward to the wrapped concrete impl instead.
    #[tokio::test]
    async fn arc_blanket_impl_forwards_without_recursion() {
        let helper: Arc<dyn AuthHelper> = Arc::new(Fixed("secret".into()));
        assert_eq!(helper.get("k".into(), None).await.unwrap(), "secret");
        assert_eq!(
            helper
                .get_secret("k".into(), None)
                .await
                .unwrap()
                .expose_secret(),
            "secret"
        );
        let _ = helper.clone_box();
    }

    #[tokio::test]
    async fn box_blanket_impl_forwards_without_recursion() {
        let helper: Box<dyn AuthHelper> = Box::new(Fixed("secret".into()));
        assert_eq!(helper.get("k".into(), None).await.unwrap(), "secret");
        assert_eq!(
            helper
                .get_secret("k".into(), None)
                .await
                .unwrap()
                .expose_secret(),
            "secret"
        );
        let _ = helper.clone_box();
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn external_cmd_failures_are_errors() {
        // the plain helper returns whatever was printed
        assert_eq!(
            ExternalCmd::new("/bin/false".into())
                .get("k".into(), None)
                .await
                .unwrap(),
            ""
        );
        assert_eq!(
            ExternalCmd::new("/bin/true".into())
                .get("k".into(), None)
                .await
                .unwrap(),
            ""
        );
        // exits with a failure
        assert!(
            ExternalCmd::strict("/bin/false".into())
                .get("k".into(), None)
                .await
                .is_err()
        );
        // succeeds but prints nothing
        assert!(
            ExternalCmd::strict("/bin/true".into())
                .get_secret("k".into(), None)
                .await
                .is_err()
        );
        // prints the key
        assert_eq!(
            ExternalCmd::strict("/bin/echo".into())
                .get("k".into(), None)
                .await
                .unwrap(),
            "k"
        );
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn fallback_is_used_on_failure() {
        let helper = Fallback::new(
            ExternalCmd::strict("/bin/false".into()),
            Fixed("manual".into()),
        );
        assert_eq!(helper.get("k".into(), None).await.unwrap(), "manual");
        assert_eq!(
            helper
                .get_secret("k".into(), None)
                .await
                .unwrap()
                .expose_secret(),
            "manual"
        );
        let helper = Fallback::new(
            ExternalCmd::strict("/bin/echo".into()),
            Fixed("manual".into()),
        );
        assert_eq!(helper.clone_box().get("k".into(), None).await.unwrap(), "k");
    }
}
