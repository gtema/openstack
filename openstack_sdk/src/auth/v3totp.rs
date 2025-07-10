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

//! Helper methods to deal with OpenStack authentication with TOTP token
//!
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "totp"
//!             ],
//!             "totp": {
//!                 "user": {
//!                     "id": "ee4dfb6e5540447cb3741905149d9b6e",
//!                     "passcode": "123456"
//!                 }
//!             }
//!         },
//!     }
//! }
//! ```

use secrecy::ExposeSecret;
use thiserror::Error;

use crate::auth::auth_helper::{AuthHelper, AuthHelperError};
use crate::auth::auth_token_endpoint as token_v3;
use crate::config;

/// TOTP related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TotpError {
    /// Authentication helper error
    #[error(transparent)]
    AuthHelper {
        /// The error source
        #[from]
        source: AuthHelperError,
    },

    /// UserID is required
    #[error("User id is missing")]
    MissingUserId,

    /// Passcode is required
    #[error("Auth token is missing")]
    MissingPasscode,

    /// TotpUser builder
    #[error("Cannot construct TOTP user information: {}", source)]
    UserBuilder {
        /// The error source
        #[from]
        source: token_v3::TotpUserBuilderError,
    },

    /// TotpUserDomain builder
    #[error("Cannot construct TOTP user domain information: {}", source)]
    UserDomainBuilder {
        /// The error source
        #[from]
        source: token_v3::DomainBuilderError,
    },

    /// Totp builder
    #[error("Cannot construct TOTP auth information: {}", source)]
    TotpBuilder {
        /// The error source
        #[from]
        source: token_v3::TotpBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with MFA passcode
pub async fn fill_identity<A: AuthHelper, S: AsRef<str>>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    connection_name: Option<S>,
    auth_helper: &mut A,
) -> Result<(), TotpError> {
    identity_builder.methods(Vec::from([token_v3::Methods::Totp]));
    let mut user = token_v3::TotpUserBuilder::default();
    if let Some(val) = &auth_data.user_id {
        user.id(val.clone());
    } else if let Some(val) = &auth_data.username {
        user.name(val.clone());
    } else {
        // Or ask user for username in interactive mode
        let name = auth_helper
            .get(
                "username".into(),
                connection_name.as_ref().map(|x| x.as_ref().to_string()),
            )
            .await
            .map_err(|_| TotpError::MissingUserId)?
            .to_owned();
        user.name(name);
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

    if let Some(passcode) = &auth_data.passcode {
        user.passcode(passcode.clone());
    } else {
        // Or ask user for username in interactive mode
        let passcode = auth_helper
            .get_secret(
                "passcode".into(),
                connection_name.as_ref().map(|x| x.as_ref().to_string()),
            )
            .await
            .map_err(|_| TotpError::MissingPasscode)?
            .expose_secret()
            .to_owned();
        user.passcode(passcode);
    }
    identity_builder.totp(
        token_v3::TotpBuilder::default()
            .user(user.build()?)
            .build()?,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tracing_test::traced_test;

    use super::*;
    use crate::auth::auth_helper::NonInteractive;
    use crate::config;

    #[tokio::test]
    async fn test_fill_raise_no_user_id() {
        let config = config::Auth {
            passcode: Some("pass".into()),
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
            TotpError::MissingUserId => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_no_user_passcode() {
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
            TotpError::MissingPasscode => {}
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
            passcode: Some("pass".into()),
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
                "methods": ["totp"],
                "totp": {
                    "user": {
                        "id": "uid",
                        "domain": {
                            "id": "udi",
                            "name": "udn"
                        },
                        "passcode": "pass"
                    },
                }
            })
        );
    }

    #[tokio::test]
    #[traced_test]
    async fn test_passcode_not_in_log() {
        let config = config::Auth {
            user_id: Some("uid".into()),
            username: Some("un".into()),
            user_domain_id: Some("udi".into()),
            user_domain_name: Some("udn".into()),
            passcode: Some("secret".into()),
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
        identity.build().unwrap();
        assert!(!logs_contain("secret"));
    }
}
