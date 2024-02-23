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

//! Helper methods to deal with OpenStack authentication with Application Credentials
//!
//! Authorization using Application Credentials look like:
//!
//! With ID and Secret
//! ```json
//! {
//!    "auth": {
//!        "identity": {
//!            "methods": [
//!                "application_credential"
//!            ],
//!            "application_credential": {
//!                "id": "423f19a4ac1e4f48bbb4180756e6eb6c",
//!                "secret": "rEaqvJka48mpv"
//!            }
//!        }
//!    }
//!}
//! ```
//!
//! With Name and Secret and user information
//! ```json
//! {
//!     "auth": {
//!         "identity": {
//!             "methods": [
//!                 "application_credential"
//!             ],
//!             "application_credential": {
//!                 "name": "monitoring",
//!                 "secret": "rEaqvJka48mpv",
//!                 "user": {
//!                     "id": "423f19a4ac1e4f48bbb4180756e6eb6c"
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

use thiserror::Error;

use crate::api::identity::v3::auth::token::create as token_v3;
use crate::config;

/// Application Credential related errors
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApplicationCredentialError {
    /// Missing secret
    #[error("Application credential secret is missing")]
    MissingSecret,

    /// Missing ID and Name
    #[error("Application credential id or name must be present")]
    MissingIdOrName,

    /// Missing User info
    #[error("User name/id is required when application credential name is used")]
    MissingUser,

    /// `applicationcredential` part build erro
    #[error("Cannot construct aplication credential data: {}", source)]
    ApplicationCredentialBuilder {
        /// The error source
        #[from]
        source: token_v3::ApplicationCredentialBuilderError,
    },

    /// `user` part build error
    #[error("Cannot construct aplication credential user data: {}", source)]
    UserBuilder {
        /// The request source
        #[from]
        source: token_v3::ApplicationCredentialUserBuilderError,
    },

    /// `user.domain` part build error
    #[error("Cannot construct aplication credential user domain data: {}", source)]
    UserDomainBuilder {
        #[from]
        source: token_v3::UserDomainStructBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with application credential
pub fn fill_identity(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
) -> Result<(), ApplicationCredentialError> {
    identity_builder.methods(Vec::from([token_v3::Methods::ApplicationCredential]));
    let mut app_cred = token_v3::ApplicationCredentialBuilder::default();
    app_cred.secret(
        auth_data
            .application_credential_secret
            .clone()
            .ok_or(ApplicationCredentialError::MissingSecret)?,
    );
    if let Some(val) = &auth_data.application_credential_id {
        app_cred.id(val.clone());
    } else if let Some(val) = &auth_data.application_credential_name {
        app_cred.name(val.clone());
        let mut user = token_v3::ApplicationCredentialUserBuilder::default();
        // Set user_id or name
        if let Some(val) = &auth_data.user_id {
            user.id(val.clone());
        }
        if let Some(val) = &auth_data.username {
            user.name(val.clone());
        }
        if auth_data.user_id.is_none() && auth_data.username.is_none() {
            return Err(ApplicationCredentialError::MissingUser);
        }
        // Process user domain information
        if auth_data.user_domain_id.is_some() || auth_data.user_domain_name.is_some() {
            let mut user_domain = token_v3::UserDomainStructBuilder::default();
            if let Some(val) = &auth_data.user_domain_id {
                user_domain.id(val.clone());
            }
            if let Some(val) = &auth_data.user_domain_name {
                user_domain.name(val.clone());
            }
            user.domain(user_domain.build()?);
        }
        app_cred.user(user.build()?);
    } else {
        return Err(ApplicationCredentialError::MissingIdOrName);
    }
    identity_builder.application_credential(app_cred.build()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::api::identity::v3::auth::token::create as token_v3;
    use crate::config;

    #[test]
    fn test_fill_raise_no_secret() {
        let config = config::Auth {
            application_credential_id: Some("foo".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config);
        match res.unwrap_err() {
            ApplicationCredentialError::MissingSecret => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill_raise_neither_id_nor_name() {
        let config = config::Auth {
            application_credential_secret: Some("foo".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config);
        match res.unwrap_err() {
            ApplicationCredentialError::MissingIdOrName => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill_raise_no_user_when_name() {
        let config = config::Auth {
            application_credential_secret: Some("foo".to_string()),
            application_credential_name: Some("bar".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config);
        match res.unwrap_err() {
            ApplicationCredentialError::MissingUser => {}
            other => {
                panic!("Unexpected error: {}", other)
            }
        }
    }

    #[test]
    fn test_fill_id_and_secret() {
        let config = config::Auth {
            application_credential_id: Some("foo".to_string()),
            application_credential_secret: Some("bar".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config).unwrap();
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["application_credential"],
                "application_credential": {
                    "id": "foo",
                    "secret": "bar"
                }
            })
        );
    }

    #[test]
    fn test_fill_name_and_secret_and_user() {
        let config = config::Auth {
            application_credential_name: Some("foo".to_string()),
            application_credential_secret: Some("bar".to_string()),
            user_id: Some("uid".to_string()),
            username: Some("un".to_string()),
            user_domain_id: Some("udi".to_string()),
            user_domain_name: Some("udn".to_string()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config).unwrap();
        assert_eq!(
            serde_json::to_value(identity.build().unwrap()).unwrap(),
            json!({
                "methods": ["application_credential"],
                "application_credential": {
                    "name": "foo",
                    "secret": "bar",
                    "user": {
                        "id": "uid",
                        "name": "un",
                        "domain": {
                            "id": "udi",
                            "name": "udn"
                        }
                    }
                }
            })
        );
    }
}
