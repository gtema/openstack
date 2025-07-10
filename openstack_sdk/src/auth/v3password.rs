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

//! Helper methods to deal with OpenStack authentication with user name/password
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "password"
//!             ],
//!             "password": {
//!                 "user": {
//!                     "name": "admin",
//!                     "domain": {
//!                         "name": "Default"
//!                     },
//!                     "password": "devstacker"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```
use secrecy::ExposeSecret;
use thiserror::Error;

use crate::auth::auth_helper::{AuthHelper, AuthHelperError};
use crate::auth::auth_token_endpoint as token_v3;
use crate::config;

/// User name/pass related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PasswordError {
    /// Authentication helper error
    #[error(transparent)]
    AuthHelper {
        /// The error source
        #[from]
        source: AuthHelperError,
    },

    /// Password missing
    #[error("User password is missing")]
    MissingPassword,

    /// User name/id missing
    #[error("User id/name is missing")]
    MissingUserId,

    /// `password` part build error
    #[error("Cannot construct password auth information from config: {}", source)]
    PasswordBuilder {
        /// The error source
        #[from]
        source: token_v3::PasswordBuilderError,
    },

    /// `user` part build error
    #[error("Cannot construct user auth information from config: {}", source)]
    UserBuilder {
        /// The error source
        #[from]
        source: token_v3::PasswordUserBuilderError,
    },

    /// `user.domain` part build error
    #[error("Cannot construct user domain information from config: {}", source)]
    UserDomainBuilder {
        /// The error source
        #[from]
        source: token_v3::DomainBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with user password data
pub async fn fill_identity<A: AuthHelper, S: AsRef<str>>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    connection_name: Option<S>,
    auth_helper: &mut A,
) -> Result<(), PasswordError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Password]));
    let mut user = token_v3::PasswordUserBuilder::default();
    // Set user_id or name
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    }
    if let Some(val) = &auth_data.username {
        user.name(val.clone());
    }
    if auth_data.user_id.is_none() && auth_data.username.is_none() {
        // Or ask user for username in interactive mode
        let name = auth_helper
            .get(
                "username".into(),
                connection_name.as_ref().map(|x| x.as_ref().to_string()),
            )
            .await
            .map_err(|_| PasswordError::MissingUserId)?
            .to_owned();
        user.name(name);
    }
    // Fill password
    if let Some(val) = &auth_data.password {
        user.password(val.clone());
    } else {
        // Or ask user for password
        let password = auth_helper
            .get_secret(
                "password".into(),
                connection_name.map(|x| x.as_ref().to_string()),
            )
            .await
            .map_err(|_| PasswordError::MissingPassword)?
            .expose_secret()
            .to_owned();
        user.password(password);
    }

    // Process user domain information
    if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
        let mut user_domain = token_v3::DomainBuilder::default();
        if let Some(val) = &auth_data.user_domain_id {
            user_domain.id(val.clone());
        }
        if let Some(val) = &auth_data.user_domain_name {
            user_domain.name(val.clone());
        }
        user.domain(user_domain.build()?);
    }

    let password = token_v3::PasswordBuilder::default()
        .user(user.build()?)
        .build()?;
    identity_builder.password(password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use secrecy::ExposeSecret;
    use serde_json::json;
    use tracing_test::traced_test;

    use super::*;
    use crate::auth::auth_helper::NonInteractive;
    use crate::config;

    #[tokio::test]
    async fn test_fill_raise_no_user_id_or_name() {
        let config = config::Auth {
            password: Some("pass".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(
            &mut identity,
            &config,
            None::<&str>,
            &mut NonInteractive::default(),
        )
        .await;
        match res.unwrap_err() {
            PasswordError::MissingUserId => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_no_password() {
        let config = config::Auth {
            user_id: Some("uid".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(
            &mut identity,
            &config,
            None::<&str>,
            &mut NonInteractive::default(),
        )
        .await;
        match res.unwrap_err() {
            PasswordError::MissingPassword => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill() {
        let config = config::Auth {
            user_id: Some("uid".into()),
            username: Some("un".into()),
            user_domain_id: Some("udi".into()),
            user_domain_name: Some("udn".into()),
            password: Some("pass".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(
            &mut identity,
            &config,
            None::<&str>,
            &mut NonInteractive::default(),
        )
        .await
        .unwrap();
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["password"],
                "password": {
                    "user": {
                        "id": "uid",
                        "name": "un",
                        "domain": {
                            "id": "udi",
                            "name": "udn"
                        },
                        "password": "pass"
                    },
                }
            })
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_password_not_in_log() {
        let config = config::Auth {
            user_id: Some("uid".into()),
            username: Some("un".into()),
            user_domain_id: Some("udi".into()),
            user_domain_name: Some("udn".into()),
            password: Some("secret".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(
            &mut identity,
            &config,
            None::<&str>,
            &mut NonInteractive::default(),
        )
        .await
        .unwrap();
        let identity = identity.build().unwrap();
        assert!(!logs_contain("secret"));
        assert_eq!(
            "secret",
            identity
                .password
                .unwrap()
                .user
                .unwrap()
                .password
                .unwrap()
                .expose_secret()
        );
    }
}
