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

use crate::auth::auth_helper::AuthHelper;
use crate::auth::auth_token_endpoint as token_v3;
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

    /// `applicationcredential` part build error
    #[error("Cannot construct application credential data: {}", source)]
    ApplicationCredentialBuilder {
        /// The error source
        #[from]
        source: token_v3::ApplicationCredentialBuilderError,
    },

    /// `user` part build error
    #[error("Cannot construct application credential user data: {}", source)]
    UserBuilder {
        /// The request source
        #[from]
        source: token_v3::ApplicationCredentialUserBuilderError,
    },

    /// `user.domain` part build error
    #[error("Cannot construct application credential user domain data: {}", source)]
    UserDomainBuilder {
        #[from]
        source: token_v3::DomainBuilderError,
    },
}

/// Fill [`IdentityBuilder`][`token_v3::IdentityBuilder`] with application credential
pub async fn fill_identity<A>(
    identity_builder: &mut token_v3::IdentityBuilder<'_>,
    auth_data: &config::Auth,
    auth_helper: &mut A,
) -> Result<(), ApplicationCredentialError>
where
    A: AuthHelper,
{
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
            let name = auth_helper
                .get("username".into(), auth_helper.get_cloud_name())
                .await
                .map_err(|_| ApplicationCredentialError::MissingUser)?
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
        app_cred.user(user.build()?);
    } else {
        let app_cred_id = auth_helper
            .get(
                "application_credential_id".into(),
                auth_helper.get_cloud_name(),
            )
            .await
            .map_err(|_| ApplicationCredentialError::MissingIdOrName)?
            .to_owned();
        app_cred.id(app_cred_id);
    }
    identity_builder.application_credential(app_cred.build()?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tracing::info;
    use tracing_test::traced_test;

    use super::*;
    use crate::auth::auth_helper::NonInteractive;
    use crate::config;

    #[tokio::test]
    async fn test_fill_raise_no_secret() {
        let config = config::Auth {
            application_credential_id: Some("foo".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, &mut NonInteractive::default()).await;
        match res.unwrap_err() {
            ApplicationCredentialError::MissingSecret => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_neither_id_nor_name() {
        let config = config::Auth {
            application_credential_secret: Some("foo".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, &mut NonInteractive::default()).await;
        match res.unwrap_err() {
            ApplicationCredentialError::MissingIdOrName => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_raise_no_user_when_name() {
        let config = config::Auth {
            application_credential_secret: Some("foo".into()),
            application_credential_name: Some("bar".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        let res = fill_identity(&mut identity, &config, &mut NonInteractive::default()).await;
        match res.unwrap_err() {
            ApplicationCredentialError::MissingUser => {}
            other => {
                panic!("Unexpected error: {other}")
            }
        }
    }

    #[tokio::test]
    async fn test_fill_id_and_secret() {
        let config = config::Auth {
            application_credential_id: Some("foo".into()),
            application_credential_secret: Some("bar".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, &mut NonInteractive::default())
            .await
            .unwrap();
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

    #[tokio::test]
    async fn test_fill_name_and_secret_and_user() {
        let config = config::Auth {
            application_credential_name: Some("foo".into()),
            application_credential_secret: Some("bar".into()),
            user_id: Some("uid".into()),
            username: Some("un".into()),
            user_domain_id: Some("udi".into()),
            user_domain_name: Some("udn".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, &mut NonInteractive::default())
            .await
            .unwrap();
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

    #[traced_test]
    #[tokio::test]
    async fn test_secret_not_in_log() {
        let config = config::Auth {
            application_credential_name: Some("foo".into()),
            application_credential_secret: Some("secret_value".into()),
            user_id: Some("uid".into()),
            username: Some("un".into()),
            user_domain_id: Some("udi".into()),
            user_domain_name: Some("udn".into()),
            ..Default::default()
        };
        let mut identity = token_v3::IdentityBuilder::default();
        fill_identity(&mut identity, &config, &mut NonInteractive::default())
            .await
            .unwrap();
        let identity = identity.build().unwrap();
        info!("Auth is {:?}", identity);
        assert!(!logs_contain("secret_value"));
    }
}
